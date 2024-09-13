use crate::workflow;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub struct Root {
    children: Vec<workflow::Node>,
}

impl Root {
    pub fn new(children: Vec<workflow::Node>) -> Self {
        Self { children }
    }
}

impl workflow::NodeChoices for Root {
    fn prompt(&self) -> String {
        "Workflow > ".to_string()
    }

    fn next(self: Box<Self>) -> Result<Vec<workflow::Node>> {
        Ok(self.children)
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root workflow")
    }
}
