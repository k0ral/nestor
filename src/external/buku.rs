use core::fmt;
use std::{
    fmt::Display,
    fs::File,
    io::BufReader,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Client {}

#[derive(Serialize, Deserialize, Debug)]
pub struct BukuItem {
    pub description: String,
    pub index: u32,
    pub tags: String,
    pub title: String,
    pub uri: String,
}

impl Client {
    pub fn list(&self) -> Result<Vec<BukuItem>> {
        let mut process = Command::new("buku").arg("--nostdin").arg("--print").arg("--json").arg("--nc").stdout(Stdio::piped()).spawn()?;
        let stdout = process.stdout.take().ok_or(anyhow!("Unable to read buku stdout"))?;
        let reader = BufReader::new(stdout);
        let items = serde_json::from_reader(reader)?;

        Ok(items)
    }
}

impl Display for BukuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.title, self.uri)
    }
}

#[derive(Debug)]
pub struct Cache {}

impl Cache {
    pub fn list(&self) -> Result<Vec<BukuItem>> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("nestor")?;
        let path = xdg_dirs.get_cache_file("buku.json");
        let reader = File::open(path)?;
        let items = serde_json::from_reader(reader)?;

        Ok(items)
    }

    pub fn save(&self, items: &Vec<BukuItem>) -> Result<()> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("nestor")?;
        xdg_dirs.create_cache_directory(".")?;
        let path = xdg_dirs.get_cache_file("buku.json");
        let writer = File::create(path)?;
        serde_json::to_writer(writer, items)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientWithCache {
    handler: Client,
    cache: Cache,
}

impl ClientWithCache {
    pub fn new(handler: Client, cache: Cache) -> ClientWithCache {
        ClientWithCache { handler, cache }
    }

    pub fn list(&self) -> Result<Vec<BukuItem>> {
        let items = self.cache.list();
        if items.is_ok() {
            items
        } else {
            let items = self.handler.list()?;
            self.cache.save(&items)?;
            Ok(items)
        }
    }

    pub fn refresh_cache(&self) -> Result<()> {
        tracing::info!("Refreshing buku cache...");
        let items = self.handler.list()?;
        self.cache.save(&items)
    }
}
