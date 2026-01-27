use crate::models::ids::*;
use crate::models::relation::*;
use crate::services::field::FieldService;
use crate::services::record::RecordService;
use crate::services::{Error, DB};

const RELATION: &str = "relation";

/// Service for record relations.
/// Only workspace owners can create, delete, or list relations.
pub struct RelationService {
    record_service: RecordService,
    field_service: FieldService,
}

impl RelationService {
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        Ok(Self {
            record_service: RecordService::init(session_token).await?,
            field_service: FieldService::init(session_token).await?,
        })
    }

    /// Validate ownership and table consistency for a relation.
    async fn assert_relation_scope(
        &self,
        from_record_id: &RecordId,
        to_record_id: &RecordId,
        field_id: &FieldId,
    ) -> Result<(), Error> {
        let from_record = self.record_service.get_record_by_id(from_record_id).await?;
        let to_record = self.record_service.get_record_by_id(to_record_id).await?;
        let field = self.field_service.get_field_by_id(field_id).await?;

        // All records must belong to the same table as the field
        if from_record.table != field.table {
            return Err(Error::BadRequest);
        }

        // Optionally, ensure both records are in the same workspace
        if from_record.table != to_record.table {
            return Err(Error::BadRequest);
        }

        Ok(())
    }

    /// Create a relation.
    pub async fn create_relation(
        &self,
        from_record: RecordId,
        to_record: RecordId,
        field: FieldId,
    ) -> Result<Relation, Error> {
        self.assert_relation_scope(&from_record, &to_record, &field)
            .await?;

        let relation = InsertRelation {
            from_record,
            to_record,
            field,
        };

        DB.create(RELATION)
            .content(Relation::from_insert(relation))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a relation by ID.
    pub async fn get_relation_by_id(&self, id: &RelationId) -> Result<Relation, Error> {
        let relation: Relation = DB
            .select((RELATION, id.key()))
            .await?
            .ok_or(Error::NotFound)?;

        self.assert_relation_scope(&relation.from_record, &relation.to_record, &relation.field)
            .await?;

        Ok(relation)
    }

    /// Delete a relation.
    pub async fn delete_relation(&self, id: &RelationId) -> Result<Relation, Error> {
        let _relation = self.get_relation_by_id(id).await?;
        DB.delete((RELATION, id.key())).await?.ok_or(Error::Db)
    }

    /// List relations originating from a record.
    pub async fn list_relations_from_record(
        &self,
        record_id: &RecordId,
    ) -> Result<Vec<Relation>, Error> {
        let _ = self.record_service.get_record_by_id(record_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM relation
                WHERE from_record = $record
                ORDER BY created_at;
                "#,
            )
            .bind(("record", record_id.key()))
            .await?;

        let relations: Vec<Relation> = res.take(0)?;
        Ok(relations)
    }

    /// List relations pointing to a record.
    pub async fn list_relations_to_record(
        &self,
        record_id: &RecordId,
    ) -> Result<Vec<Relation>, Error> {
        let _ = self.record_service.get_record_by_id(record_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM relation
                WHERE to_record = $record
                ORDER BY created_at;
                "#,
            )
            .bind(("record", record_id.key()))
            .await?;

        let relations: Vec<Relation> = res.take(0)?;
        Ok(relations)
    }
}
