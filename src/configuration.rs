#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub redis: RedisSettings,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub db: String
}

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        "redis://redis".to_string()
        // format!("redis://{}:{}/{}", self.host, self.port, self.db)
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config/configuration.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}