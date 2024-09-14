use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::BufReader,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct Unicode {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodePoint {
    pub char: String,
    pub name: String,
}

impl Unicode {
    pub fn list_codepoints(&self) -> Result<Vec<CodePoint>> {
        let mut process = Command::new("uni")
            .arg("print")
            .arg("-f")
            .arg("%(char) %(name)")
            .arg("-j")
            .arg("block:Miscellaneous Symbols and Pictographs")
            .arg("-or")
            .arg("block:Emoticons")
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = process.stdout.take().ok_or(anyhow!("Unable to read uni stdout"))?;
        let reader = BufReader::new(stdout);
        let items = serde_json::from_reader(reader)?;
        Ok(items)
    }
}
