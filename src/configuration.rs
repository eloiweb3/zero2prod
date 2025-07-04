use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?
        .try_deserialize()
}


impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.username ,self.password , self.host, self.port,self.database_name)
    }
}
