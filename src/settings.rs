use config::{File,Config,ConfigError};

#[derive(Deserialize)]
pub struct Telegram {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub path: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub telegram: Telegram,
    pub database: Database,
}

impl Settings {
    pub fn read(path: &str) -> Result<Settings, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(path))?;
        s.try_into()
    }
}
