#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use ansi_term::Colour;
extern crate dirs;
use crate::shell::handler::Handler;
use crate::app::state::AppState;
use crate::app::ui::render::list as render;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::fmt;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
enum FsItemType {
    Nothing = 0,
    Dir = 1,
    File = 2,
    SomethingElse = 3,
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Ord)]
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

impl PartialOrd for FsItem {
    fn lt(&self, other: &Self) -> bool {
        let names: (String, String) = (
            match &self.name {
                Some(name) => name.to_string(),
                None => String::default(),
            },
            match &other.name {
                Some(name) => name.to_string(),
                None => String::default(),
            }
        );
        self.r#type.cmp(&other.r#type) == std::cmp::Ordering::Less ||
            names.0.cmp(&names.1) == std::cmp::Ordering::Less ||
            self.path.cmp(&other.path) == std::cmp::Ordering::Less
    }
    fn gt(&self, other: &Self) -> bool {
        let names: (String, String) = (
            match &self.name {
                Some(name) => name.to_string(),
                None => String::default(),
            },
            match &other.name {
                Some(name) => name.to_string(),
                None => String::default(),
            }
        );
        self.r#type.cmp(&other.r#type) == std::cmp::Ordering::Greater ||
            names.0.cmp(&names.1) == std::cmp::Ordering::Greater ||
            self.path.cmp(&other.path) == std::cmp::Ordering::Greater
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) {
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

// fn get_home() -> String {
//     dirs::get_home().unwrap_or(String::new("."))
// }

impl FsItem {
    fn max_info_width(&self) -> usize {
        match &self.enumerated_items {
            None => self.info_area_noansi().len(),
            Some(enumerated_items) => {
                let mut max_found: usize = 0;
                for item in enumerated_items {
                    max_found = std::cmp::max(max_found, item.max_info_width());           
                }
                max_found
            }
        }
    }
    
    fn pad_info_area(&self, max_info_width: usize) -> String {
        " ".repeat(max_info_width - self.info_area_noansi().len())
    }

    fn info_area_noansi(&self) -> String {
        if self.is_symlink {
            match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => render::symlink::dir::info::from_noansi(),
                FsItemType::File => {
                    let extension: String = match &self.extension {
                        Some(extension) => extension.to_string(),
                        None => String::from("file"),
                    };
                    render::symlink::file::info::from_noansi(&extension)
                },
                FsItemType::SomethingElse => String::default()
            }
        } else {
            match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => render::dir::info::from_noansi(),
                FsItemType::File => {
                    let extension: String = match &self.extension {
                        Some(extension) => extension.to_string(),
                        None => String::from("file"),
                    };
                    render::file::info::from_noansi(&extension)
                },
                FsItemType::SomethingElse => String::default()
            }
        }
    }

    fn info_area(&self) -> String {
        if self.is_symlink {
            match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => render::symlink::dir::info::from(),
                FsItemType::File => {
                    let extension: String = match &self.extension {
                        Some(extension) => extension.to_string(),
                        None => String::from("file"),
                    };
                    render::symlink::file::info::from(&extension)
                },
                FsItemType::SomethingElse => String::default()
            }
        } else {
            match self.r#type {
                FsItemType::Nothing => String::default(),
                FsItemType::Dir => render::dir::info::from(),
                FsItemType::File => {
                    let extension: String = match &self.extension {
                        Some(extension) => extension.to_string(),
                        None => String::from("file"),
                    };
                    render::file::info::from(&extension)
                },
                FsItemType::SomethingElse => String::default()
            }
        }
    }
    
    fn name_area(&self) -> String {
        let name = match &self.name {
            Some(name) => name.to_string(),
            None => String::default(),
        };
        if self.is_symlink {
            let canonical_path = match &self.canonical_path {
                Some(canonical_path) => String::from(canonical_path.to_str().unwrap_or_default()),
                None => String::default(),
            };
            match self.r#type {
                FsItemType::Nothing => name,
                FsItemType::Dir => render::symlink::to_dir::name::from(&name, &canonical_path),
                FsItemType::File => render::symlink::to_file::name::from(&name, &canonical_path),
                FsItemType::SomethingElse => name,
            }
        } else {
            match self.r#type {
                FsItemType::Nothing => name,
                FsItemType::Dir => render::dir::name::from(&name),
                FsItemType::File => render::file::name::from(&name),
                FsItemType::SomethingElse => name,
            }
        }
    }
    
    fn print_search_path(&self) {
        let path: &str = self.path.as_os_str().to_str().unwrap_or_default();
        println!("{}", match &self.canonical_path {
            Some(canonical_path) => {
                let canonical_path: &str = canonical_path.as_os_str().to_str().unwrap_or_default();
                if path.eq(canonical_path) {
                    render::search_path::from(path)
                } else {
                    render::search_path::with_canonical_path(path, canonical_path)
                }
            },
            None => render::search_path::from(path),
        });
    }

    fn item_line(&self, max_info_width: usize) -> String {
        let name = self.name_area();
        let pad_info = self.pad_info_area(max_info_width);
        let info = self.info_area();
        format!("{}{} {}", pad_info, info, name)
    }

    fn items_lines(&self) -> Vec<String> {
        let max_info_width: usize = self.max_info_width();
        let mut v_lines: Vec<String> = Vec::new();
        match &self.enumerated_items {
            Some(enumerated_items) => {
                // print directories first
                for item in enumerated_items {
                    if item.r#type == FsItemType::Dir {
                        v_lines.append(&mut vec![item.item_line(max_info_width)]);
                    }
                }
                // print files and symlinks next
                for item in enumerated_items {
                    if item.r#type == FsItemType::File {
                        v_lines.append(&mut vec![item.item_line(max_info_width)]);
                    }
                }
            },
            None => {},
        }
        v_lines
    }

    fn print_item(&self) {
        let max_info_width: usize = self.max_info_width();
        println!("{}", self.item_line(max_info_width));
    }
    
    fn print_items(&self) {
        let v_lines = self.items_lines();
        for line in &v_lines {
            println!("{}", line);
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
            };
            v_fsi.sort();
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
            println!("Invalid file or directory. {} does not exist.", path);
            // println!("nothing! {}", fsi.path.as_os_str().to_str().unwrap_or("no path!"));
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
    true // continue read-eval-print-loop
}

pub const ListHandler: Handler = Handler {
    validate,
    execute,
};
