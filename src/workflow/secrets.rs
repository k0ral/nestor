use crate::external::clipboard;
use crate::external::sops;
use crate::workflow;
use crate::workflow::NodeRun;
use anyhow::Result;
use core::fmt;
use notify_rust::Notification;
use notify_rust::Urgency;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Secrets {
    sops: Rc<sops::Client>,
    file: PathBuf,
}

impl Secrets {
    pub fn new(sops: Rc<sops::Client>, file: PathBuf) -> Secrets {
        Secrets { sops, file }
    }
}

impl workflow::NodeChoices for Secrets {
    fn prompt(&self) -> String {
        "Secret > ".to_string()
    }

    #[tracing::instrument]
    fn next(self: Box<Self>) -> Result<Vec<workflow::Node>> {
        let f = File::open(&self.file)?;
        let secrets_tree: serde_json::Value = serde_json::from_reader(BufReader::new(f))?;
        let secrets = crate::json::ScalarIterator::new(secrets_tree);

        Ok(secrets
            .map(|(path, _value)| {
                Secrets2 {
                    sops: Rc::clone(&self.sops),
                    file: self.file.clone(),
                    path,
                }
                .into_node()
            })
            .collect())
    }
}

impl Display for Secrets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Secrets")
    }
}

#[derive(Clone)]
pub struct Secrets2 {
    sops: Rc<sops::Client>,
    file: PathBuf,
    path: Vec<crate::json::PathStep>,
}

impl workflow::NodeRun for Secrets2 {
    fn run(&self) -> Result<()> {
        let secret = self.sops.decrypt_secret(&self.file, &self.path)?;
        clipboard::Client::copy(&secret)?;
        Notification::new()
            .summary("Copied secret into clipboard")
            .body(&format!("{}", crate::json::PathStep::to_json_pointer(&self.path)))
            .urgency(Urgency::Normal)
            .show()?;
        Ok(())
    }
}

impl Display for Secrets2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", crate::json::PathStep::to_json_pointer(&self.path))
    }
}
