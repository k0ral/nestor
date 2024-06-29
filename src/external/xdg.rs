use anyhow::Result;
use http::Uri;
use std::fs;
use std::process::Command;

use freedesktop_desktop_entry::{default_paths, get_languages_from_env, DesktopEntry, Iter};

pub struct Xdg {}

#[derive(Debug)]
pub struct Application {
    pub name: String,
    pub comment: Option<String>,
    pub exec: String,
}

impl Xdg {
    pub fn open_uri(uri: &Uri) -> Result<()> {
        tracing::info!("Browsing {uri:?}");
        let status = Command::new("xdg-open").arg(uri.to_string()).status()?;
        if !status.success() {
            tracing::error!("Unable to browse URI");
        }

        Ok(())
    }

    pub fn list_desktop_applications() -> Vec<Application> {
        let mut results = vec![];
        let locales = get_languages_from_env();

        for path in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::from_str(&path, &bytes, &locales) {
                    results.push(Application {
                        name: entry.name(&locales).unwrap().to_string(),
                        comment: entry.comment(&locales).map(|s| s.to_string()),
                        exec: entry.exec().unwrap().to_string(),
                    });
                }
            }
        }

        results
    }
}
