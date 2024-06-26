use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    io::BufReader,
    process::{Command, Stdio},
};

pub struct Pipewire {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: u8,

    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: String,

    pub version: u8,

    #[serde(default)]
    pub info: Info,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Info {
    #[serde(default)]
    pub state: String,
    pub props: Properties,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Properties {
    #[serde(rename = "application.id")]
    #[serde(default)]
    pub application_id: String,

    #[serde(rename = "application.name")]
    #[serde(default)]
    pub application_name: String,

    #[serde(rename = "media.class")]
    #[serde(default)]
    pub media_class: String,

    #[serde(rename = "node.description")]
    #[serde(default)]
    pub node_description: String,

    #[serde(rename = "node.nick")]
    #[serde(default)]
    pub node_nick: String,
}

#[derive(Debug)]
pub struct AudioSink {
    pub id: u8,
    pub name: String,
    pub description: String,
}

impl Pipewire {
    pub fn list_audio_sinks() -> Result<Vec<AudioSink>> {
        let mut process = Command::new("pw-dump").arg("--no-colors").stdout(Stdio::piped()).spawn()?;
        let stdout = process.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let entities: Vec<Entity> = serde_json::from_reader(reader)?;

        Ok(entities
            .into_iter()
            .filter(|e| e.info.props.media_class == "Audio/Sink")
            .map(|e| AudioSink {
                id: e.id,
                name: e.info.props.node_nick,
                description: e.info.props.node_description,
            })
            .collect())
    }

    pub fn enable_audio_sink(sink_id: u8) -> Result<()> {
        let status = Command::new("wpctl").arg("set-default").arg(format!("{sink_id}")).status()?;
        if !status.success() {
            println!("Unable to enable audio sink");
        }

        Ok(())
    }
}
