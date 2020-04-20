use crate::tui::handler::Handler;
use std::io::{stdout, Write};

pub fn clear_screen() {
    stdout().write(&[0o033, b'c'][..]);
}

fn validate(cmd: &str) -> bool {
    cmd.eq("cls") || cmd.eq("clear")   
}

fn execute(_cmd: &str) -> bool {
    clear_screen();
    true // continue read-eval-print-loop
}

pub const ClearHandler: Handler = Handler {
    validate,
    execute,
};
