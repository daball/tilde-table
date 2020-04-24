use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::state::AppState;
use crate::app::ui::render::exit as render;

pub struct ExitCommand { }

impl ExitCommand {
    pub fn handler(_state: &mut AppState, _cmd: &str) -> HandlerResult {
        println!("{}", render::goodbye());
        HandlerResult::ExitLoop
    }
}

impl CommandConfig for ExitCommand {
    fn config() -> Command {
        Command::configure("exit")
            .alias("quit").alias("qui").alias("qu").alias("q")
            .short_desc("Prints this help page.")
            .category("Basic")
            .handle(ExitCommand::handler)
            .configured()
    }
}
