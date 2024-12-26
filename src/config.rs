use config::{Config as ConfigBuilder, ConfigError};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        let cfg = ConfigBuilder::builder()
            .add_source(config::Environment::default()) // 从环境变量加载配置
            .add_source(config::File::with_name(".env").required(false)) // 从 .env 文件加载配置
            .build()?;
        println!("{:?}", cfg);

        cfg.try_deserialize()
    }
}

pub fn load_config() -> Result<Config, config::ConfigError> {
    Config::from_env()
}
