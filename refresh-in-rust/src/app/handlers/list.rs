#[cfg(feature="ansi_term")] extern crate ansi_term;
extern crate dirs;
#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::shell::handler::Handler;
use crate::app::state::AppState;
use std::path::{Path, PathBuf};
use std::fmt;
// use std::io::Write;

#[derive(PartialEq)]
enum FsItemType {
    Nothing,
    Dir,
    File,
    SomethingElse,
}

struct FsItem {
    r#type: FsItemType,
    path: PathBuf,
    canonical_path: Option<PathBuf>,
    exists: bool,
    is_symlink: bool,
    name: Option<String>,
    extension: Option<String>,
    enumerated_items: Option<Vec<FsItem>>,
}

// fn get_home() -> String {
//     dirs::get_home().expect("Could not get home directory.")
// }

impl FsItem {
    fn print_search_path_noansi(&self) {
        let s: String = match &self.canonical_path {
            Some(canonical_path) => fmt::format(format_args!("Search path: {} -> {}",
                self.path.as_os_str().to_str().unwrap_or("invalid path!"),
                canonical_path.as_os_str().to_str().unwrap_or("invalid path!")
            )),
            None => fmt::format(format_args!("Search path: {}",
                self.path.as_os_str().to_str().unwrap_or("invalid path!")
            )),
        };
        println!("{}", s);
    }

    #[cfg(feature="ansi_term")]
    fn print_search_path(&self) {
        let s: String = match &self.canonical_path {
            Some(canonical_path) => fmt::format(format_args!("Search path: {} -> {}",
                Colour::Red.bold().paint(self.path.as_os_str().to_str().unwrap_or("invalid path!")),
                Colour::Red.bold().paint(canonical_path.as_os_str().to_str().unwrap_or("invalid path!"))
            )),
            None => fmt::format(format_args!("Search path: {}",
                Colour::Red.bold().paint(self.path.as_os_str().to_str().unwrap_or("invalid path!"))
            )),
        };
        println!("{}", s);
    }

    #[cfg(not(feature="ansi_term"))]
    fn print_search_path(&self) {
        self.print_search_path_noansi();
    }
    
    fn print_item_noansi(&self) {
        if self.is_symlink {
            let r#type = match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => String::from("<SYM_D>"),
                FsItemType::File => match &self.extension {
                    Some(extension) => fmt::format(format_args!("<SYM>:{}", extension.to_uppercase())),
                    None => String::default(),
                },
                FsItemType::SomethingElse => String::default(),
            };
            println!(" {}  {} -> {}",
                r#type,
                match &self.name {
                    Some(name) => name.as_str(),
                    None => "no-name!",
                },
                match &self.canonical_path {
                    Some(canonical_path) => canonical_path.to_str().unwrap_or(""),
                    None => "?",
                }
            );
        } else {
            let r#type: String = match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => String::from("<DIR>"),
                FsItemType::File => match &self.extension {
                    Some(extension) => fmt::format(format_args!("{}", extension.to_uppercase())),
                    None => String::default(),
                },
                FsItemType::SomethingElse => String::default(),
            };
            println!(" {}  {}", r#type, match &self.name {
                Some(name) => name.as_str(),
                None => "no-name!",
            });
        }
    }

    #[cfg(feature="ansi_term")]
    fn print_item(&self) {
        if self.is_symlink {
            let r#type: String = match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => Colour::Yellow.bold().paint("<SYM_D>").to_string(),
                FsItemType::File => match &self.extension {
                    Some(extension) => fmt::format(format_args!("<SYM>:{}", Colour::Cyan.bold().paint(extension.to_uppercase()).to_string())),
                    None => String::default(),
                },
                FsItemType::SomethingElse => String::default(),
            };
            println!(" {}  {} -> {}",
                r#type,
                match &self.name {
                    Some(name) => name.as_str(),
                    None => "no-name!",
                },
                match &self.canonical_path {
                    Some(canonical_path) => canonical_path.to_str().unwrap_or(""),
                    None => "?",
                }
            );
        } else {
            let r#type: String = match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => Colour::Green.bold().paint("<DIR>").to_string(),
                FsItemType::File => match &self.extension {
                    Some(extension) => fmt::format(format_args!("{}", ansi_term::Colour::Blue.bold().paint(extension.to_uppercase()).to_string())),
                    None => String::default(),
                }
                FsItemType::SomethingElse => String::default(),
            };
            println!(" {}  {}",
                r#type,
                match &self.name {
                    Some(name) => name.as_str(),
                    None => "no-name!",
                }
            );
        }
    }
    
    #[cfg(not(feature="ansi_term"))]
    fn print_item(&self) {
        self.print_item_noansi();
    }
    
    fn print_items(&self) {
        match &self.enumerated_items {
            Some(enumerated_items) => {
                // print directories first
                for item in enumerated_items {
                    if item.r#type == FsItemType::Dir {
                        item.print_item();
                    }
                }
                // print files and symlinks next
                for item in enumerated_items {
                    if item.r#type == FsItemType::File {
                        item.print_item();
                    }
                }
            },
            None => {},
        }
    }
    
    fn from_path_with_depth(path: &Path, depth: usize) -> FsItem {
        let path: PathBuf = PathBuf::from(Path::new(path));
        let exists: bool = path.exists();
        let name: Option<String> = match path.file_name() {
            Some(name) => Some(String::from(name.to_str().unwrap_or("no-name!"))),
            None => None,
        };
        let canonical_path: Option<PathBuf> = match path.canonicalize() {
            Ok(canonical_path) => Some(canonical_path),
            Err(_e) => None,
        };
        let r#type: FsItemType = if !exists {
            FsItemType::Nothing
        } else {
            if path.is_file() {
                FsItemType::File
            }
            else if path.is_dir() {
                FsItemType::Dir
            }
            else {
                FsItemType::SomethingElse
            }
        };
        let is_symlink: bool = match path.symlink_metadata() {
            Ok(metadata) => metadata.file_type().is_symlink(),
            Err(_e) => false,
        };
        let extension: Option<String> = match path.extension() {
            Some(extension) => Some(String::from(extension.to_str().unwrap_or(""))),
            None => None,
        };
        let enumerated_items: Option<Vec<FsItem>> = if depth > 0 && r#type == FsItemType::Dir {
            let depth: usize = depth - 1;
            let mut v_fsi: Vec<FsItem> = Vec::new();
            match std::fs::read_dir(&path) {
                Ok(dir) => {
                    for entry in dir {
                        match entry {
                            Ok(entry) => {
                                let fsi_entry: FsItem = FsItem::from_path_with_depth(entry.path().as_path(), depth);
                                v_fsi.append(&mut vec![fsi_entry]);
                            },
                            Err(_e) => {}
                        }
                    }
                },
                Err(_e) => {}
            }
            Some(v_fsi)
        } else {
            None
        };
        FsItem {
            r#type,
            path,
            canonical_path,
            exists,
            is_symlink,
            name,
            extension,
            enumerated_items,
        }
    }

    fn from_path(path: &Path) -> FsItem {
        FsItem::from_path_with_depth(path, 1)
    }
    
    fn with_depth(path: &str, depth: usize) -> FsItem {
        FsItem::from_path_with_depth(PathBuf::from(path).as_path(), depth)
    }
    
    fn from(path: &str) -> FsItem {
        FsItem::with_depth(path, 1)
    }
}

