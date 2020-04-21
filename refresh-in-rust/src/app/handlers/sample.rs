use crate::shell::handler::Handler;
use crate::app::handlers::utils::handle_not_implemented as execute;
use crate::app::state::AppState;

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    cmd.eq("sample")    
}

pub const SampleHandler: Handler = Handler {
    validate,
    execute,
};
