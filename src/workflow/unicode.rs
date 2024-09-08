use crate::workflow;
// use crate::workflow::NodeChoices;
// use crate::workflow::NodeFreeText;
use crate::external::clipboard;
use crate::external::unicode;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use notify_rust::Notification;
use notify_rust::Urgency;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Unicode {
    unicode: Rc<unicode::Unicode>,
}

impl Unicode {
    pub fn new(unicode: Rc<unicode::Unicode>) -> Self {
        Self { unicode }
    }
}

impl workflow::NodeChoices for Unicode {
    fn prompt(&self) -> String {
        "Unicode > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(self.unicode.list_codepoints()?.into_iter().map(|c| Unicode2 { codepoint: c }.into_node()).collect())
    }
}

impl Display for Unicode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unicode")
    }
}

pub struct Unicode2 {
    codepoint: unicode::CodePoint,
}

impl workflow::NodeRun for Unicode2 {
    fn run(&self) -> Result<()> {
        clipboard::Client::copy(&self.codepoint.char)?;
        Notification::new()
            .summary(&format!("Copied {} to clipboard", self.codepoint.char))
            .body(&self.codepoint.name)
            .urgency(Urgency::Low)
            .show()?;
        Ok(())
    }
}

impl Display for Unicode2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:3.3}   {}", self.codepoint.char, self.codepoint.name)
    }
}
