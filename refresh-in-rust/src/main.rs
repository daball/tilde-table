
// use std::cmp::Eq;
// use std::io;
// use std::fs::{self, DirEntry, ReadDir};
// use std::path::Path;
mod tui;
mod app;
// use crate::tui::command::Command;
use app::handlers::repl;

fn main() {
    repl();
}
