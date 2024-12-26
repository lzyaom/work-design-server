use crate::{config::Config, db};
use sqlx::PgPool;
use std::sync::Once;
use tokio::runtime::Runtime;

static INIT: Once = Once::new();

pub fn setup() -> (Runtime, PgPool) {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });

    let runtime = Runtime::new().unwrap();
    let pool = runtime.block_on(async {
        let config = Config::from_env().expect("Failed to load config");
        db::init_db(&config.database_url)
            .await
            .expect("Failed to initialize database")
    });

    (runtime, pool)
}

mod api {
    pub mod auth_tests;
    pub mod document_tests;
    pub mod log_tests;
    pub mod task_tests;
    pub mod user_tests;
}

mod services {
    pub mod document_tests;
    pub mod log_tests;
    pub mod task_tests;
    pub mod user_tests;
} 