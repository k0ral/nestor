use crate::external::buku;
use crate::external::xdg;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Bookmarks {
    buku: Rc<buku::ClientWithCache>,
    xdg: Rc<xdg::Client>,
}

impl Bookmarks {
    pub fn new(buku: Rc<buku::ClientWithCache>, xdg: Rc<xdg::Client>) -> Bookmarks {
        Bookmarks { buku, xdg }
    }
}

impl workflow::NodeChoices for Bookmarks {
    fn prompt(&self) -> String {
        "Bookmark > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(self
            .buku
            .list()?
            .into_iter()
            .map(|buku_item| {
                Bookmarks2 {
                    buku_item,
                    xdg: Rc::clone(&self.xdg),
                }
                .into_node()
            })
            .collect())
    }
}

impl Display for Bookmarks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bookmarks workflow")
    }
}

pub struct Bookmarks2 {
    buku_item: buku::BukuItem,
    xdg: Rc<xdg::Client>,
}

impl workflow::NodeRun for Bookmarks2 {
    fn run(&self) -> Result<()> {
        let uri = self.buku_item.uri.parse::<http::Uri>()?;
        self.xdg.open_uri(&uri)
    }
}

impl Display for Bookmarks2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOKMARK {} | {} | {}", self.buku_item.title, self.buku_item.description.replace('\n', ""), self.buku_item.uri)
    }
}
