// extern crate ansi_term;
// pub const PS1: &str = "≈ % ";
pub mod dispatch_command;
pub mod invalid_command;
pub mod empty_command;
pub mod utils;
// use ansi_term::Colour;
// use crate::app::handlers::dispatch::DispatchHandler;
// use crate::app::handlers::version::print_version;
// use crate::app::state::AppState;
// use std::fmt;
// use std::io::{self, stdin, stdout, BufRead, Read, Write};
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

// use crossterm::{
//     execute,
//     style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//     ExecutableCommand, Result,
//     event,
// };

// when I press the up arrow, the console says ^[[A or [27, 91, 65]
// when I press the down arrow, the console says ^[[B or [27, 91, 66]
// when I press the right arrow, the console says ^[[C or [27, 91, 67]
// when I press the left arrow, the console says ^[[D or [27, 91, 68]

// struct CommandLine {
//     prompt: String,
//     cursor_pos: (u8, u8),
//     line: String,
//     buffer: Vec<u8>,
//     bytes_read: usize,
// }

// impl CommandLine {
//     pub fn new(prompt: &str) -> CommandLine {
//         CommandLine {
//             prompt: String::from(prompt),
//             cursor_pos: (0, 0),
//             line: String::default(),
//             buffer: Vec::new(),
//             bytes_read: 0,
//         }
//     }
    // pub fn read(&mut self) -> String {
        // print!("{}", Colour::Cyan.bold().paint(PS1));
        // stdout().flush();
        // let stdin = stdin();    
        // let mut stdin = stdin.lock();
        // let mut buf = stdin.fill_buf()
        //     .expect("Failed to read standard input.").to_vec();
        // self.buffer.append(&mut buf);
        // println!("{:?}", self.buffer);
        // self.bytes_read = self.buffer.len();
        // stdin.consume(self.bytes_read);
        // self.line += &fmt::format(format_args!("{} bytes <== {:?}", self.bytes_read, self.buffer));
        // self.bytes_read = stdin().lock().read_until(b'\n', &mut self.buffer)
        //     .expect("Failed to read standard input.");
        // std::io::default_read_vectored(read: F, bufs: &mut [IoSliceMut<'_>])
        // let mut cmd: String = String::from("");
        // let mut buf: 
        // let bytes_read = std::io::Stdin.read(&mut self.buffer)
            // .expect("Failed to read standard input.");
        // let bytes_read = stdin().read(&mut self.buffer)
        //     .expect("Failed to read standard input.");
        // let string_read = String::from_raw_parts(self.buffer, bytes_read, bytes_read);
        // self.line += &fmt::format(format_args!("{} bytes <== {}", bytes_read, string_read));
        // println!("Read input {}", &self.line);
        // stdin().read_line(&mut cmd)
        //     .expect("Failed to read standard input.");
//         String::from(&self.line)
//     }
// }

// #[cfg(target_family="windows")]
// pub fn read() {
//     // execute!(
//     //     stdout(),
//     //     SetForegroundColor(Color::Cyan),
//     //     Print(PS1),
//     //     ResetColor
//     // );
//     // print!("{}", Colour::Cyan.bold().paint(PS1));
//     // stdout().flush();
// }

// #[cfg(target_family="unix")]
// pub fn read() {
    
// }
