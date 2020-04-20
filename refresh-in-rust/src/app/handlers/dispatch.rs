use crate::tui::handler::Handler;

use crate::app::handlers::clear::ClearHandler;
use crate::app::handlers::exit::ExitHandler;
use crate::app::handlers::help::HelpHandler;
use crate::app::handlers::invalid::InvalidHandler;
use crate::app::handlers::list::ListHandler;
use crate::app::handlers::no::NoHandler;
use crate::app::handlers::sample::SampleHandler;
use crate::app::handlers::version::VersionHandler;

use crate::app::utils::always_true as validate;

pub fn dispatch(cmd: &str) -> bool {
    let cmd = cmd.trim();
    if { let v = NoHandler.validate; v(cmd) } {
        let e = NoHandler.execute; e(cmd)
    } else if { let v = ExitHandler.validate; v(cmd) } {
        let e = ExitHandler.execute; e(cmd)
    } else if { let v = ClearHandler.validate; v(cmd) } {
        let e = ClearHandler.execute; e(cmd)
    } else if { let v = HelpHandler.validate; v(cmd) } {
        let e = HelpHandler.execute; e(cmd)
    } else if { let v = SampleHandler.validate; v(cmd) } {
        let e = SampleHandler.execute; e(cmd)
    } else if { let v = VersionHandler.validate; v(cmd) } {
        let e = VersionHandler.execute; e(cmd)
    } else if { let v = ListHandler.validate; v(cmd) } {
        let e = ListHandler.execute; e(cmd)
    // } else if cmd.starts_with("open") {
    //     let path = cmd.strip_prefix("open").trim();
    //     return load_relation_from_file(path);
    // } else if cmd.starts_with("load") {
    //     let path = cmd.strip_prefix("load").trim();
    //     return load_relation_from_file(path);
    // }
    } else if { let v = InvalidHandler.validate; v(cmd) } {
        let e = InvalidHandler.execute; e(cmd)
    } else { false }
}

pub const DispatchHandler: Handler = Handler {
    validate,
    execute: dispatch,
};
