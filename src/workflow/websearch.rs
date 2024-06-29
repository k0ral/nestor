use crate::external::s_search;
use crate::workflow;
use crate::workflow::NodeFreeText;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Websearch {
    browser: String,
    client: Rc<s_search::Client>,
}

impl Websearch {
    pub fn new(browser: &str, client: Rc<s_search::Client>) -> Websearch {
        Websearch {
            browser: browser.to_string(),
            client,
        }
    }
}

impl workflow::NodeChoices for Websearch {
    fn prompt(&self) -> String {
        "Provider > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(self
            .client
            .list_providers()?
            .into_iter()
            .map(|provider| {
                Websearch2 {
                    browser: self.browser.clone(),
                    provider,
                    client: Rc::clone(&self.client),
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
    provider: s_search::Provider,
    client: Rc<s_search::Client>,
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
            client: Rc::clone(&self.client),
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
    provider: s_search::Provider,
    query: String,
    client: Rc<s_search::Client>,
}

impl workflow::NodeRun for Websearch3 {
    fn run(&self) -> Result<()> {
        self.client.search(&self.browser, &self.provider, &self.query)
    }
}

impl Display for Websearch3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {}", self.browser, self.provider, self.query)
    }
}
