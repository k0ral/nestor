use anyhow::Result;
use external::fuzzel::Fuzzel;
use tracing_subscriber::{self, fmt::format::FmtSpan};
use workflow::NodeChoices;

mod external;
mod workflow;

fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt().with_span_events(FmtSpan::CLOSE).with_target(false).with_level(false).init();

    let prompter = Fuzzel::new("bottom".to_string(), "000000ff".to_string(), "000033ff".to_string(), 160);
    let mut current = workflow::combo::Combo {}.into_node();

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

    Ok(())
}
