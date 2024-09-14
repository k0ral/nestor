use std::rc::Rc;

use anyhow::Result;
use external::{buku, fuzzel, hyprland, pipewire, s_search, sops, udiskie, unicode, xdg};
use tracing_subscriber::{self, fmt::format::FmtSpan};
use workflow::NodeChoices;

mod config;
mod external;
mod json;
mod workflow;

fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt().with_span_events(FmtSpan::CLOSE).with_target(false).with_level(false).init();
    let config = config::Config::new()?;

    // External programs
    let prompter = fuzzel::Client::new("bottom".to_string(), 160);
    let buku = Rc::new(buku::ClientWithCache::new(buku::Client {}, buku::Cache {}));
    let hyprland = Rc::new(hyprland::Client {});
    let pipewire = Rc::new(pipewire::Client {});
    let s_search = Rc::new(s_search::Client {});
    let sops = Rc::new(sops::Client {});
    let udiskie = Rc::new(udiskie::Client {});
    let unicode = Rc::new(unicode::Unicode {});
    let xdg = Rc::new(xdg::Client {});

    // Workflows
    let audio_sink = workflow::audio_sink::AudioSink::new(Rc::clone(&pipewire));
    let bookmarks = workflow::bookmarks::Bookmarks::new(Rc::clone(&buku), Rc::clone(&xdg));
    let disks = workflow::disks::Disks::new(Rc::clone(&udiskie));
    let hyprland = workflow::hyprland::Hyprland::new(Rc::clone(&hyprland));
    let run = workflow::run::Run::new(Rc::clone(&xdg));
    let secrets = workflow::secrets::Secrets::new(Rc::clone(&sops), config.secrets_file.into());
    let unicode = workflow::unicode::Unicode::new(Rc::clone(&unicode));
    let websearch = workflow::websearch::Websearch::new(&config.browser, Rc::clone(&s_search));

    let all_workflows: Vec<workflow::Node> = vec![
        audio_sink.into_node(),
        bookmarks.into_node(),
        disks.into_node(),
        hyprland.into_node(),
        run.into_node(),
        secrets.into_node(),
        unicode.into_node(),
        websearch.into_node(),
    ];

    let mut current = workflow::root::Root::new(all_workflows).into_node();

    loop {
        match current {
            workflow::Node::Choices(value) => {
                let prompt = &value.prompt();
                let choices = value.next()?;
                let choice = prompter.prompt_choices(&prompt, choices)?;
                current = choice;
            }
            workflow::Node::FreeText(value) => {
                let string = prompter.prompt_freetext(&value.prompt())?;
                current = value.next(&string)?;
            }
            workflow::Node::Run(value) => {
                value.run()?;
                break;
            }
        }
    }

    buku.refresh_cache()?;
    Ok(())
}
