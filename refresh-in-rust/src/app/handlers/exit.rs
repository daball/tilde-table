#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::shell::handler::Handler;
use crate::app::state::AppState;

#[cfg(feature="ansi_term")]
fn print_goodbye() {
    println!("{}", Colour::Blue.bold().paint("Later!"));   
}

fn print_goodbye_noansi() {
    println!("{}", "Later!");   
}

#[cfg(not(feature="ansi_term"))]
const print_goodbye: fn () = print_goodbye_noansi;

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    cmd.eq("exit") || cmd.eq("quit") || cmd.eq("qui") || cmd.eq("qu") || cmd.eq("q")    
}

fn execute(_state: &mut AppState, _cmd: &str) -> bool {
    print_goodbye();
    false // break read-eval-print-loop
}

#[allow(non_upper_case_globals)]
pub const ExitHandler: Handler = Handler {
    validate,
    execute,
};
