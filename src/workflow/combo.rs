use crate::workflow;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub struct Combo {
    audio_sink: workflow::audio_sink::AudioSink,
    bookmarks: workflow::bookmarks::Bookmarks,
    run: workflow::run::Run,
    unicode: workflow::unicode::Unicode,
    websearch: workflow::websearch::Websearch,
}

impl Combo {
    pub fn new(
        audio_sink: workflow::audio_sink::AudioSink, bookmarks: workflow::bookmarks::Bookmarks, run: workflow::run::Run,
        unicode: workflow::unicode::Unicode, websearch: workflow::websearch::Websearch,
    ) -> Combo {
        Combo {
            audio_sink,
            bookmarks,
            run,
            unicode,
            websearch,
        }
    }
}

impl workflow::NodeChoices for Combo {
    fn prompt(&self) -> String {
        "> ".to_string()
    }

    fn next(&self) -> Result<Vec<workflow::Node>> {
        let mut output = vec![];

        output.append(&mut self.audio_sink.next()?);
        output.append(&mut self.bookmarks.next()?);
        output.append(&mut self.run.next()?);
        output.append(&mut self.websearch.next()?);
        output.append(&mut self.unicode.next()?);

        Ok(output)
    }
}

impl Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Combo")
    }
}
