use crate::shell::command::{Command, CommandConfig, HandlerResult, ValidatorResult};
use crate::app::state::AppState;

pub struct HistoryCommand { }

impl HistoryCommand {
    pub fn handler(state: &mut AppState, cmd: &str) -> HandlerResult {
        let cmd = cmd.trim();
        let search_at = if cmd.starts_with("history") { 7 } else { cmd.len() - 1 };
        let n: &str = cmd[search_at..].trim();
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
        HandlerResult::ContinueLoop
    }
}

impl CommandConfig for HistoryCommand {
    fn config() -> Command {
        Command::configure("history")
            .param("n").desc("Number of items to display.").optional()
            .short_desc("Displays up to optional number of history items.")
            .category("Basic")
            .handle(HistoryCommand::handler)
            .configured()
    }
}
