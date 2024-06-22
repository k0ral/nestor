use crate::workflow;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub struct Combo {}

impl workflow::NodeChoices for Combo {
    fn prompt(&self) -> String {
        "> ".to_string()
    }

    fn next(&self) -> Result<Vec<workflow::Node>> {
        let mut output = vec![];

        let websearch = workflow::websearch::Websearch::new("firefox");
        output.append(&mut websearch.next()?);

        let bookmarks = workflow::bookmarks::Bookmarks {};
        output.append(&mut bookmarks.next()?);

        let run = workflow::run::Run {};
        output.append(&mut run.next()?);

        let audio_sink = workflow::audio_sink::AudioSink {};
        output.append(&mut audio_sink.next()?);

        Ok(output)
    }
}

impl Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Combo")
    }
}
