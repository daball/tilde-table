use crate::app::state::AppState;

pub fn always_false(_state: &mut AppState, _cmd: &str) -> bool {
    false
}

pub fn always_true(_state: &mut AppState, _cmd: &str) -> bool {
    true
}

pub fn print_not_implemented() {
    println!("This feature has not been implemented.");
}

pub fn print_not_implemented_requires_ansi() {
    println!("This feature requires the ansi feature to be compiled and enabled.");
}

pub fn handle_not_implemented(_state: &mut AppState, _cmd: &str) -> bool {
    print_not_implemented();
    true
}

pub fn handle_not_implemented_requires_ansi(_state: &mut AppState, _cmd: &str) -> bool {
    print_not_implemented_requires_ansi();
    true
}
