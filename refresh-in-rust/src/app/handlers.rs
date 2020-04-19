extern crate ansi_term;

use crate::tui::handler::Handler;
use ansi_term::Colour;
use ansi_term::Style;
use std::io::Write;

const NoCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.is_empty() // the validation looks for 0 or more whitespace, put it at the start of the chain
    },
    execute: |cmd: &str| -> bool {
        true // continue read-eval-print-loop
    },
};

const InvalidCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        true // the validation for this command works no matter what, put it at the end of the chain
    },
    execute: |cmd: &str| -> bool {
        println!("Invalid command. Enter \"help\" to list possible commands."); // invalid command
        true // continue read-eval-print-loop
    },
};

const ClearCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.eq("cls") || cmd.eq("clear")
    },
    execute: |cmd: &str| -> bool {
        std::io::stdout().write(&[0o033, b'c'][..]);
        true // continue read-eval-print-loop
    },
};

const ExitCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.eq("exit") || cmd.eq("quit") || cmd.eq("qui") || cmd.eq("qu") || cmd.eq("q")
    },
    execute: |cmd: &str| -> bool {
        println!("Goodbye!");
        false // break read-eval-print-loop
    },
};

const VersionCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.eq("ver") || cmd.eq("version")
    },
    execute: |cmd: &str| -> bool {
        println!("{}", Colour::Blue.bold().underline().paint("Tilde Tabulator Refresh Project by David Ball"));
        println!();
        true // continue read-eval-print-loop
    },
};

const HelpCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.eq("?") || cmd.eq("help")
    },
    execute: |cmd: &str| -> bool {
        { let e = VersionCommand.execute; e("ver"); }
        println!("About this application:");
        println!("  Processes spreadsheet data files.");
        println!("  To use this application, type a command using the syntax below.");
        println!();
        println!("Basic commands:");
        println!("  exit|quit      Exits/quits the application.");
        println!("  ?|help         Prints this help page.");
        println!("  cls|clear      Clears the screen.");
        println!("  sample         Runs the sample routine.");
        println!("  ver|version    Prints version information page.");
        println!("Relation commands:");
        println!("  ls [path]      Lists all valid (*.txt) files to read at optional path.");
        println!("  open [path]    Opens file at required path to a named relation.");
        println!("  show           Lists all loaded relations by name.");
        println!("  describe [rel] Describes a relation, including name, path, and filter chain.");
        println!("  rn [rel] [new] Renames a relation.");
        println!("  filter [rel]   Applies filters on a relationn as a separate relation.");
        println!("  project [rel]  Prints the contents of the relation.");
        println!("  close [rel]    Unloads a relation.");
        println!();
        true // continue read-eval-print-loop
    },
};

const SampleCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.eq("sample")
    },
    execute: |cmd: &str| -> bool {
        println!("NOT IMPLEMENTED!");
        true // continue read-eval-print-loop
    },
};

const ListCommand: Handler = Handler {
    validate: |cmd: &str| -> bool {
        cmd.starts_with("list") || cmd.starts_with("ls") || cmd.starts_with("dir")
    },
    execute: |cmd: &str| -> bool {
        let path = cmd.trim();
        let searchAt = if cmd.starts_with("list") { 4 } else if cmd.starts_with("dir") { 3 } else if cmd.starts_with("ls") { 2 } else { 0 };
        let path = path[searchAt..].trim();
        let path = if path.eq("") { "." } else { path };
        println!("Search path: {}", ansi_term::Colour::Red.bold().paint(path));
        let q = std::fs::read_dir(path);
        if q.is_err() {
            println!("{}", q.err().unwrap());
        } else {
            let dir: std::fs::ReadDir = q.ok().unwrap();
            for entry in dir { // Lap 1: <DIR>
                if entry.is_err() {
                    println!("{}", entry.err().unwrap());
                } else {
                    let e = entry.ok().unwrap();
                    let etype = e.file_type();
                    let name = String::from(e.file_name().to_str().unwrap());
                    if etype.is_err() {
                        println!("{}", etype.err().unwrap());
                    } else if !name.starts_with(".") {
                        let etype = etype.ok().unwrap();
                        if etype.is_dir() {
                            println!(" {}  {}", ansi_term::Colour::Green.bold().paint("<DIR>"), name);
                        }
                    }
                }
            }
        }
        let q = std::fs::read_dir(path);
        if q.is_err() {
            println!("{}", q.err().unwrap());
        } else {
            let dir: std::fs::ReadDir = q.ok().unwrap();
            for entry in dir { // Lap 2: everything else
                if entry.is_err() {
                    println!("{}", entry.err().unwrap());
                } else {
                    let e = entry.ok().unwrap();
                    let etype = e.file_type();
                    let name = String::from(e.file_name().to_str().unwrap());
                    if etype.is_err() {
                        println!("{}", etype.err().unwrap());
                    } else if !name.starts_with(".") {
                        let etype = etype.ok().unwrap();
                        if etype.is_symlink() {
                            println!(" {}  {}", ansi_term::Colour::Cyan.bold().paint("<SYM>"), name);
                        } else if etype.is_file() {
                            if name.to_lowercase().ends_with(".txt") {
                                println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("TXT"), name);
                            } else if name.to_lowercase().ends_with(".csv") {
                                println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("CSV"), name);
                            } else if name.to_lowercase().ends_with(".tsv") {
                                println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("TSV"), name);
                            }
                        }
                    }
                }
            }
        }
        true // continue read-eval-print-loop
    },
};

