// Source - https://stackoverflow.com/a/25877389
// Posted by Arjan, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-26, License - CC BY-SA 4.0

#![allow(unexpected_cfgs)]

#[cfg(test)]
mod test;

mod core;
use crate::core::db::DB;
use crate::core::service::user::{SessionI, UserService};
use hackclub_auth_api::HCAuth;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        dotenv!("CLIENT_ID"),
        dotenv!("CLIENT_SECRET"),
        dotenv!("REDIRECT_URI"),
    )
});

pub static MASTER_KEY: &str = dotenv!("MASTER_KEY");

#[macro_use]
extern crate bitmask;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("100.118.244.5:3001").await?;

    DB.signin(Root {
        username: "yousafe".to_string(),
        password: "MRAOWRR".to_string(),
    })
    .await?;

    DB.use_ns("main").use_db("main").await?;

    let smt = UserService::login(core::service::user::AuthMethod::Session(SessionI {
        ip: "192.168.11.100".to_string(),
        agent: "owo".to_string(),
        token: "IIOOII".to_string(),
    }))
    .await;

    match smt {
        Ok(v) => {
            let user = v.is_admin().await;
            println!("{user:#?}");
            println!("{v:#?}")
        }
        Err(e) => eprintln!("{e:?}"),
    }

    Ok(())
}
