use crate::app::commands::clear::ClearCommand;
use crate::app::commands::exit::ExitCommand;
use crate::app::commands::help::HelpCommand;
use crate::app::commands::history::HistoryCommand;
use crate::app::commands::list::ListCommand;
use crate::app::commands::open::OpenCommand;
use crate::app::commands::sample::SampleCommand;
use crate::app::commands::version::{VersionCommand};
use super::handlers::empty_command::EmptyCommandHandler;
use super::handlers::dispatch_command::DispatchCommandHandler;
use super::handlers::invalid_command::InvalidCommandHandler;
use crate::shell::{command::{Command, CommandConfig}, handler::{Handler, HandlerConfig}};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialOrd)]
#[derive(Ord)]
pub enum CommandHandler {
    EmptyCommand = 1,
    DispatchCommand = 2,
    InvalidCommand = 3,
}

pub struct CommandHandlers {
    pub handlers: HashMap<CommandHandler, Handler>,
}

impl CommandHandlers {
    pub fn create() -> CommandHandlers {
        let mut handlers: HashMap<CommandHandler, Handler> = HashMap::new();
        handlers.insert(CommandHandler::EmptyCommand, EmptyCommandHandler::config());
        handlers.insert(CommandHandler::DispatchCommand, DispatchCommandHandler::config());
        handlers.insert(CommandHandler::InvalidCommand, InvalidCommandHandler::config());
        CommandHandlers {
            handlers
        }
    }
}

pub struct CommandRoutes {
    pub commands: Vec<Command>,
}

impl CommandRoutes {
    pub fn create() -> CommandRoutes {
        let mut commands: Vec<Command> = vec![
            ExitCommand::config(),
            HelpCommand::config(),
            HistoryCommand::config(),
            ListCommand::config(),
            OpenCommand::config(),
            SampleCommand::config(),
            VersionCommand::config(),
        ];
        if crate::features::USE_ANSI {
            commands.append(&mut vec![
                ClearCommand::config()
            ]);
        }
        CommandRoutes {
            commands,
        }
    }
}

pub struct AppState {
    // pub appTerminal: AppTerminal,
    // pub name: String,
    pub command_history: Vec<String>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            command_history: Vec::new(),
            // name: String::new(),
            // appTerminal: AppTerminal::create(),
        }
    }
}

// use std::io;
// use tui::Terminal;
// use tui::widgets::{Widget, Block, Borders};
// use tui::layout::{Layout, Constraint, Direction};

// #[cfg(target_family="unix")]
// use tui::backend::TermionBackend;
// #[cfg(target_family="unix")]
// use termion::raw::{IntoRawMode, RawTerminal};
// #[cfg(target_family="windows")]
// use tui::backend::CrosstermBackend;

// #[cfg(target_family="windows")]
// pub struct AppTerminal {
//     pub terminal: Terminal<TermionBackend<io::Stdout>>,
// }

// #[cfg(target_family="unix")]
// pub struct AppTerminal {
//     pub terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
// }

// #[cfg(target_family="windows")]
// impl AppTerminal {
//     #[cfg(target_family="windows")]
//     pub fn create() -> Terminal<CrosstermBackend<io::Stdout>> {
//         let stdout = io::stdout();
//         let backend = CrosstermBackend::new(stdout);
//         let terminal = Terminal::new(backend);
//         AppTerminal {
//             terminal,
//         }
//     }
// }

// #[cfg(target_family="unix")]
// impl AppTerminal {
//     #[cfg(target_family="unix")]
//     pub fn create() -> AppTerminal {
//         let stdout = io::stdout();
//         let raw_terminal = stdout.into_raw_mode().expect("Failed to acquire standard output in raw mode.");
//         let backend = TermionBackend::new(raw_terminal);
//         let terminal = Terminal::new(backend).expect("Failed to acquire terminal.");
//         AppTerminal {
//             terminal
//         }
//     }
// }
