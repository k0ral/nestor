use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct Client {}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub label: String,
    pub mount_path: String,
    pub size: u64,
    pub is_mounted: String,
    pub is_unlocked: String,
    pub is_luks: String,
}

impl Client {
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        let mut process = Command::new("udiskie-info").arg("--all").arg("--output")
            .arg("{{ \"id\": \"{device_presentation}\", \"label\": \"{id_label}\", \"mount_path\": \"{mount_path}\", \"size\": {device_size}, \"is_mounted\": \"{is_mounted}\", \"is_unlocked\": \"{is_unlocked}\", \"is_luks\": \"{is_luks}\" }}")
            .stdout(Stdio::piped()).spawn()?;
        let stdout = process.stdout.take().ok_or(anyhow!("Unable to read udiskie-info stdout"))?;
        let devices = BufReader::new(stdout).lines().filter_map(|line| line.map(|l| serde_json::from_str(&l).unwrap()).ok()).collect();

        Ok(devices)
    }

    pub fn mount(&self, device_id: &str) -> Result<()> {
        let status = Command::new("udiskie-mount").arg(device_id).status()?;
        if !status.success() {
            tracing::error!("Unable to mount {}", device_id);
        }

        Ok(())
    }

    pub fn unmount(&self, device_id: &str) -> Result<()> {
        let status = Command::new("udiskie-umount").arg(device_id).status()?;
        if !status.success() {
            tracing::error!("Unable to unmount {}", device_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_list_mounted_devices() -> Result<()> {
        let client = Client {};
        let devices = client.list_devices()?;
        assert_eq!(1, devices.len());
        Ok(())
    }
}
