#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    //pub database: DatabaseSettings,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config/configuration.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}