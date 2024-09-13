use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub browser: String,
    pub secrets_file: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("nestor")?;
        let config_path = xdg_dirs.get_config_file("config");
        let result =
            config::Config::builder().add_source(config::File::with_name(config_path.to_str().unwrap())).build()?.try_deserialize()?;

        Ok(result)
    }
}
