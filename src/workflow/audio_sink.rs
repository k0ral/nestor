use crate::workflow;
// use crate::workflow::NodeChoices;
// use crate::workflow::NodeFreeText;
use crate::external::pipewire;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct AudioSink {}

impl workflow::NodeChoices for AudioSink {
    fn prompt(&self) -> String {
        "Audio sink > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(pipewire::Pipewire::list_audio_sinks()?.into_iter().map(|s| AudioSink2 { sink: s }.into_node()).collect())
    }
}

impl Display for AudioSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Audio sink workflow")
    }
}

pub struct AudioSink2 {
    sink: pipewire::AudioSink,
}

impl workflow::NodeRun for AudioSink2 {
    fn run(&self) -> Result<()> {
        pipewire::Pipewire::enable_audio_sink(self.sink.id)
    }
}

impl Display for AudioSink2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AUDIO {} | {} | {}", self.sink.id, self.sink.name, self.sink.description)
    }
}
