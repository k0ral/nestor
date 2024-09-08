use crate::external::xdg;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::{anyhow, Result};
use core::fmt;
use notify_rust::{Notification, Urgency};
use std::fmt::Display;
use std::process::Command;
use std::rc::Rc;

#[derive(Debug)]
pub struct Run {
    xdg: Rc<xdg::Client>,
}

impl Run {
    pub fn new(xdg: Rc<xdg::Client>) -> Run {
        Run { xdg }
    }
}

impl workflow::NodeChoices for Run {
    fn prompt(&self) -> String {
        "Run > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        Ok(self.xdg.list_desktop_applications().into_iter().map(|application| Run2 { application }.into_node()).collect())
    }
}

impl Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Run workflow")
    }
}

pub struct Run2 {
    application: xdg::Application,
}

impl workflow::NodeRun for Run2 {
    fn run(&self) -> Result<()> {
        let mut exec = self.application.exec.split(' ');
        let mut command = Command::new(exec.next().unwrap());

        for parameter in exec {
            // See: https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#exec-variables
            if matches!(parameter, "%f" | "%F" | "%u" | "%U") {
                continue;
            }

            command.arg(parameter);
        }

        let status = command.status()?;
        if !status.success() {
            return Err(anyhow!("Command failed"));
        }

        Notification::new()
            .summary(&format!("Launched command {}", self.application.name))
            .body(&self.application.exec)
            .urgency(Urgency::Low)
            .show()?;
        Ok(())
    }
}

impl Display for Run2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comment = self.application.comment.clone().unwrap_or_default().replace('\n', "");
        write!(f, "RUN {} | {} | {}", self.application.name, comment, self.application.exec)
    }
}
