use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::handlers::utils::print_not_implemented;
use crate::app::state::AppState;

pub struct SampleCommand { }

impl SampleCommand {
    pub fn validator(_state: &mut AppState, cmd: &str) -> ValidatorResult {
        if cmd.eq("sample") {
            ValidatorResult::Valid
        } else {
            ValidatorResult::Invalid
        }
    }
    pub fn handler(_state: &mut AppState, _cmd: &str) -> HandlerResult {
        print_not_implemented();
        HandlerResult::ContinueLoop
    }
}

impl CommandConfig for SampleCommand {
    fn config() -> Command {
        Command::configure("sample")
            .short_desc("Runs the sample routine.")
            .category("Basic")
            .validate(SampleCommand::validator)
            .handle(SampleCommand::handler)
            .configured()
    }
}
