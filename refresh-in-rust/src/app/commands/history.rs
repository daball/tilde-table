use crate::shell::command::{Command, CommandDefinition};
use crate::app::state::AppState;

pub struct HistoryCommand { }

impl Command for HistoryCommand {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("history")
            .param("n").desc("Number of items to display.").optional()
            .short_desc("Displays up to optional number of history items.")
            .category("Basic")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.starts_with("history")
    }
    fn execute(&self, state: &mut AppState, cmd: &str) -> bool {
        let cmd = cmd.trim();
        let searchAt = if cmd.starts_with("history") { 7 } else { cmd.len() - 1 };
        let n: &str = cmd[searchAt..].trim();
        let mut n: usize = if n.eq("") { 0 } else {
            match n.parse::<usize>() {
                Ok(n) => n,
                Err(_) => 0,
            }
        };
        let len = state.command_history.len();
        if n == 0 || n > len { n = len; }
        let start = len - n;
        let end = len;
        for i in start..end {
            println!("{} {}", (i+1), match state.command_history.get(i) {
                Some(item) => item,
                None => "",
            });
        } 
        true
    }
}
