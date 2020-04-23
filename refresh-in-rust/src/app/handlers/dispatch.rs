use crate::shell::handler::Handler;

use crate::app::state::AppState;
use crate::shell::command::Command;
// use crate::app::handlers::clear::ClearHandler;
use crate::app::commands::clear::ClearCommand;
// use crate::app::handlers::exit::ExitHandler;
use crate::app::commands::exit::ExitCommand;
use crate::app::handlers::help::HelpHandler;
use crate::app::handlers::invalid::InvalidHandler;
use crate::app::handlers::list::ListHandler;
use crate::app::handlers::no::NoHandler;
use crate::app::handlers::sample::SampleHandler;
use crate::app::handlers::version::VersionHandler;

use crate::app::handlers::utils::always_true as validate;

pub fn dispatch(state: &mut AppState, cmd: &str) -> bool {
    let clear_command: ClearCommand = ClearCommand {};
    let exit_command: ExitCommand = ExitCommand {};
    let cmd = cmd.trim();
    if { let v = NoHandler.validate; v(state, cmd) } {
        let e = NoHandler.execute; e(state, cmd)
    } else if exit_command.validate(state, cmd) {
        exit_command.execute(state, cmd)
    } else if clear_command.validate(state, cmd) {
        clear_command.execute(state, cmd)
    } else if { let v = HelpHandler.validate; v(state, cmd) } {
        let e = HelpHandler.execute; e(state, cmd)
    } else if { let v = SampleHandler.validate; v(state, cmd) } {
        let e = SampleHandler.execute; e(state, cmd)
    } else if { let v = VersionHandler.validate; v(state, cmd) } {
        let e = VersionHandler.execute; e(state, cmd)
    } else if { let v = ListHandler.validate; v(state, cmd) } {
        let e = ListHandler.execute; e(state, cmd)
    // } else if cmd.starts_with("open") {
    //     let path = cmd.strip_prefix("open").trim();
    //     return load_relation_from_file(path);
    // } else if cmd.starts_with("load") {
    //     let path = cmd.strip_prefix("load").trim();
    //     return load_relation_from_file(path);
    // }
    } else if { let v = InvalidHandler.validate; v(state, cmd) } {
        let e = InvalidHandler.execute; e(state, cmd)
    } else { false }
}

#[allow(non_upper_case_globals)]
pub const DispatchHandler: Handler = Handler {
    validate,
    execute: dispatch,
};
