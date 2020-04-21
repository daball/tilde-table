pub mod drivers;

pub struct Driver {
    pub validate: fn (path: &str) -> bool, // return false to try the next driver, return true if driver will handle it
    pub exists: fn (path: &str) -> bool,
    pub open: fn (path: &str) -> bool, // return false to exit the loop, return true to read-eval-print-loop
}
