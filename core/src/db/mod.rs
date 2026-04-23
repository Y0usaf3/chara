#[macro_export]
macro_rules! env_required {
    ($key:expr) => {{
        use std::env;
        let _ = $crate::dotenvy::dotenv();

        match env::var($key) {
            Ok(val) if !val.trim().is_empty() => val,
            _ => panic!(concat!("Missing required env variable: ", $key)),
        }
    }};
}

use std::sync::LazyLock;
use surrealdb::Surreal;
//use surrealdb::engine::local::{Db, Mem};

pub mod error;
pub use error::Irror;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
//pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

// TODO: use env vars to choose which surli file to use

pub async fn init() {
    //let _ = DB.connect::<Mem>(()).await;
    DB.connect::<Ws>(env_required!("DB_URL")).await.unwrap();
    DB.signin(Root {
        username: env_required!("DB_USERNAME"),
        password: env_required!("DB_PASSWORD"),
    })
    .await
    .unwrap();

    DB.use_ns("main").use_db("main").await.unwrap();
    let res = DB
        .query("DEFINE BUCKET OVERWRITE bucki BACKEND 'file:/home/dietpi/';")
        .await
        .unwrap()
        .check();
    res.unwrap();
    DB.query("DEFINE MODULE OVERWRITE mod::bit AS f'bucki:/chaira-bitwise_ops-0.0.1.surli';")
        .await
        .unwrap();
    DB.query(include_str!("../../SQL/main.surql"))
        .await
        .unwrap();
}
