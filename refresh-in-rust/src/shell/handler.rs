use crate::app::state::AppState;

pub struct Handler {
    pub validate: fn (state: &mut AppState, cmd: &str) -> bool, // return false to try the next handler, return true if handler will handle it
    pub execute: fn (state: &mut AppState, cmd: &str) -> bool, // return false to exit the loop, return true to read-eval-print-loop
}

// pub enum Handler {
// }
