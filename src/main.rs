use std::rc::Rc;

use anyhow::Result;
use external::{buku, fuzzel, hyprland, pipewire, s_search, unicode, xdg};
use tracing_subscriber::{self, fmt::format::FmtSpan};
use workflow::NodeChoices;

mod external;
mod workflow;

fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt().with_span_events(FmtSpan::CLOSE).with_target(false).with_level(false).init();

    // External programs
    let prompter = fuzzel::Client::new("bottom".to_string(), 160);
    let buku = Rc::new(buku::ClientWithCache::new(buku::Client {}, buku::Cache {}));
    let hyprland = Rc::new(hyprland::Client {});
    let pipewire = Rc::new(pipewire::Client {});
    let s_search = Rc::new(s_search::Client {});
    let unicode = Rc::new(unicode::Unicode {});
    let xdg = Rc::new(xdg::Client {});

    // Workflows
    //let mut current = workflow::combo::Combo::new(audio_sink, bookmarks, hyprland, run, unicode, websearch).into_node();
    let mut current = workflow::root::Root::new(buku.clone(), hyprland, pipewire, s_search, unicode, xdg).into_node();

    loop {
        match current {
            workflow::Node::Choices(value) => {
                let choices = value.next()?;
                let choice = prompter.prompt_choices(&value.prompt(), choices)?;
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
