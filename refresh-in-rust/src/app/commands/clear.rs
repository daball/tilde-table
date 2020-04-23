use crate::shell::command::{Command, CommandDefinition};
use crate::app::state::AppState;
use std::io::{stdout, Write};

#[cfg(feature="ansi_term")]
pub fn clear_screen() {
    stdout().write(&[0o033, b'c'][..]);
}

#[cfg(not(feature="ansi_term"))]
pub use crate::app::handlers::utils::print_not_implemented_requires_ansi as clear_screen;

pub struct ClearCommand { }

impl Command for ClearCommand {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("clear")
            .alias("cls")
            .short_desc("Clears the screen.")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.eq("cls") || cmd.eq("clear")   
    }
    fn execute(&self, _state: &mut AppState, _cmd: &str) -> bool {
        clear_screen();
        true // continue read-eval-print-loop    
    }
}
