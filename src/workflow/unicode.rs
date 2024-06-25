use crate::workflow;
// use crate::workflow::NodeChoices;
// use crate::workflow::NodeFreeText;
use crate::external::clipboard;
use crate::external::unicode;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub struct Unicode {}

impl workflow::NodeChoices for Unicode {
    fn prompt(&self) -> String {
        "Unicode > ".to_string()
    }

    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(unicode::Unicode::list_codepoints()?.into_iter().map(|c| Unicode2 { codepoint: c }.into_node()).collect())
    }
}

impl Display for Unicode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unicode workflow")
    }
}

pub struct Unicode2 {
    codepoint: unicode::CodePoint,
}

impl workflow::NodeRun for Unicode2 {
    fn run(&self) -> Result<()> {
        clipboard::Clipboard::copy(&self.codepoint.char)
    }
}

impl Display for Unicode2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UNICODE {} | {}", self.codepoint.char, self.codepoint.name)
    }
}
