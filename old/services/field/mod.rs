use crate::models::field::*;
use crate::models::ids::*;
use crate::models::user::User;
use crate::services::table::TableService;
use crate::services::user::UserService;
use crate::services::{Error, DB};

const FIELD: &str = "field";

/// Service for field-related actions.
/// All actions require ownership of the parent table.
pub struct FieldService {
    actor: User,
    table_service: TableService,
}

impl FieldService {
    /// Initialize the service from a session token.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let user_service = UserService::init(session_token).await?;
        let table_service = TableService::init(session_token).await?;

        Ok(Self {
            actor: user_service.actor,
            table_service,
        })
    }

    /// Internal helper: ensure the actor owns the table.
    async fn assert_table_owner(&self, table_id: &TableId) -> Result<(), Error> {
        let _ = self.table_service.get_table_by_id(table_id).await?;
        Ok(())
    }

    /// Create a field.
    pub async fn create_field(
        &self,
        table_id: TableId,
        name: String,
        field_type: FieldType,
        order: i32,
    ) -> Result<Field, Error> {
        self.assert_table_owner(&table_id).await?;

        // Validate field type config
        field_type.validate().map_err(|_| Error::BadRequest)?;

        let field = InsertField {
            table: table_id,
            name,
            field_type,
            order,
        };

        DB.create(FIELD)
            .content(Field::from_insert(field))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a field by ID.
    pub async fn get_field_by_id(&self, id: &FieldId) -> Result<Field, Error> {
        let field: Field = DB.select((FIELD, id.key())).await?.ok_or(Error::NotFound)?;

        self.assert_table_owner(&field.table).await?;
        Ok(field)
    }

    /// Update field metadata (name, order).
    ///
    /// Does NOT allow changing the field type.
    pub async fn update_field(&self, id: &FieldId, patch: FieldPatch) -> Result<Field, Error> {
        let _ = self.get_field_by_id(id).await?;

        let mut res = DB
            .query(
                r#"
                UPDATE field
                SET $patch
                WHERE id = $id
                RETURN AFTER;
                "#,
            )
            .bind(("id", id.key()))
            .bind(("patch", patch))
            .await?;

        let updated: Option<Field> = res.take(0)?;
        updated.ok_or(Error::Db)
    }

    /// Delete a field.
    pub async fn delete_field(&self, id: &FieldId) -> Result<Field, Error> {
        let _ = self.get_field_by_id(id).await?;
        DB.delete((FIELD, id.key())).await?.ok_or(Error::Db)
    }

    /// List fields belonging to a table.
    pub async fn list_fields_by_table(&self, table_id: &TableId) -> Result<Vec<Field>, Error> {
        self.assert_table_owner(table_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM field
                WHERE table = $table
                ORDER BY order;
                "#,
            )
            .bind(("table", table_id.key()))
            .await?;

        let fields: Vec<Field> = res.take(0)?;
        Ok(fields)
    }

    /// Change a field type (schema migration).
    ///
    /// This is intentionally separate from `update_field`.
    pub async fn change_field_type(
        &self,
        id: &FieldId,
        change: FieldTypeChange,
    ) -> Result<Field, Error> {
        let mut field = self.get_field_by_id(id).await?;

        // Validate target type
        change.to.validate().map_err(|_| Error::BadRequest)?;

        let kind = change.from.migration_kind_to(&change.to);

        if !kind.is_allowed() {
            return Err(Error::BadRequest);
        }

        if kind.requires_user_input() && matches!(change.strategy, MigrationStrategy::Cast) {
            return Err(Error::BadRequest);
        }

        // ⚠️ Actual data migration will live elsewhere (Cell / RecordValue service)
        // Here we only update schema metadata

        field.field_type = change.to;
        field.updated_at = surrealdb::sql::Datetime::from(chrono::Utc::now());

        DB.update((FIELD, id.key()))
            .content(field)
            .await?
            .ok_or(Error::Db)
    }
}
