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
        format!("redis://{}:{}/{}", self.host, self.port, self.db)
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let environment = std::env::var("ENV").unwrap_or_else(|_| "production".into());
    let base_path = match environment.as_str() {
        "production" => std::path::PathBuf::from("/etc/blitz-login"),
        "development" => std::env::current_dir().expect("Failed to get current directory").join("config"),
        _ => panic!("Invalid environment: {}", environment),
    };
    let config_file = match environment.as_str() {
        "production" => base_path.join("configuration.yaml"),
        "development" => base_path.join("development.yaml"),
        _ => panic!("Invalid environment: {}", environment),
    };
    let settings = config::Config::builder()
        .add_source(config::File::new(config_file.to_str().unwrap(), config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}