use crate::shell::command::Command;
use crate::shell::handler::Handler;
use crate::app::state::{AppState, CommandRoutes};

pub struct DispatchCommandHandler {}

impl Handler for DispatchCommandHandler {
    fn validate(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> bool {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            let command: &dyn Command = command.as_ref(); 
            if command.validate(state, cmd) {
                return true
            }
        }
        false
    }
    
    fn handle(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> bool {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            let command: &dyn Command = command.as_ref(); 
            if command.validate(state, cmd) {
                return command.execute(state, cmd)
            }
        }
        false
    }
}
