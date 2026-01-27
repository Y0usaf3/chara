use crate::models::ids::*;
use crate::models::record::*;
use crate::models::user::User;
use crate::services::table::TableService;
use crate::services::user::UserService;
use crate::services::{Error, DB};

const RECORD: &str = "record";

/// Service for record-related actions.
/// All actions require ownership of the parent table.
pub struct RecordService {
    actor: User,
    table_service: TableService,
}

impl RecordService {
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

    /// Create a record.
    pub async fn create_record(&self, table_id: TableId) -> Result<Record, Error> {
        self.assert_table_owner(&table_id).await?;

        let record = InsertRecord { table: table_id };

        DB.create(RECORD)
            .content(Record::from_insert(record))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a record by ID.
    pub async fn get_record_by_id(&self, id: &RecordId) -> Result<Record, Error> {
        let record: Record = DB
            .select((RECORD, id.key()))
            .await?
            .ok_or(Error::NotFound)?;

        self.assert_table_owner(&record.table).await?;
        Ok(record)
    }

    /// Update a record.
    ///
    /// (For now this only touches metadata like `updated_at`)
    pub async fn update_record(&self, id: &RecordId, patch: RecordPatch) -> Result<Record, Error> {
        let mut record = self.get_record_by_id(id).await?;

        // Explicit touch so timestamps stay consistent
        record.touch();

        DB.update((RECORD, id.key()))
            .content(patch)
            .await?
            .ok_or(Error::Db)
    }

    /// Delete a record.
    pub async fn delete_record(&self, id: &RecordId) -> Result<Record, Error> {
        let _ = self.get_record_by_id(id).await?;
        DB.delete((RECORD, id.key())).await?.ok_or(Error::Db)
    }

    /// List records belonging to a table.
    pub async fn list_records_by_table(&self, table_id: &TableId) -> Result<Vec<Record>, Error> {
        self.assert_table_owner(table_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM record
                WHERE table = $table
                ORDER BY created_at;
                "#,
            )
            .bind(("table", table_id.key()))
            .await?;

        let records: Vec<Record> = res.take(0)?;
        Ok(records)
    }
}
