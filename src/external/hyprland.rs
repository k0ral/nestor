use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::BufReader,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct Client {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub width: u16,
    pub height: u16,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
    pub focused: bool,
    pub disabled: bool,
    #[serde(rename = "availableModes")]
    pub available_modes: Vec<String>,
}

impl Client {
    pub fn list_monitors(&self) -> Result<Vec<Monitor>> {
        let mut process = Command::new("hyprctl").arg("-j").arg("monitors").arg("all").stdout(Stdio::piped()).spawn()?;
        let stdout = process.stdout.take().ok_or(anyhow!("Unable to read hyprctl stdout"))?;
        let reader = BufReader::new(stdout);
        let monitors = serde_json::from_reader(reader)?;

        Ok(monitors)
    }

    pub fn enable_monitor(&self, name: &str) -> Result<()> {
        let status = Command::new("hyprctl").arg("keyword").arg("monitor").arg(format!("{name}, enable")).status()?;
        if !status.success() {
            tracing::error!("Unable to enable monitor");
        }

        Ok(())
    }

    pub fn disable_monitor(&self, name: &str) -> Result<()> {
        let status = Command::new("hyprctl").arg("keyword").arg("monitor").arg(format!("{name}, disable")).status()?;
        if !status.success() {
            tracing::error!("Unable to disable monitor");
        }

        Ok(())
    }
}
