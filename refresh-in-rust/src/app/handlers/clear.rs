use crate::shell::handler::Handler;
use crate::app::state::AppState;
use std::io::{stdout, Write};

#[cfg(feature="ansi_term")]
pub fn clear_screen() {
    stdout().write(&[0o033, b'c'][..]);
}

#[cfg(not(feature="ansi_term"))]
pub use crate::app::handlers::utils::print_not_implemented_requires_ansi as clear_screen;

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    cmd.eq("cls") || cmd.eq("clear")   
}

fn execute(_state: &mut AppState, _cmd: &str) -> bool {
    clear_screen();
    true // continue read-eval-print-loop
}

#[allow(non_upper_case_globals)]
pub const ClearHandler: Handler = Handler {
    validate,
    execute,
};
