use crate::external::pipewire;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use notify_rust::{Notification, Urgency};
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct AudioSink {
    pipewire: Rc<pipewire::Client>,
}

impl AudioSink {
    pub fn new(pipewire: Rc<pipewire::Client>) -> AudioSink {
        AudioSink { pipewire }
    }
}

impl workflow::NodeChoices for AudioSink {
    fn prompt(&self) -> String {
        "Audio sink > ".to_string()
    }

    #[tracing::instrument]
    fn next(self: Box<Self>) -> Result<Vec<workflow::Node>> {
        Ok(self
            .pipewire
            .list_audio_sinks()?
            .into_iter()
            .map(|s| {
                AudioSink2 {
                    pipewire: Rc::clone(&self.pipewire),
                    sink: s,
                }
                .into_node()
            })
            .collect())
    }
}

impl Display for AudioSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Audio sink")
    }
}

pub struct AudioSink2 {
    pipewire: Rc<pipewire::Client>,
    sink: pipewire::AudioSink,
}

impl workflow::NodeRun for AudioSink2 {
    fn run(&self) -> Result<()> {
        self.pipewire.enable_audio_sink(self.sink.id)?;
        Notification::new()
            .summary(&format!("Switched to audio sink {}", self.sink.name))
            .body(&self.sink.description)
            .urgency(Urgency::Low)
            .show()?;

        Ok(())
    }
}

impl Display for AudioSink2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}   {:20.20}   {}", self.sink.id, self.sink.name, self.sink.description)
    }
}
