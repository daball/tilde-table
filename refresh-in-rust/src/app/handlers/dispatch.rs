use crate::shell::command::Command;
use crate::shell::handler::Handler;
use crate::app::commands::clear::ClearCommand;
use crate::app::commands::exit::ExitCommand;
use crate::app::commands::help::HelpCommand;
use crate::app::commands::list::ListCommand;
use crate::app::commands::sample::SampleCommand;
use crate::app::commands::version::VersionCommand;
use crate::app::handlers::invalid::InvalidHandler;
use crate::app::handlers::no::NoHandler;
use crate::app::handlers::utils::always_true as validate;
use crate::app::state::AppState;
use std::rc::Rc;

pub fn dispatch(state: &mut AppState, cmd: &str) -> bool {
    let commands: Vec<Rc<dyn Command>> = vec![
        Rc::new(ExitCommand {}),
        Rc::new(HelpCommand {}),
        Rc::new(ListCommand {}),
        Rc::new(ClearCommand {}),
        Rc::new(SampleCommand {}),
        Rc::new(VersionCommand {}),
    ];
    let cmd = cmd.trim();
    if { let v = NoHandler.validate; v(state, cmd) } {
        let e = NoHandler.execute; e(state, cmd)
    } else {
        for command in &commands {
            let command: &dyn Command = command.as_ref(); 
            if command.validate(state, cmd) {
                return command.execute(state, cmd)
            }
        }
        // } else if cmd.starts_with("open") {
        //     let path = cmd.strip_prefix("open").trim();
        //     return load_relation_from_file(path);
        // } else if cmd.starts_with("load") {
        //     let path = cmd.strip_prefix("load").trim();
        //     return load_relation_from_file(path);
        // }
        if { let v = InvalidHandler.validate; v(state, cmd) } {
            let e = InvalidHandler.execute; e(state, cmd)
        } else { false }
    }
}

#[allow(non_upper_case_globals)]
pub const DispatchHandler: Handler = Handler {
    validate,
    execute: dispatch,
};
