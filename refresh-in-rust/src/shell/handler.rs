use crate::app::state::{AppState, CommandRoutes};

pub trait Handler {
    fn validate(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> bool; // return false to try the next handler, return true if handler will handle it
    fn handle(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> bool; // return false to exit the loop, return true to read-eval-print-loop
}
