use crate::external::s_search::{Provider, SSearch};
use crate::workflow;
use core::fmt;
use std::fmt::Display;
// use crate::workflow::NodeChoices;
use crate::workflow::NodeFreeText;
use crate::workflow::NodeRun;
use anyhow::Result;

#[derive(Debug)]
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
        "Provider > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(SSearch::list_providers()?
            .into_iter()
            .map(|provider| {
                Websearch2 {
                    browser: self.browser.clone(),
                    provider,
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

pub struct Websearch2 {
    browser: String,
    provider: Provider,
}

impl Websearch2 {}

impl workflow::NodeFreeText for Websearch2 {
    fn prompt(&self) -> String {
        "Query > ".to_string()
    }

    fn next(&self, query: &str) -> Result<workflow::Node> {
        Ok(Websearch3 {
            browser: self.browser.clone(),
            provider: self.provider.clone(),
            query: query.to_string(),
        }
        .into_node())
    }
}

impl Display for Websearch2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WEBSEARCH {}", self.provider)
    }
}

pub struct Websearch3 {
    browser: String,
    provider: Provider,
    query: String,
}

impl workflow::NodeRun for Websearch3 {
    fn run(&self) -> Result<()> {
        SSearch::search(&self.browser, &self.provider, &self.query)
    }
}

impl Display for Websearch3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {}", self.browser, self.provider, self.query)
    }
}
