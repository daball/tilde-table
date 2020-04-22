use crate::shell::handler::Handler;
use crate::app::handlers::utils::always_true as execute;
use crate::app::state::AppState;

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    // the validation looks for 0 or more whitespace,
    // so put this command at the start of the chain
    cmd.is_empty()
}

#[allow(non_upper_case_globals)]
pub const NoHandler: Handler = Handler {
    validate,
    execute, // always true
};
