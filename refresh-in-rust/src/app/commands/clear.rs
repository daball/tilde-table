use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::state::AppState;
use crate::app::ui::render::clear as render;

pub struct ClearCommand { }

impl ClearCommand {
    pub fn validator(_state: &mut AppState, cmd: &str) -> ValidatorResult {
        if cmd.eq("cls") || cmd.eq("clear") {
            ValidatorResult::Valid
        } else {
            ValidatorResult::Invalid
        }
    }
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
            .validate(ClearCommand::validator)
            .handle(ClearCommand::handler)
            .configured()
    }
}
