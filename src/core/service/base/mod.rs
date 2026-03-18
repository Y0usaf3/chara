use crate::core::{
    db::{error::Error, DB},
    models::{
        base::Base,
        ids::{BaseId, TableId, UserId},
        permissions::BasePermissions,
        table::Table,
    },
};

#[derive(Debug, Clone)]
pub struct BaseService {
    pub base: Base,
    pub user: UserId,
    base_record_id: BaseId,
}

impl BaseService {
    pub fn id(&self) -> &BaseId {
        &self.base_record_id
    }

    pub async fn new(base_id: BaseId, user: UserId) -> Result<Self, Error> {
        let mut res = DB
            .query(
                "
BEGIN TRANSACTION;

LET $base_record = (SELECT * FROM $base WHERE is_deleted = false)[0];

LET $accessible_base = (
    SELECT * FROM $base WHERE 
        is_deleted = false AND (
            owner = $user OR 
            fn::can(
                (SELECT VALUE perms FROM can_access_base WHERE in = $user AND out = $this.id)[0], 
                2
            )
        )
)[0];

IF $accessible_base == NONE {
    THROW 'Permission Denied: User ' + <string>$user + ' cannot access ' + <string>$base;
};

RETURN $accessible_base;

COMMIT TRANSACTION;
            ",
            )
            .bind(("base", base_id.clone()))
            .bind(("user", user.clone()))
            .await?;
        let base: Base = res.take::<Option<Base>>(7)?.ok_or(Error::Database(
            super::errors::DatabaseError::QueryFailed(format!("{}:{}", file!(), line!())),
        ))?;
        Ok(Self {
            base,
            base_record_id: base_id,
            user,
        })
    }

    pub async fn invite_user(&self, user: UserId, perms: BasePermissions) -> Result<(), Error> {
        let res = DB.query("
            BEGIN TRANSACTION;

-- View (1 << 1) = 2 + ManageInvitations (1 << 8) = 256

LET $is_owner = (SELECT VALUE owner FROM $target_base)[0] == $inviter_id;
LET $inviter_perms = (SELECT VALUE perms FROM can_access_base WHERE in = $inviter_id AND out = $target_base)[0] OR 0;

IF !$is_owner AND !fn::can($inviter_perms, 258) {
    THROW 'Unauthorized: You need [View] and [ManageInvitations] to invite others.';
};

RELATE $invited_id->can_access_base->$target_base 
    SET perms = $requested_perms;

COMMIT TRANSACTION;
            ")
        .bind(("inviter_id",self.user.clone()))
        .bind(("invited_id",user))
        .bind(("target_base",self.base_record_id.clone()))
        .bind(("perms",perms)).await?;
        res.check()?;
        Ok(())
    }

    pub async fn delete(&self) -> Result<Base, Error> {
        let mut res = DB.query("
        BEGIN TRANSACTION;

        -- 'Delete' (1 << 3 = 8) 
        
        LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
        LET $is_admin = (SELECT VALUE role FROM $user WHERE id = $user)[0] == 'admin';
        LET $user_perms = (SELECT VALUE perms FROM can_access_base WHERE in = $user AND out = $base)[0] OR 0;
        
        IF !$is_owner AND !$is_admin AND !fn::can($user_perms, 8) {
            THROW 'Unauthorized: You do not have permission to delete this base.';
        };

        UPDATE $base SET 
            is_deleted = true, 
            updated_at = time::now();

        COMMIT TRANSACTION;
    ")
    .bind(("base", self.base_record_id.clone()))
    .bind(("user", self.user.clone()))
    .await?;
        let base: Option<Base> = res.take(8)?;
        let base = base.ok_or(Error::Database(
            super::errors::DatabaseError::TransactionFailed(format!("{}:{}", file!(), line!())),
        ))?;

        Ok(base)
    }

    pub async fn create_table(&self, name: String) -> Result<Table, Error> {
        let mut res = DB.query("
            BEGIN TRANSACTION;

            LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
            LET $user_perms = (SELECT VALUE perms FROM can_access_base WHERE in = $user AND out = $base)[0] OR 0;

            IF !$is_owner AND !fn::can($user_perms, 16) {
                THROW 'Unauthorized: You do not have ManageTables permission.';
            };

            -- Create the table linked to this base
            LET $table = (CREATE table SET 
                name = $name, 
                base = $base, 
                is_deleted = false
            );

            -- Automatically grant creator 'Full Access' (1|2|4 = 7) to this specific table
            RELATE $user->can_access_table->$table SET perms = 7;

            RETURN $table;

            COMMIT TRANSACTION;
        ")
        .bind(("user", self.user.clone()))
        .bind(("base", self.base_record_id.clone()))
        .bind(("name", name))
        .await?;

        let table: Table = res.take::<Option<Table>>(3)?.ok_or(Error::Database(
            super::errors::DatabaseError::QueryFailed("Failed to create table".into()),
        ))?;
        Ok(table)
    }

    pub async fn delete_table(&self, table_id: TableId) -> Result<(), Error> {
        let res = DB.query("
            BEGIN TRANSACTION;

            LET $is_owner = (SELECT VALUE owner FROM $base)[0] == $user;
            LET $base_perms = (SELECT VALUE perms FROM can_access_base WHERE in = $user AND out = $base)[0] OR 0;
            
            LET $table_perms = (SELECT VALUE perms FROM can_access_table WHERE in = $user AND out = $table_id)[0] OR 0;

            IF !$is_owner AND !fn::can($base_perms, 16) AND !fn::can($table_perms, 4) {
                THROW 'Unauthorized: Cannot delete this table.';
            };

            UPDATE $table_id SET is_deleted = true, updated_at = time::now();
            
            UPDATE record SET is_deleted = true WHERE table = $table_id;

            COMMIT TRANSACTION;
        ")
        .bind(("user", self.user.clone()))
        .bind(("base", self.base_record_id.clone()))
        .bind(("table_id", table_id))
        .await?;

        res.check()?;
        Ok(())
    }

    pub async fn open_table(&self, table_id: TableId) -> Result<TableService, Error> {
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
        .bind(("user", self.user.clone()))
        .bind(("base", self.base_record_id.clone()))
        .bind(("table_id", table_id.clone()))
        .await?;

        let table: Table = res.take::<Option<Table>>(1)?.ok_or(Error::Database(
            super::errors::DatabaseError::QueryFailed("Table not found or Access Denied".into()),
        ))?;

        Ok(TableService::new(
            table,
            self.base_record_id.clone(),
            self.user.clone(),
        ))
    }
}
