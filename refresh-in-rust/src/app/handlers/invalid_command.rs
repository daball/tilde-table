use crate::app::state::{AppState, CommandRoutes};
use crate::shell::handler::Handler;
use crate::app::ui::render::invalid as render;

pub struct InvalidCommandHandler {}

impl Handler for InvalidCommandHandler {
    fn validate(&self, _command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> bool {
        true
    }
    fn handle(&self, _command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> bool {
        println!("{}", render::invalid(cmd.trim()));
        true // continue read-eval-print-loop
    }    
}