// fn list_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> bool {
//     println!("NOT IMPLEMENTED!");
//     // if dir.is_dir() {
//     //     for entry in fs::read_dir(dir) {
//     //         let entry = entry.?;
//     //         let path = Ok(entry).ok().unwrap().;
//     //         if (path.is_dir()) {
//     //             println!("<dir> {}", path);
//     //         } else {
                
//     //         }
//     //     }
//     // }
//     true
// }

// fn print_file_list(path: String) -> bool {
//     println!("NOT IMPLEMENTED!");
//     let path = path.trim();
//     let path = if path.eq("") { "." } else { path };
//     let q = fs::read_dir(path);
//     if q.is_err() {
//         let err = q.unwrap_err();
//         eprintln!("An error has occurred.");
//         eprintln!("{}", err);
//     } else {
//         // let entries = Ok(q).unwrap().unwrap();
//         // for entry in entries {
//         //     entry
//         // }
//         // let dirs = dir.map(|e| e.);
//         // let dir = q;
//         // let mut dirs = q?.map(|res| res.map(|e| e.path()).collect::<Result<Vec<_>, io::Error>>();
//         // let files = q.ok().map(|entry| !entry.is_dir() && );
//     }
//     // if path.is_dir
//     // let mut entries = fs::read_dir(path)?
//     //     .map(|res| res.map(|e| e.path()))
//     //     .collect::<Result<Vec<_>, io::Error>>()?;
//     // entries.sort();
//     // for entry in std::fs::read_dir(path) {
        
//     // }
//     true
// }

// fn print_invalid_command(cmd: String) -> bool {
//     println!("Invalid command. Enter \"help\" to list possible commands.");
//     true
// }

fn dispatch_cmd(cmd: &str) -> bool {
    let cmd = cmd.trim();
    // #![feature(str_strip)]
    return if { let v = NoCommand.validate; v(cmd) } {
        let e = NoCommand.execute; e(cmd)
    } else if { let v = ExitCommand.validate; v(cmd) } {
        let e = ExitCommand.execute; e(cmd)
    } else if { let v = ClearCommand.validate; v(cmd) } {
        let e = ClearCommand.execute; e(cmd)
    } else if { let v = HelpCommand.validate; v(cmd) } {
        let e = HelpCommand.execute; e(cmd)
    } else if { let v = SampleCommand.validate; v(cmd) } {
        let e = SampleCommand.execute; e(cmd)
    } else if { let v = VersionCommand.validate; v(cmd) } {
        let e = VersionCommand.execute; e(cmd)
    } else if { let v = ListCommand.validate; v(cmd) } {
        let e = ListCommand.execute; e(cmd)
    // } else if cmd.starts_with("list") {
        //     return print_file_list(path);
        //     let path = cmd.strip_prefix("list").or(Some("")).unwrap().trim();
    // } else if cmd.starts_with("ls") {
    //     let path = cmd.strip_prefix("ls").trim();
    //     return print_file_list(path);
    // } else if cmd.starts_with("dir") {
    //     let path = cmd.strip_prefix("dir").trim();
    //     return print_file_list(path);
    // } else if cmd.starts_with("open") {
    //     let path = cmd.strip_prefix("open").trim();
    //     return load_relation_from_file(path);
    // } else if cmd.starts_with("load") {
    //     let path = cmd.strip_prefix("load").trim();
    //     return load_relation_from_file(path);
    // }
    } else if { let v = InvalidCommand.validate; v(cmd) } {
        let e = InvalidCommand.execute; e(cmd)
    } else { false }
}

pub fn repl() {
    const PS1: &str = "≈ % ";
    { let e = VersionCommand.execute; e("ver") };
    loop {
        // prompt
        print!("{}", Colour::Cyan.bold().paint(PS1));
        std::io::stdout().flush();
        // read
        let mut cmd: String = String::from("");
        std::io::stdin().read_line(&mut cmd)
            .expect("Failed to read standard input.");
        // eval/print
        if dispatch_cmd(&cmd) {
            continue; // loop
        } else {
            break; // exit loop
        }
    }
}
