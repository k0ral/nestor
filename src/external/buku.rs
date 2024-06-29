use core::fmt;
use std::{
    fmt::Display,
    io::BufReader,
    process::{Command, Stdio},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct Buku {}

#[derive(Serialize, Deserialize, Debug)]
pub struct BukuItem {
    pub description: String,
    pub index: u32,
    pub tags: String,
    pub title: String,
    pub uri: String,
}

impl Buku {
    #[tracing::instrument]
    pub fn list() -> Result<Vec<BukuItem>> {
        let mut process = Command::new("buku").arg("--nostdin").arg("--print").arg("--json").arg("--nc").stdout(Stdio::piped()).spawn()?;
        let stdout = process.stdout.take().unwrap();
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
