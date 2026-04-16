use crate::db::*;
use crate::models::*;
use crate::service::errors::TableError;

// TODO: gotta work here :sob:
// fr :noooooovanish:

#[derive(Debug, Clone)]
pub struct TableService {
    pub table: Table,
    pub user: UserId,
    pub base: BaseId,
    table_record_id: TableId,
}

impl TableService {
    pub async fn new(tablee: TableId, base: BaseId, user: UserId) -> Result<Self, Irror> {
        let mut res = DB.query("
            LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
            
            SELECT * FROM $table_id WHERE is_deleted = false AND (
                $is_owner OR 
                fn::can(
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
}
