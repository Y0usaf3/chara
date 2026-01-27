mod base;
mod cell;
mod error;
mod field;
mod identity;
mod record;
mod relation;
mod session;
mod table;
mod user;
mod workspace;
mod workspace_users;

use crate::services::error::Error;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

async fn init_db() -> Result<(), Error> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",   // to change
        password: "secret", // to change too :p
    })
    .await?;

    DB.use_ns("main").use_db("main").await?;

    DB.query(include_str!("../../src/models/sql/user.surql"))
        .query(include_str!("../../src/models/sql/identity.surql"))
        .query(include_str!("../../src/models/sql/workspace.surql"))
        .query(include_str!("../../src/models/sql/base.surql"))
        .query(include_str!("../../src/models/sql/table.surql"))
        .query(include_str!("../../src/models/sql/field.surql"))
        .query(include_str!("../../src/models/sql/cell.surql"))
        .query(include_str!("../../src/models/sql/record.surql"))
        .query(include_str!("../../src/models/sql/relation.surql"))
        .query(include_str!("../../src/models/sql/workspace_users.surql"))
        .await?;
    Ok(())
}
