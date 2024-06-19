use crate::external::surfraw::{Elvi, Surfraw};
use crate::workflow;
use core::fmt;
use std::fmt::Display;
// use crate::workflow::NodeChoices;
use crate::workflow::NodeFreeText;
use crate::workflow::NodeRun;
use anyhow::Result;

#[derive(Clone)]
pub struct Websearch {
    browser: String,
}

impl Websearch {
    pub fn new(browser: &str) -> Websearch {
        Websearch {
            browser: browser.to_string(),
        }
    }
}

impl workflow::NodeChoices for Websearch {
    fn prompt(&self) -> String {
        "Elvi > ".to_string()
    }

    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(Surfraw::list_elvis()?
            .iter()
            .map(|elvi| {
                Websearch2 {
                    browser: self.browser.clone(),
                    elvi: elvi.clone(),
                }
                .into_node()
            })
            .collect())
    }
}

impl Display for Websearch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Websearch\t{}", self.browser)
    }
}

#[derive(Debug, Clone)]
pub struct Websearch2 {
    browser: String,
    elvi: Elvi,
}

impl Websearch2 {}

impl workflow::NodeFreeText for Websearch2 {
    fn prompt(&self) -> String {
        "Query > ".to_string()
    }

    fn next(&self, query: &str) -> Result<workflow::Node> {
        Ok(Websearch3 {
            browser: self.browser.clone(),
            elvi: self.elvi.clone(),
            query: query.to_string(),
        }
        .into_node())
    }
}

impl Display for Websearch2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.elvi.name, self.elvi.description)
    }
}

#[derive(Clone)]
pub struct Websearch3 {
    browser: String,
    elvi: Elvi,
    query: String,
}

impl workflow::NodeRun for Websearch3 {
    fn run(&self) -> Result<()> {
        Surfraw::search(&self.browser, &self.elvi.name, &self.query)
    }
}

impl Display for Websearch3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {}", self.browser, self.elvi, self.query)
    }
}
