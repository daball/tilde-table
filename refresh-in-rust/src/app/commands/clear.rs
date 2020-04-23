use crate::shell::command::{Command, CommandDefinition};
use crate::app::state::AppState;
use crate::app::ui::render::clear as render;

pub struct ClearCommand { }

impl Command for ClearCommand
{
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("clear").alias("cls")
            .short_desc("Clears the screen.")
            .category("Basic")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.eq("cls") || cmd.eq("clear")
    }
    fn execute(&self, _state: &mut AppState, _cmd: &str) -> bool {
        print!("{}", render::clear_screen());
        true // continue read-eval-print-loop    
    }
}