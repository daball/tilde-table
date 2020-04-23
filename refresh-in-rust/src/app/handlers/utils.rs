use crate::app::state::AppState;

pub fn always_false(_state: &mut AppState, _cmd: &str) -> bool {
    false
}

pub fn always_true(_state: &mut AppState, _cmd: &str) -> bool {
    true
}

pub const NOT_IMPLEMENTED: &'static str = "This feature has not been implemented.";
pub const REQUIRES_ANSI: &'static str = "This feature requires the ansi feature to be compiled and enabled.";

pub fn print_not_implemented() {
    println!("{}", String::from(NOT_IMPLEMENTED));
}

pub fn print_not_implemented_requires_ansi() {
    println!("{}", String::from(REQUIRES_ANSI));
}

pub fn handle_not_implemented(_state: &mut AppState, _cmd: &str) -> bool {
    print_not_implemented();
    true
}

pub fn handle_not_implemented_requires_ansi(_state: &mut AppState, _cmd: &str) -> bool {
    print_not_implemented_requires_ansi();
    true
}
