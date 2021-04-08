use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct CfgDB {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CfgWeb {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct CfgRedis {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db: CfgDB,
    pub web: CfgWeb,
    pub redis: CfgRedis,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}