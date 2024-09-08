use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub mod audio_sink;
pub mod bookmarks;
pub mod combo;
pub mod hyprland;
pub mod root;
pub mod run;
pub mod unicode;
pub mod websearch;

pub enum Node {
    Choices(Box<dyn NodeChoices>),
    FreeText(Box<dyn NodeFreeText>),
    Run(Box<dyn NodeRun>),
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Choices(value) => value.fmt(f),
            Node::FreeText(value) => value.fmt(f),
            Node::Run(value) => value.fmt(f),
        }
    }
}

pub trait NodeChoices: Display {
    fn prompt(&self) -> String;
    fn next(&self) -> Result<Vec<Node>>;

    fn into_node(self) -> Node
    where
        Self: Sized + 'static,
    {
        Node::Choices(Box::new(self))
    }
}

pub trait NodeFreeText: Display {
    fn prompt(&self) -> String;
    fn next(&self, value: &str) -> Result<Node>;

    fn into_node(self) -> Node
    where
        Self: Sized + 'static,
    {
        Node::FreeText(Box::new(self))
    }
}

pub trait NodeRun: Display {
    fn run(&self) -> Result<()>;

    fn into_node(self) -> Node
    where
        Self: Sized + 'static,
    {
        Node::Run(Box::new(self))
    }
}
