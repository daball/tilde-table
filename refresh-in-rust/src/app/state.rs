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

pub struct AppState {
    // pub appTerminal: AppTerminal,
    pub name: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            name: String::new(),
            // appTerminal: AppTerminal::create(),
        }
    }
}
