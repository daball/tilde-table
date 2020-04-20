extern crate ansi_term;
pub const PS1: &str = "≈ % ";
pub mod clear;
pub mod dispatch;
pub mod exit;
pub mod help;
pub mod invalid;
pub mod list;
pub mod no;
pub mod sample;
pub mod version;
use ansi_term::Colour;
use crate::app::handlers::dispatch::DispatchHandler;
use crate::app::handlers::version::print_version;
use std::io::{stdin, stdout, Write};

pub fn read() -> String {
    print!("{}", Colour::Cyan.bold().paint(PS1));
    stdout().flush();
    let mut cmd: String = String::from("");
    stdin().read_line(&mut cmd)
        .expect("Failed to read standard input.");
    cmd
}

pub fn repl() {
    print_version();
    loop {
        // read
        let cmd = read();
        // eval/print
        if { let e = DispatchHandler.execute; e(&cmd) } {
            continue; // loop
        } else {
            break; // exit loop
        }
    }
}
