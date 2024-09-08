use crate::external::{hyprland, s_search, unicode, xdg};
use crate::{
    external::{buku, pipewire},
    workflow,
};
use anyhow::Result;
use core::fmt;
use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct Root {
    buku: Rc<buku::ClientWithCache>,
    hyprland: Rc<hyprland::Client>,
    pipewire: Rc<pipewire::Client>,
    s_search: Rc<s_search::Client>,
    unicode: Rc<unicode::Unicode>,
    xdg: Rc<xdg::Client>,
}

impl Root {
    pub fn new(
        buku: Rc<buku::ClientWithCache>, hyprland: Rc<hyprland::Client>, pipewire: Rc<pipewire::Client>, s_search: Rc<s_search::Client>,
        unicode: Rc<unicode::Unicode>, xdg: Rc<xdg::Client>,
    ) -> Root {
        Root {
            buku,
            hyprland,
            pipewire,
            s_search,
            unicode,
            xdg,
        }
    }
}

impl workflow::NodeChoices for Root {
    fn prompt(&self) -> String {
        "Workflow > ".to_string()
    }

    fn next(&self) -> Result<Vec<workflow::Node>> {
        let audio_sink = workflow::audio_sink::AudioSink::new(Rc::clone(&self.pipewire));
        let bookmarks = workflow::bookmarks::Bookmarks::new(Rc::clone(&self.buku), Rc::clone(&self.xdg));
        let hyprland = workflow::hyprland::Hyprland::new(Rc::clone(&self.hyprland));
        let run = workflow::run::Run::new(Rc::clone(&self.xdg));
        let unicode = workflow::unicode::Unicode::new(Rc::clone(&self.unicode));
        let websearch = workflow::websearch::Websearch::new("firefox", Rc::clone(&self.s_search));

        let output = vec![
            audio_sink.into_node(),
            bookmarks.into_node(),
            hyprland.into_node(),
            run.into_node(),
            unicode.into_node(),
            websearch.into_node(),
        ];

        Ok(output)
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root workflow")
    }
}
