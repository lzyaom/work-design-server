use config::{Config as ConfigReader, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    #[serde(default = "Uuid::new_v4")]
    pub system_user_id: Uuid,
    #[serde(default = "default_system_email")]
    pub system_user_email: String,
}

fn default_system_email() -> String {
    "system@example.com".to_string()
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        ConfigReader::builder()
            .add_source(Environment::default()) // 从环境变量中加载配置
            .add_source(File::with_name(".env").required(false)) // 从 .env 文件中加载配置
            // .add_source(Environment::with_prefix("APP"))
            .build()?
            .try_deserialize()
    }
}

pub fn load_config() -> Result<Config, ConfigError> {
    Config::new()
}
