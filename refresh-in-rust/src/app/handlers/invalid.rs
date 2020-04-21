#[cfg(feature="ansi_term")] extern crate ansi_term;

use crate::app::state::AppState;
use crate::app::handlers::utils::always_true as validate;
use crate::shell::handler::Handler;

fn execute(state: &mut AppState, cmd: &str) -> bool {
    println!("Invalid command {}. Enter {} to list possible commands.", cmd, "help"); // invalid command
    true // continue read-eval-print-loop
}

pub const InvalidHandler: Handler = Handler {
    validate,
    execute,
};
