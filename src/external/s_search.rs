use anyhow::{anyhow, Result};
use core::fmt;
use std::fmt::Display;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Client {}

#[derive(Clone)]
pub struct Provider(String);

impl Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Client {
    pub fn search(&self, browser: &str, provider: &Provider, query: &str) -> Result<()> {
        let status = Command::new("s")
            .arg("-b")
            .arg(browser)
            .arg("-p")
            .arg(&provider.0)
            .arg(query)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .status()?;
        if !status.success() {
            return Err(anyhow!("s search failed"));
        }

        Ok(())
    }

    pub fn list_providers(&self) -> Result<Vec<Provider>> {
        let output = Command::new("s").arg("--list-providers").stdin(Stdio::null()).stdout(Stdio::piped()).output()?;
        Ok(std::str::from_utf8(&output.stdout)?.lines().map(|p| Provider(p.to_owned())).collect())
    }
}
