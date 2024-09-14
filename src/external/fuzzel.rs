use anyhow::{anyhow, Result};
use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

pub struct Client {
    anchor: String,
    width: u16,
}

impl Client {
    pub fn new(anchor: String, width: u16) -> Client {
        Client { anchor, width }
    }

    pub fn prompt_freetext(&self, prompt: &str) -> Result<String> {
        let output = Command::new("fuzzel")
            .arg(format!("--anchor={}", self.anchor))
            .arg(format!("--width={}", self.width))
            .arg("--dmenu")
            .arg("--prompt")
            .arg(prompt)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }

    pub fn prompt_choices<T>(&self, prompt: &str, mut choices: Vec<T>) -> Result<T>
    where
        T: Display,
    {
        let mut process = Command::new("fuzzel")
            .arg(format!("--anchor={}", self.anchor))
            .arg(format!("--width={}", self.width))
            .arg("--prompt")
            .arg(prompt)
            .arg("--dmenu")
            .arg("--index")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut stdin = process.stdin.take().ok_or(anyhow!("Unable to acquire fuzzel stdin"))?;

        for item in &choices {
            stdin.write_all(item.to_string().as_bytes())?;
            stdin.write_all(b"\n")?;
        }

        drop(stdin);
        let output = process.wait_with_output()?;
        tracing::info!("Fuzzel output: {output:?}");

        let index: usize = std::str::from_utf8(&output.stdout)?.trim().parse()?;
        let selected = choices.remove(index);
        Ok(selected)
    }
}
