use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::state::AppState;
use crate::app::ui::render::exit as render;

pub struct ExitCommand { }

impl ExitCommand {
    pub fn validator(_state: &mut AppState, cmd: &str) -> ValidatorResult {
        if cmd.eq("exit") || cmd.eq("quit") || cmd.eq("qui") || cmd.eq("qu") || cmd.eq("q") {
            ValidatorResult::Valid
        } else {
            ValidatorResult::Invalid
        }
    }
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
            .validate(ExitCommand::validator)
            .handle(ExitCommand::handler)
            .configured()
    }
}
