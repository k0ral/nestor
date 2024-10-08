use crate::external::buku;
use crate::external::xdg;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use notify_rust::Notification;
use notify_rust::Urgency;
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
    fn next(self: Box<Self>) -> Result<Vec<workflow::Node>> {
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
        write!(f, "Bookmarks")
    }
}

pub struct Bookmarks2 {
    buku_item: buku::BukuItem,
    xdg: Rc<xdg::Client>,
}

impl workflow::NodeRun for Bookmarks2 {
    fn run(&self) -> Result<()> {
        let uri = self.buku_item.uri.parse::<http::Uri>()?;
        self.xdg.open_uri(&uri)?;
        Notification::new()
            .summary(&format!("Opened bookmark {}", self.buku_item.title))
            .body(&self.buku_item.uri)
            .urgency(Urgency::Low)
            .show()?;
        Ok(())
    }
}

impl Display for Bookmarks2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:40.40}   {}", self.buku_item.title, self.buku_item.uri)
    }
}
