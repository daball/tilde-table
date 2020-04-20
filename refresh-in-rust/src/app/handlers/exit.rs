extern crate ansi_term;
use ansi_term::Colour;
use crate::tui::handler::Handler;

fn print_goodbye() {
    println!("{}", Colour::Blue.bold().paint("Later!"));   
}

fn validate(cmd: &str) -> bool {
    cmd.eq("exit") || cmd.eq("quit") || cmd.eq("qui") || cmd.eq("qu") || cmd.eq("q")    
}

fn execute(_cmd: &str) -> bool {
    print_goodbye();
    false // break read-eval-print-loop
}

pub const ExitHandler: Handler = Handler {
    validate,
    execute,
};
