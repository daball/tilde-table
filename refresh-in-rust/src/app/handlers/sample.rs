use crate::tui::handler::Handler;
use crate::app::utils::handle_not_implemented as execute;

fn validate(cmd: &str) -> bool {
    cmd.eq("sample")    
}

pub const SampleHandler: Handler = Handler {
    validate,
    execute,
};
