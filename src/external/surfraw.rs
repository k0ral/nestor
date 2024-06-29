use anyhow::{anyhow, Result};
use core::fmt;
use nom::{
    character::complete::{alphanumeric1, char, multispace1, not_line_ending},
    IResult,
};
use std::{
    fmt::Display,
    io::{BufRead, BufReader, Lines},
    process::{self, ChildStdout, Command, Stdio},
};

#[allow(dead_code)]
pub struct Client {}

#[derive(Debug, Clone)]
pub struct Elvi {
    pub name: String,
    pub description: String,
}

pub struct ElviIterator {
    lines: Lines<BufReader<ChildStdout>>,
}

impl Client {
    #[allow(dead_code)]
    pub fn search(browser: &str, elvi: &str, query: &str) -> Result<()> {
        let status = Command::new("surfraw")
            .arg(format!("-browser={browser}"))
            .arg(elvi)
            .arg(query)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .status()?;
        if !status.success() {
            return Err(anyhow!("Surfraw search failed"));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn elvis_lines() -> Result<Lines<BufReader<ChildStdout>>> {
        let mut process = Command::new("surfraw").arg("-elvi").stdin(Stdio::null()).stdout(Stdio::piped()).spawn()?;
        let stdout: process::ChildStdout = process.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        Ok(reader.lines())
    }

    #[allow(dead_code)]
    pub fn list_elvis() -> Result<Vec<Elvi>> {
        let lines = Self::elvis_lines()?;
        let elvi = ElviIterator::new(lines)?;
        elvi.collect::<Result<Vec<Elvi>>>()
    }
}

impl ElviIterator {
    #[allow(dead_code)]
    pub fn new(mut lines: Lines<BufReader<ChildStdout>>) -> Result<ElviIterator> {
        let line = lines.next().ok_or(anyhow!("Empty surfraw output"))??;

        if line != " GLOBAL ELVI:" {
            return Err(anyhow!("Invalid surfraw output: {}", line));
        }

        Ok(ElviIterator { lines })
    }

    fn elvi_line(input: &str) -> IResult<&str, Elvi> {
        let (next, name) = alphanumeric1(input)?;
        let (next2, _) = multispace1(next)?;
        let (next3, _) = char('-')(next2)?;
        let (next4, _) = char('-')(next3)?;
        let (next5, _) = multispace1(next4)?;
        let (leftover, description) = not_line_ending(next5)?;

        Ok((
            leftover,
            Elvi {
                name: name.to_string(),
                description: description.to_string(),
            },
        ))
    }

    fn parse_elvi_line(input: &str) -> Result<Elvi> {
        let (_, elvi) = Self::elvi_line(input).map_err(|e| e.to_owned())?;
        Ok(elvi)
    }
}

impl Iterator for ElviIterator {
    type Item = Result<Elvi>;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_item = self.lines.next()?.map_err(|e| e.into());
        let item = raw_item.and_then(|line| Self::parse_elvi_line(&line));
        Some(item)
    }
}

impl Display for Elvi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.name, self.description)
    }
}
