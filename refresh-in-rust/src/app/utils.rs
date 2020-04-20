pub fn always_false(_cmd: &str) -> bool {
    false
}

pub fn always_true(_cmd: &str) -> bool {
    true
}

pub fn print_not_implemented() {
    println!("This feature has not yet been implemented!");
}

pub fn handle_not_implemented(_cmd: &str) -> bool {
    print_not_implemented();
    true
}
