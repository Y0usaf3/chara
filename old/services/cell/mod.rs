use crate::models::cell::*;
use crate::models::field::Field;
use crate::models::ids::*;
use crate::models::record::Record;
use crate::services::field::FieldService;
use crate::services::record::RecordService;
use crate::services::{Error, DB};

const CELL: &str = "cell";

/// Service for cell-related actions.
/// All actions require ownership of the parent table (via record + field).
pub struct CellService {
    record_service: RecordService,
    field_service: FieldService,
}

impl CellService {
    /// Initialize the service from a session token.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        Ok(Self {
            record_service: RecordService::init(session_token).await?,
            field_service: FieldService::init(session_token).await?,
        })
    }

    /// Internal helper:
    /// - record exists and is owned
    /// - field exists and is owned
    /// - record.table == field.table
    async fn assert_cell_scope(
        &self,
        record_id: &RecordId,
        field_id: &FieldId,
    ) -> Result<(Record, Field), Error> {
        let record = self.record_service.get_record_by_id(record_id).await?;
        let field = self.field_service.get_field_by_id(field_id).await?;

        if record.table != field.table {
            return Err(Error::BadRequest);
        }

        Ok((record, field))
    }

    /// Create a cell.
    pub async fn create_cell(
        &self,
        record_id: RecordId,
        field_id: FieldId,
        value: serde_json::Value,
    ) -> Result<Cell, Error> {
        let _ = self.assert_cell_scope(&record_id, &field_id).await?;

        let cell = InsertCell {
            record: record_id,
            field: field_id,
            value,
        };

        DB.create(CELL)
            .content(Cell::from_insert(cell))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a cell by ID.
    pub async fn get_cell_by_id(&self, id: &CellId) -> Result<Cell, Error> {
        let cell: Cell = DB.select((CELL, id.key())).await?.ok_or(Error::NotFound)?;

        // Ownership is proven transitively
        let _ = self.assert_cell_scope(&cell.record, &cell.field).await?;
        Ok(cell)
    }

    /// Update a cell value.
    pub async fn update_cell(&self, id: &CellId, patch: CellPatch) -> Result<Cell, Error> {
        let mut cell = self.get_cell_by_id(id).await?;
        cell.apply_patch(patch);

        DB.update((CELL, id.key()))
            .content(cell)
            .await?
            .ok_or(Error::Db)
    }

    /// Delete a cell.
    pub async fn delete_cell(&self, id: &CellId) -> Result<Cell, Error> {
        let _ = self.get_cell_by_id(id).await?;
        DB.delete((CELL, id.key())).await?.ok_or(Error::Db)
    }

    /// List cells belonging to a record.
    pub async fn list_cells_by_record(&self, record_id: &RecordId) -> Result<Vec<Cell>, Error> {
        // Ownership check happens here
        let _ = self.record_service.get_record_by_id(record_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM cell
                WHERE record = $record
                ORDER BY created_at;
                "#,
            )
            .bind(("record", record_id.key()))
            .await?;

        let cells: Vec<Cell> = res.take(0)?;
        Ok(cells)
    }

    /// List cells belonging to a field.
    pub async fn list_cells_by_field(&self, field_id: &FieldId) -> Result<Vec<Cell>, Error> {
        // Ownership check happens here
        let _ = self.field_service.get_field_by_id(field_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM cell
                WHERE field = $field
                ORDER BY created_at;
                "#,
            )
            .bind(("field", field_id.key()))
            .await?;

        let cells: Vec<Cell> = res.take(0)?;
        Ok(cells)
    }
}
