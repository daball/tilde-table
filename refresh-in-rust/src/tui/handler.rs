pub struct Handler {
    pub validate: fn (cmd: &str) -> bool, // return false to try the next handler, return true if handler will handle it
    pub execute: fn (cmd: &str) -> bool, // return false to exit the loop, return true to read-eval-print-loop
}