// fn print_dirs(path: &str) {
//     let q = std::fs::read_dir(path);
//     if q.is_err() {
//         println!("{}", q.err().unwrap());
//     } else {
//         let dir: std::fs::ReadDir = q.ok().unwrap();
//         for entry in dir { // Lap 1: <DIR>
//             if entry.is_err() {
//                 println!("{}", entry.err().unwrap());
//             } else {
//                 let e = entry.ok().unwrap();
//                 let etype = e.file_type();
//                 let name = String::from(e.file_name().to_str().unwrap());
//                 if etype.is_err() {
//                     println!("{}", etype.err().unwrap());
//                 } else if !name.starts_with(".") {
//                     let etype = etype.ok().unwrap();
//                     if etype.is_dir() {
//                         print_dir(name);
//                     }
//                 }
//             }
//         }
//     }
// }

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    cmd.starts_with("list") || cmd.starts_with("ls") || cmd.starts_with("dir")
}

fn execute(_state: &mut AppState, cmd: &str) -> bool {
    let path = cmd.trim();
    let searchAt = if cmd.starts_with("list") { 4 } else if cmd.starts_with("dir") { 3 } else if cmd.starts_with("ls") { 2 } else { 0 };
    let path = path[searchAt..].trim();
    let path = if path.eq("") { "." } else { path };
    let fsi = FsItem::from(path);
    fsi.print_search_path();
    match fsi.r#type {
        FsItemType::Nothing => {
            println!("nothing! {}", fsi.path.as_os_str().to_str().unwrap_or("no path!"));
        },
        FsItemType::File => {
            fsi.print_item();
        },
        FsItemType::Dir => {
            fsi.print_items();
        },
        FsItemType::SomethingElse => {
            println!("something else! {}", fsi.path.as_os_str().to_str().unwrap_or("no path!"));
        },
    }
    // fsi.print_items();
    // print_search_path(&path);
    // print_dirs(&path);
    // let q = std::fs::read_dir(path);
    // if q.is_err() {
    //     println!("{}", q.err().unwrap());
    // } else {
    //     let dir: std::fs::ReadDir = q.ok().unwrap();
    //     for entry in dir { // Lap 2: everything else
    //         if entry.is_err() {
    //             println!("{}", entry.err().unwrap());
    //         } else {
    //             let e = entry.ok().unwrap();
    //             let etype = e.file_type();
    //             let name = String::from(e.file_name().to_str().unwrap());
    //             if etype.is_err() {
    //                 println!("{}", etype.err().unwrap());
    //             } else if !name.starts_with(".") {
    //                 let etype = etype.ok().unwrap();
    //                 if etype.is_symlink() {
    //                     print_symlink(&name);
    //                 } else if etype.is_file() {
    //                     if name.to_lowercase().ends_with(".txt") {
    //                         println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("TXT"), name);
    //                     } else if name.to_lowercase().ends_with(".csv") {
    //                         println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("CSV"), name);
    //                     } else if name.to_lowercase().ends_with(".tsv") {
    //                         println!("  {}   {}", ansi_term::Colour::Blue.bold().paint("TSV"), name);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    true // continue read-eval-print-loop
}

pub const ListHandler: Handler = Handler {
    validate,
    execute,
};
