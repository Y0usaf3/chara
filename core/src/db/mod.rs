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
use surrealdb::engine::local::{Db, Mem};

pub mod error;
pub use error::Irror;

// use surrealdb::engine::remote::ws::{Client, Ws};
// use surrealdb::opt::auth::Root;

// pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

pub async fn init() {
    DB.connect::<Mem>(()).await.unwrap();
    DB.use_ns("main").use_db("main").await.unwrap();
    DB.query(include_str!("../../SQL/main.surql"))
        .await
        .unwrap();
}
