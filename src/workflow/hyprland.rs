use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::external::hyprland;

#[derive(Debug)]
pub struct Hyprland {
    client: Rc<hyprland::Client>,
}

impl Hyprland {
    pub fn new(client: Rc<hyprland::Client>) -> Self {
        Self { client }
    }
}

impl workflow::NodeChoices for Hyprland {
    fn prompt(&self) -> String {
        "Hyprland > ".to_string()
    }

    #[tracing::instrument]
    fn next(&self) -> Result<Vec<workflow::Node>> {
        let mut children = vec![];
        children.extend(self.client.list_monitors()?.into_iter().map(|monitor| {
            HyprlandToggleMonitor {
                client: Rc::clone(&self.client),
                monitor,
            }
            .into_node()
        }));

        Ok(children)
    }
}

impl Display for Hyprland {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hyprland workflow")
    }
}

pub struct HyprlandToggleMonitor {
    client: Rc<hyprland::Client>,
    monitor: hyprland::Monitor,
}

impl workflow::NodeRun for HyprlandToggleMonitor {
    fn run(&self) -> Result<()> {
        if self.monitor.disabled {
            self.client.enable_monitor(&self.monitor.name)?;
        } else {
            self.client.disable_monitor(&self.monitor.name)?;
        }

        Ok(())
    }
}

impl Display for HyprlandToggleMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HYPRLAND Toggle monitor {} | {}", self.monitor.name, self.monitor.description)
    }
}
