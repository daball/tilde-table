use std::fs;
use std::path::Path;

pub fn open(path: &str) -> std::io::Result<()> {
    open(String::from(path))
}

pub fn open(path: String) -> std::io::Result<()> {
    open(Path::new(path))
}

pub fn open(path: Path) -> std::io::Result<()> {
    fs::File::open(path)
}
