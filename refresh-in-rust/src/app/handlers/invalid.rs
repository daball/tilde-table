extern crate ansi_term;

use crate::app::utils::always_true as validate;
use crate::tui::handler::Handler;

fn execute(cmd: &str) -> bool {
    println!("Invalid command {}. Enter {} to list possible commands.", cmd, "help"); // invalid command
    true // continue read-eval-print-loop
}

pub const InvalidHandler: Handler = Handler {
    validate,
    execute,
};
