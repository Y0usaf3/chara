use crate::db::*;
use crate::models::*;
use crate::service::errors::TableError;
use crate::service::table::migration::MigrationStrategy;
use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;
use surrealdb::types::SurrealValue;
use surrealdb::types::ToSql;

// TODO: gotta work here :sob:
// fr :noooooovanish:

#[derive(Debug, Clone)]
pub struct TableService {
    pub table: Table,
    pub user: UserId,
    pub base: BaseId,
    table_record_id: TableId,
}

// NOTE: FR stands for frontend :p
#[derive(Serialize, Deserialize, SurrealValue)]
pub struct FieldConfigFR {
    pub is_deleted: bool,
    pub config: FieldConfig,
    pub is_primary: bool,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub name: String,
    pub description: Option<String>,
}

pub struct PaginationParams {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct MigrationReport {
    pub can_migrate: bool,
    pub success_rate: f32,
    pub affected_records: usize,
    pub failed_records: usize,
    pub warning: Option<String>,
}

impl TableService {
    pub async fn new(tablee: TableId, base: BaseId, user: UserId) -> Result<Self, Irror> {
        let mut res = DB.query("
            LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
            
            SELECT * FROM $table_id WHERE is_deleted = false AND (
                $is_owner OR 
                mod::bit::can(
                    (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $this.id)[0], 
                    2
                )
            );
        ")
        .bind(("user", user.clone()))
        .bind(("base", base.clone()))
        .bind(("table_id", tablee.clone()))
        .await?;

        let table: Table = res.take::<Option<Table>>(1)?.ok_or(TableError::NotFound)?;

        Ok(Self {
            table,
            user,
            base,
            table_record_id: tablee,
        })
    }

    pub async fn get_field_config(&self, field_id: FieldId) -> Result<FieldConfigFR, Irror> {
        let mut res = DB
        .query("
            SELECT * FROM $field 
            WHERE 
                table = $table_id AND 
                table.base = $base_id AND 
                is_deleted = false AND
                (
                    (SELECT VALUE owner FROM $base_id)[0] == $user OR
                    mod::bit::can(
                        (SELECT VALUE perms FROM can_access_field WHERE in = $user AND out = $this.id)[0], 
                        2
                    )
                )
        ")
        .bind(("field", field_id))
        .bind(("table_id", self.table_record_id.clone()))
        .bind(("base_id", self.base.clone()))
        .bind(("user", self.user.clone()))
        .await?;
        dbg!(&res);

        let field_config: Option<FieldConfigFR> = res.take(0)?;

        match field_config {
            Some(config) => Ok(config),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }
    pub async fn create_field(&self, field: InsertField) -> Result<Field, Irror> {
        let field = Field::from_insert(field);
        let mut res = DB
            .query(
                "
            LET $is_owner = (SELECT VALUE owner FROM $base_id)[0] == $user;
            LET $has_table_edit = mod::bit::can(
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
                4
            );

            IF $is_owner OR $has_table_edit THEN
                (CREATE field SET 
                    name = $data.name,
                    table = $table_id,
                    is_primary = $data.is_primary,
                    is_nullable = $data.is_nullable,
                    is_unique = $data.is_unique,
                    order = $data.order,
                    description = $data.description,
                    config = $data.config,
                    created_at = time::now(),
                    updated_at = time::now()
                )
            END;
        ",
            )
            .bind(("user", self.user.clone()))
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("data", field))
            .await?;
        dbg!(&res);

        let created_field: Option<Field> = res.take(2)?;

        match created_field {
            Some(f) => Ok(f),
            None => Err(Irror::Table(TableError::CreateFailed)),
        }
    }
    pub async fn update_field(
        &self,
        field_id: FieldId,
        field: InsertField,
    ) -> Result<Result<Field, MigrationStrategy>, Irror> {
        let field = Field::from_insert(field);
        let mut res = DB
            .query("SELECT * FROM $field WHERE table = $table_id")
            .bind(("field", field_id.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .await?;

        let current_field: Field = res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;

        let mut perm_res = DB
            .query(
                "
        SELECT VALUE 
            (SELECT VALUE owner FROM $base_id)[0] == $user OR
            mod::bit::can(
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
                4
            )
            ",
            )
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;

        let is_authorized: bool = perm_res.take::<Option<bool>>(0)?.unwrap_or(false);
        if !is_authorized {
            return Err(Irror::Table(TableError::Unauthorized));
        }

        let strategy = current_field.config.get_migration_strategy(&field.config);

        if strategy == MigrationStrategy::Risky || strategy == MigrationStrategy::Destructive {
            return Ok(Err(strategy));
        }

        let mut update_res = DB
            .query("UPDATE $field CONTENT $field_config")
            .bind(("field", field_id))
            .bind(("field_config", field))
            .await?;

        let updated: Field = update_res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;

        Ok(Ok(updated))
    }

    pub async fn delete_field(&self, field: FieldId) -> Result<Field, Irror> {
        let mut res = DB
            .query(
                "
        LET $is_owner = (SELECT VALUE owner FROM $base_id)[0] == $user;
        LET $has_table_edit = mod::bit::can(
            (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
            4
        );

        IF $is_owner OR $has_table_edit THEN
            (
                
                UPDATE record SET cells = object::remove(cells, $field)
                WHERE table = $table_id AND is_deleted = false;
                
                UPDATE $field SET 
                    is_deleted = true,
                    updated_at = time::now()
            )
        END;
    ",
            )
            .bind(("user", self.user.clone()))
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("field", field))
            .await?;
        dbg!(&res);

        let deleted_field: Option<Field> = res.take(4)?;

        match deleted_field {
            Some(f) => Ok(f),
            None => Err(Irror::Table(TableError::DeleteFailed)),
        }
    }

    pub async fn get_record(&self, record_id: RecordId) -> Result<Record, Irror> {
        let mut res = DB
            .query(
                "
        SELECT * FROM $record_id 
        WHERE 
            table = $table_id AND 
            is_deleted = false AND
            mod::bit::can(
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0],
                2
            )
    ",
            )
            .bind(("record_id", record_id))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;
        dbg!(&res);

        let record: Option<Record> = res.take(0)?;

        match record {
            Some(r) => Ok(r),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }

    pub async fn list_records(
        &self,
        pagination_params: PaginationParams,
    ) -> Result<Vec<Record>, Irror> {
        let limit = pagination_params.limit.unwrap_or(50);
        let skip = pagination_params.offset.unwrap_or(0);

        let mut res = DB
            .query(
                "
        LET $perms = (
            SELECT VALUE perms 
            FROM can_access_table 
            WHERE in = $user AND out = $table_id
        )[0] ?? 0;

        SELECT * FROM record 
        WHERE 
            table = $table_id AND 
            is_deleted = false AND
            mod::bit::can($perms, 2)
        LIMIT $limit
        START $skip;",
            )
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .bind(("limit", limit))
            .bind(("skip", skip))
            .await?;

        let records: Vec<Record> = res.take(0)?;

        Ok(records)
    }

    pub async fn create_record(&self, record: InsertRecord) -> Result<Record, Irror> {
        let record = Record::from_insert(record);

        let mut res = DB
            .query(
                "
        LET $has_table_edit = mod::bit::can(
            (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
            4
        );

        IF $has_table_edit THEN
            (CREATE record SET 
                table = $table_id,
                cells = $data.cells,
                is_deleted = false,
                created_at = time::now(),
                updated_at = time::now()
            )
        END;
    ",
            )
            .bind(("user", self.user.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("data", record))
            .await?;
        dbg!(&res);

        let created_record: Option<Record> = res.take(1)?;

        match created_record {
            Some(r) => Ok(r),
            None => Err(Irror::Table(TableError::CreateFailed)),
        }
    }

    pub async fn update_record(
        &self,
        record_id: RecordId,
        patch: RecordPatch,
    ) -> Result<Record, Irror> {
        let mut perm_res = DB
            .query(
                "
                (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0]; 
        ",
            )
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;

        // TODO: make a function for rust to check permissions better than doing this

        let perms: TablePermissions =
            perm_res
                .take::<Option<TablePermissions>>(0)?
                .ok_or(Irror::Db(
                    "something wrong happened with the permissions".to_string(),
                ))?;
        if perms.contains(TablePermission::Edit) || perms.contains(TablePermission::Admin) {
            return Err(Irror::Table(TableError::Unauthorized));
        }

        // Merge changed cells into existing record
        let merge_query = if let Some(ref changed_cells) = patch.changed_cells {
            let mut cells_update = String::new();
            for (i, (key, _)) in changed_cells.iter().enumerate() {
                if i > 0 {
                    cells_update.push(',');
                }
                cells_update.push_str(&format!("cells.{} = $cells[{}]", key, i));
            }

            format!(
                "UPDATE $record_id SET {} , updated_at = time::now();",
                cells_update
            )
        } else {
            "UPDATE $record_id SET updated_at = time::now();".to_string()
        };

        let mut update_res = DB
            .query(&merge_query)
            .bind(("record_id", record_id))
            .bind(("cells", patch.changed_cells.unwrap_or_default()))
            .await?;

        let updated: Option<Record> = update_res.take(0)?;

        match updated {
            Some(r) => Ok(r),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }

    pub async fn delete_record(&self, record_id: RecordId) -> Result<Record, Irror> {
        let mut perm_res = DB
            .query(
                "
        SELECT VALUE mod::bit::can(
            (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 
            4
        )
        ",
            )
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;

        let is_authorized: bool = perm_res.take::<Option<bool>>(0)?.unwrap_or(false);
        if !is_authorized {
            return Err(Irror::Table(TableError::Unauthorized));
        }

        let mut res = DB
            .query(
                "
        UPDATE $record_id SET 
            is_deleted = true,
            updated_at = time::now()
        ",
            )
            .bind(("record_id", record_id))
            .await?;
        dbg!(&res);

        let deleted_record: Option<Record> = res.take(0)?;

        match deleted_record {
            Some(r) => Ok(r),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }

    pub async fn check_migration(
        &self,
        field_id: FieldId,
        target_config: FieldConfig,
    ) -> Result<MigrationReport, Irror> {
        let mut field_res = DB
            .query("SELECT name FROM $field_id WHERE table = $table_id")
            .bind(("field_id", field_id))
            .bind(("table_id", self.table_record_id.clone()))
            .await?;
        let field_name: String = field_res
            .take::<Option<String>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;

        let mut record_res = DB
            .query("SELECT * FROM record WHERE table = $table_id AND is_deleted = false")
            .bind(("table_id", self.table_record_id.clone()))
            .await?;
        let records: Vec<Record> = record_res.take(0)?;

        let total = records.len();
        let mut successful = 0;

        for record in &records {
            if let Some(cell) = record.cells.get(&field_name) {
                if cell.value.convert_to(&target_config).is_ok() {
                    successful += 1;
                }
            } else {
                // If cell is missing and target allows it, consider it "safe"
                successful += 1;
            }
        }

        let failed = total - successful;
        let success_rate = if total == 0 {
            1.0
        } else {
            successful as f32 / total as f32
        };

        Ok(MigrationReport {
            can_migrate: success_rate > 0.5,
            success_rate,
            affected_records: total,
            failed_records: failed,
            warning: if success_rate < 1.0 {
                Some(format!(
                    "{} out of {} records will fail conversion",
                    failed, total
                ))
            } else {
                None
            },
        })
    }

    pub async fn migrate_field_type(
        &self,
        field_id: FieldId,
        new_config: FieldConfig,
    ) -> Result<Result<Field, String>, Irror> {
        // 1. Permissions
        let mut perm_res = DB
            .query("SELECT VALUE (SELECT VALUE owner FROM $base_id)[0] == $user OR mod::bit::can((SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0], 4)")
            .bind(("base_id", self.base.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("user", self.user.clone()))
            .await?;
        if !perm_res.take::<Option<bool>>(0)?.unwrap_or(false) {
            return Err(Irror::Table(TableError::Unauthorized));
        }
        // 2. Strategy Check
        let mut field_res = DB
            .query("SELECT * FROM $field_id WHERE table = $table_id")
            .bind(("field_id", field_id.clone()))
            .bind(("table_id", self.table_record_id.clone()))
            .await?;
        let current_field: Field = field_res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::NotFound))?;
        let strategy = current_field.config.get_migration_strategy(&new_config);

        if strategy == MigrationStrategy::Risky || strategy == MigrationStrategy::Destructive {
            return Ok(Err(format!(
                "Migration is {:?}. Data loss might occur.",
                strategy
            )));
        }
        if strategy == MigrationStrategy::NoOp {
            return Ok(Err("No operation needed".to_string()));
        }

        let mut record_res = DB
            .query("SELECT * FROM record WHERE table = $table_id AND is_deleted = false")
            .bind(("table_id", self.table_record_id.clone()))
            .await?;
        let records: Vec<Record> = record_res.take(0)?;

        for record in records {
            if let Some(cell) = record.cells.get(&current_field.name)
                && let Ok(new_value) = cell.value.convert_to(&new_config)
            {
                let mut new_cell = cell.clone();
                new_cell.value = new_value;
                new_cell.updated_at = Datetime::from(chrono::Utc::now());

                DB.query("UPDATE $record_id SET cells[$field_name] = $new_cell")
                    .bind(("record_id", record.id.clone()))
                    .bind(("field_name", current_field.name.clone()))
                    .bind(("new_cell", new_cell))
                    .await?;
            }
        }

        // 4. Update Field Config
        let mut update_res = DB
            .query("UPDATE $field_id SET config = $new_config, updated_at = time::now()")
            .bind(("field_id", field_id))
            .bind(("new_config", new_config))
            .await?;

        let updated: Field = update_res
            .take::<Option<Field>>(0)?
            .ok_or(Irror::Table(TableError::UpdateFailed))?;
        Ok(Ok(updated))
    }

    pub async fn force_edit_config(
        &self,
        field_id: FieldId,
        new_config: FieldConfig,
    ) -> Result<Field, Irror> {
        let mut perm_res = DB
            .query(
                "
        SELECT VALUE (SELECT VALUE owner FROM $base_id)[0] == $user
        ",
            )
            .bind(("base_id", self.base.clone()))
            .bind(("user", self.user.clone()))
            .await?;

        let is_owner: bool = perm_res.take::<Option<bool>>(0)?.unwrap_or(false);
        if !is_owner {
            return Err(Irror::Table(TableError::Unauthorized));
        }

        let mut res = DB
            .query(
                "
        UPDATE $field_id SET 
            config = $new_config,
            updated_at = time::now()
        WHERE table = $table_id AND is_deleted = false
    ",
            )
            .bind(("field_id", field_id))
            .bind(("table_id", self.table_record_id.clone()))
            .bind(("new_config", new_config))
            .await?;
        dbg!(&res);

        let updated: Option<Field> = res.take(0)?;

        match updated {
            Some(f) => Ok(f),
            None => Err(Irror::Table(TableError::NotFound)),
        }
    }
}
