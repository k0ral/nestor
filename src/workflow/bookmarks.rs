use crate::workflow;
// use crate::workflow::NodeChoices;
// use crate::workflow::NodeFreeText;
use crate::external::buku;
use crate::external::xdg::Xdg;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Bookmarks {}

impl workflow::NodeChoices for Bookmarks {
    fn prompt(&self) -> String {
        "Bookmark > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(buku::Buku::list()?.into_iter().map(|buku_item| Bookmarks2 { buku_item }.into_node()).collect())
    }
}

impl Display for Bookmarks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bookmarks workflow")
    }
}

pub struct Bookmarks2 {
    buku_item: buku::BukuItem,
}

impl workflow::NodeRun for Bookmarks2 {
    fn run(&self) -> Result<()> {
        let uri = self.buku_item.uri.parse::<http::Uri>()?;
        Xdg::open_uri(&uri)
    }
}

impl Display for Bookmarks2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOKMARK {} | {} | {}", self.buku_item.title, self.buku_item.description.replace('\n', ""), self.buku_item.uri)
    }
}
