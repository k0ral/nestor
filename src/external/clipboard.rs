use anyhow::Result;
use std::process::Command;

pub struct Clipboard {}

impl Clipboard {
    pub fn copy(value: &str) -> Result<()> {
        let status = Command::new("wl-copy").arg("-p").arg(value).status()?;
        if !status.success() {
            tracing::error!("Unable to copy into clipboard");
        }

        let status = Command::new("wl-copy").arg(value).status()?;
        if !status.success() {
            tracing::error!("Unable to copy into clipboard");
        }

        Ok(())
    }
}
