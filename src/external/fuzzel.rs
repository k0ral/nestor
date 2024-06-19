use anyhow::Result;
use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

pub struct Fuzzel {}

impl Fuzzel {
    pub fn prompt_freetext(&self, prompt: &str) -> Result<String> {
        let output =
            Command::new("fuzzel").arg("--dmenu").arg("--prompt").arg(prompt).stdin(Stdio::null()).stdout(Stdio::piped()).output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }

    pub fn prompt_choices<T>(&self, prompt: &str, mut choices: Vec<T>) -> Result<T>
    where
        T: Display,
    {
        let mut process = Command::new("fuzzel")
            .arg("--prompt")
            .arg(prompt)
            .arg("--dmenu")
            .arg("--index")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut stdin = process.stdin.take().unwrap();

        for item in choices.iter() {
            stdin.write_all(format!("{}", item).as_bytes())?;
            stdin.write_all(b"\n")?;
        }

        drop(stdin);
        let output = process.wait_with_output()?;
        println!("{:?}", output);

        let index: usize = std::str::from_utf8(&output.stdout)?.trim().parse()?;
        let selected = choices.remove(index);
        Ok(selected)
    }
}
