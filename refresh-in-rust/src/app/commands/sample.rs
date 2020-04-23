use crate::shell::command::{Command, CommandDefinition};
use crate::app::handlers::utils::print_not_implemented;
use crate::app::state::AppState;

pub struct SampleCommand { }

impl Command for SampleCommand {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("sample")
            .short_desc("Runs the sample routine.")
            .category("Basic")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.eq("sample")
    }
    fn execute(&self, _state: &mut AppState, _cmd: &str) -> bool {
        print_not_implemented();
        true
    }
}
