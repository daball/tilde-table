use crate::shell::command::{Command, CommandConfig, HandlerResult};
use crate::app::handlers::utils::print_not_implemented;
use crate::app::state::AppState;
use crate::io::drivers::{file, Driver, DriverOptionParam, ReadDriver, Reader, ReaderContext};
use crate::app::ui::render::open;
use std::io::Write;

pub struct OpenCommand { }

impl OpenCommand {
    pub fn handler(_state: &mut AppState, cmd: &str) -> HandlerResult {
        let path = cmd.trim();
        let search_at = 4;
        let path = path[search_at..].trim();
        // let path = if path.eq("") { "." } else { path };
        if path.eq("") {
            println!("{}", open::no_path());
        } else {
            let path_buf = std::path::PathBuf::from(path);
            if !path_buf.exists() {
                println!("{}", open::non_existing_path(path));
            } else if !path_buf.is_file() {
                println!("{}", open::not_a_file_path(path));
            } else {
                let path_opt = DriverOptionParam {
                    name: String::from("path"),
                    value: String::from(path),
                };
                match file::FileDriver::reader().open(vec![path_opt]) {
                    Ok(mut file_reader_context) => {
                        // file_reader_context.is_open();
                        match file_reader_context.bytes() {
                            Ok(bytes_iterator) => {
                                let mut bytes_read: Vec<u8> = Vec::new();
                                for byte in bytes_iterator {
                                    match byte {
                                        Ok(byte) => {
                                            bytes_read.append(&mut vec![byte]);
                                        },
                                        Err(e) => {
                                            eprintln!("{}", "An error has occurred trying to read the file.");
                                            eprintln!("{}", e);            
                                        }
                                    }
                                }
                                println!("File contents:");
                                std::io::stdout().write(&bytes_read[..]);
                            },
                            Err(e) => {
                                eprintln!("{}", "An error has occurred trying to read the file.");
                                eprintln!("{}", e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", "An error has occurred trying to read the file.");
                        eprintln!("{}", e);
                    }
                }
                for opt in file::FileDriver::options() {   
                    println!("{}", opt.name);
                }
            }
        }
        print_not_implemented();
        HandlerResult::ContinueLoop
    }
}

impl CommandConfig for OpenCommand {
    fn config() -> Command {
        Command::configure("open")
            .short_desc("Runs the sample routine.")
            .category("Basic")
            .handle(OpenCommand::handler)
            .configured()
    }
}
