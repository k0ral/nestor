use crate::external::udiskie;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Disks {
    udiskie: Rc<udiskie::Client>,
}

impl Disks {
    pub fn new(udiskie: Rc<udiskie::Client>) -> Self {
        Self { udiskie }
    }
}

impl workflow::NodeChoices for Disks {
    fn prompt(&self) -> String {
        "Disks > ".to_string()
    }

    #[tracing::instrument]
    fn next(self: Box<Self>) -> Result<Vec<workflow::Node>> {
        let mut children = vec![];
        children.extend(self.udiskie.list_devices()?.into_iter().filter_map(|device| {
            match (device.is_mounted.as_str(), device.is_luks.as_str(), device.is_unlocked.as_str()) {
                ("True", _, _) => Some(
                    DisksUmount {
                        udiskie: Rc::clone(&self.udiskie),
                        device,
                    }
                    .into_node(),
                ),
                ("False", "True", "False") | ("False", "False", _) => Some(
                    DisksMount {
                        udiskie: Rc::clone(&self.udiskie),
                        device,
                    }
                    .into_node(),
                ),
                _ => None,
            }
        }));

        Ok(children)
    }
}

impl Display for Disks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Disks")
    }
}

pub struct DisksUmount {
    udiskie: Rc<udiskie::Client>,
    device: udiskie::Device,
}

impl workflow::NodeRun for DisksUmount {
    fn run(&self) -> Result<()> {
        self.udiskie.unmount(&self.device.id)?;
        Ok(())
    }
}

impl Display for DisksMount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mount {:40.40}   {}", self.device.id, self.device.label)
    }
}

pub struct DisksMount {
    udiskie: Rc<udiskie::Client>,
    device: udiskie::Device,
}

impl workflow::NodeRun for DisksMount {
    fn run(&self) -> Result<()> {
        self.udiskie.mount(&self.device.id)?;
        Ok(())
    }
}

impl Display for DisksUmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unmount {:40.40}   {:20.20}   {}", self.device.id, self.device.label, self.device.mount_path)
    }
}
