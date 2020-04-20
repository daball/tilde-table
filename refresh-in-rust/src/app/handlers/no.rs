use crate::tui::handler::Handler;
use crate::app::utils::always_true as execute;

fn validate(cmd: &str) -> bool {
    // the validation looks for 0 or more whitespace,
    // so put this command at the start of the chain
    cmd.is_empty()
}

pub const NoHandler: Handler = Handler {
    validate,
    execute, // always true
};
