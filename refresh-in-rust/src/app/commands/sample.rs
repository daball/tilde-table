use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::handlers::utils::print_not_implemented;
use crate::app::state::AppState;

pub struct SampleCommand { }

impl SampleCommand {
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
            .handle(SampleCommand::handler)
            .configured()
    }
}
