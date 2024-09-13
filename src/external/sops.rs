use anyhow::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn decrypt_secret(&self, file: &PathBuf, secret_path: &Vec<crate::json::PathStep>) -> Result<String> {
        let output = Command::new("sops")
            .arg("decrypt")
            .arg("--extract")
            .arg(crate::json::PathStep::to_json_path(secret_path))
            .arg(file)
            .stdout(Stdio::piped())
            .output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }
}
