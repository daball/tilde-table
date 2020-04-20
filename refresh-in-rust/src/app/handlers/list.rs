use crate::tui::handler::Handler;
// use std::io::Write;

pub const ListHandler: Handler = Handler {
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
