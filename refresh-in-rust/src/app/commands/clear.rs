use crate::shell::command::{Command, CommandConfig, HandlerResult};
use crate::app::state::AppState;
use crate::app::ui::render::clear as render;

pub struct ClearCommand { }

impl ClearCommand {
    pub fn handler(_state: &mut AppState, _cmd: &str) -> HandlerResult {
        print!("{}", render::clear_screen());
        HandlerResult::ContinueLoop
    }
}

impl CommandConfig for ClearCommand
{
    fn config() -> Command {
        Command::configure("clear").alias("cls")
            .short_desc("Clears the screen.")
            .category("Basic")
            .handle(ClearCommand::handler)
            .configured()
    }
}
