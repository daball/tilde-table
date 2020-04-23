use crate::shell::command::{Command, CommandDefinition};
use crate::app::state::AppState;
use crate::app::ui::render::exit as render;

pub struct ExitCommand { }

impl Command for ExitCommand {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("?")
            .alias("help")
            .short_desc("Prints this help page.")
            .category("Basic")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.eq("exit") || cmd.eq("quit") || cmd.eq("qui") || cmd.eq("qu") || cmd.eq("q")
    }
    fn execute(&self, _state: &mut AppState, _cmd: &str) -> bool {
        render::goodbye();
        true // continue read-eval-print-loop    
    }
}
