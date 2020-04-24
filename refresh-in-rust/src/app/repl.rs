#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::app::commands::version;
use super::state::{AppState, CommandRoutes, CommandHandler, CommandHandlers};
use crate::shell::handler::{Handler, HandlerResult, ValidatorResult};
use std::io::{stdin, stdout, Write};
use std::fmt;
use std::iter::FromIterator;

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
    let mut command_routes = CommandRoutes::create();
    let command_handlers = CommandHandlers::create();
    let handlers = &command_handlers.handlers;
    let mut handlers: Vec<&CommandHandler> = Vec::from_iter(handlers.keys());
    handlers.sort();
    let handlers = handlers;
    'repl: loop {
        // read
        let cmd = read();
        for handler in &handlers {
            if cfg!(feature="debug") {
                match handler {
                    CommandHandler::DispatchCommand => {
                        println!("[debug] Trying DispatchCommandHandler.");
                    },
                    CommandHandler::EmptyCommand => {
                        println!("[debug] Trying EmptyCommandHandler.");
                    },
                    CommandHandler::InvalidCommand => {
                        println!("[debug] Trying InvalidCommandHandler.");
                    }
                }
            }
            let handler: &Handler = command_handlers.handlers.get(handler).unwrap();
            match handler.validate(&mut command_routes, &mut app_state, &cmd) {
                ValidatorResult::Valid => {
                    match handler.handle(&mut command_routes, &mut app_state, &cmd) {
                        HandlerResult::ContinueLoop => continue 'repl,
                        HandlerResult::Exit => break 'repl,
                    }
                },
                ValidatorResult::Invalid => {}
            }
        }
    }
    // Ok(())
}
