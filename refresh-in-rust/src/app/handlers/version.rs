extern crate ansi_term;
use ansi_term::Colour;
use crate::tui::handler::Handler;

pub fn print_version() {
    println!("{}", Colour::Blue.bold().underline().paint("Tilde Tabulator Refresh Project by David Ball"));
    println!();
}

fn validate(cmd: &str) -> bool {
    cmd.eq("ver") || cmd.eq("version")
}

fn execute(cmd: &str) -> bool {
    print_version();
    true // continue read-eval-print-loop
}

pub const VersionHandler: Handler = Handler {
    validate,
    execute,
};
