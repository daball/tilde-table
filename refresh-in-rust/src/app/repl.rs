#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::app::handlers::dispatch::DispatchHandler;
use crate::app::handlers::version;
use crate::app::state::AppState;
use std::io::{stdin, stdout, Write};
use std::fmt;

pub const PS1: &str = "≈ % ";

#[cfg(feature="ansi_term")]
pub fn get_prompt() -> String {
    fmt::format(format_args!("{}", Colour::Cyan.bold().paint(PS1)))
}

#[cfg(not(feature="ansi_term"))]
pub fn get_prompt() -> String {
    String::from(PS1)
}

pub fn print_prompt() {
    print!("{}", get_prompt());
}

pub fn read() -> String {
    print_prompt();
    stdout().flush();
    let mut cmd: String = String::from("");
    stdin().read_line(&mut cmd)
        .expect("Failed to read standard input.");
    cmd
}

pub fn repl() { //-> Result<(), Box<dyn std::error::Error>> {
    version::print_version();
    let mut app_state = AppState::new();
    loop {
        // read
        let cmd = read();
        // eval/print
        let eval_print = DispatchHandler.execute;
        let continue_loop = eval_print(&mut app_state, &cmd);
        if continue_loop {
            continue; // loop
        } else {
            break; // exit loop
        }
    }
    // Ok(())
}
