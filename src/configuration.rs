use std::net::IpAddr;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use smart_default::SmartDefault;
use std::str::FromStr;

#[derive(Clone, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Configuration {
    pub connection: AppConnection,
    pub database: DatabaseConfiguration,
}

#[derive(Clone, Deserialize, SmartDefault)]
#[serde(default)]
pub struct AppConnection {
    #[default(IpAddr::from_str("127.0.0.1").unwrap())]
    pub ip: IpAddr,

    #[default = 8000]
    pub port: u16,
}

impl ToString for AppConnection {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Clone, Deserialize, SmartDefault)]
pub struct DatabaseConfiguration {
    #[default = "postgres"]
    pub username: String,

    #[default = "password"]
    pub password: String,

    #[default = 5432]
    pub port: u16,

    #[default(IpAddr::from_str("127.0.0.1").unwrap())]
    pub host: IpAddr,

    #[default = "dev"]
    pub name: String,
}

impl DatabaseConfiguration {
    pub fn instance_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }

    pub fn database_url(&self) -> String {
        format!("{}/{}", self.instance_url(), self.name)
    }
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .add_source(Environment::with_prefix("Z2D"))
        .build()?;

    settings.try_deserialize()
}
