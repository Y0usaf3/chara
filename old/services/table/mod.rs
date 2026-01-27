use crate::models::ids::*;
use crate::models::table::*;
use crate::models::user::User;
use crate::services::base::BaseService;
use crate::services::user::UserService;
use crate::services::{Error, DB};

const TABLE: &str = "table";

/// Service for table-related actions.
/// All actions require ownership of the parent base.
pub struct TableService {
    actor: User,
    base_service: BaseService,
}

impl TableService {
    /// Initialize the service from a session token.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let user_service = UserService::init(session_token).await?;
        let base_service = BaseService::init(session_token).await?;

        Ok(Self {
            actor: user_service.actor,
            base_service,
        })
    }

    /// Internal helper: ensure the actor owns the base.
    async fn assert_base_owner(&self, base_id: &BaseId) -> Result<(), Error> {
        // We don’t care about the Base value, only the permission check
        let _ = self.base_service.get_base_by_id(base_id).await?;
        Ok(())
    }

    /// Create a table.
    pub async fn create_table(
        &self,
        base_id: BaseId,
        name: String,
        order: i32,
    ) -> Result<Table, Error> {
        self.assert_base_owner(&base_id).await?;

        let table = InsertTable {
            base: base_id,
            name,
            order,
        };

        DB.create(TABLE)
            .content(Table::from_insert(table))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a table by ID.
    pub async fn get_table_by_id(&self, id: &TableId) -> Result<Table, Error> {
        let table: Table = DB.select((TABLE, id.key())).await?.ok_or(Error::NotFound)?;

        self.assert_base_owner(&table.base).await?;
        Ok(table)
    }

    /// Update a table.
    pub async fn update_table(&self, id: &TableId, patch: TablePatch) -> Result<Table, Error> {
        let _ = self.get_table_by_id(id).await?;

        let mut res = DB
            .query(
                r#"
                UPDATE table
                SET $patch
                WHERE id = $id
                RETURN AFTER;
                "#,
            )
            .bind(("id", id.key()))
            .bind(("patch", patch))
            .await?;

        let updated: Option<Table> = res.take(0)?;
        updated.ok_or(Error::Db)
    }

    /// Delete a table.
    pub async fn delete_table(&self, id: &TableId) -> Result<Table, Error> {
        let _ = self.get_table_by_id(id).await?;
        DB.delete((TABLE, id.key())).await?.ok_or(Error::Db)
    }

    /// List tables belonging to a base.
    pub async fn list_tables_by_base(&self, base_id: &BaseId) -> Result<Vec<Table>, Error> {
        self.assert_base_owner(base_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM table
                WHERE base = $base
                ORDER BY order;
                "#,
            )
            .bind(("base", base_id.key()))
            .await?;

        let tables: Vec<Table> = res.take(0)?;
        Ok(tables)
    }
}
