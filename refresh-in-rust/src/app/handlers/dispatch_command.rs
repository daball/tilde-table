use crate::shell::handler::{Handler, HandlerConfig, HandlerResult, ValidatorResult};
use crate::app::state::{AppState, CommandRoutes};

pub struct DispatchCommandHandler {}

impl DispatchCommandHandler {
    pub fn validator(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            match command.validate(state, cmd) {
                ValidatorResult::Valid => return ValidatorResult::Valid,
                ValidatorResult::Invalid => continue,
            }
        }
        ValidatorResult::Invalid
    }
    
    pub fn handler(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            if cfg!(feature="debug") {
                println!("[debug: repl] Trying handler for {} command.", command.aliases.join("|"));
            }
            match command.validate(state, cmd) {
                ValidatorResult::Valid => {
                    let handled: HandlerResult = command.handle(state, cmd);
                    state.command_history.append(&mut vec![cmd.to_string()]);
                    return handled
                },
                ValidatorResult::Invalid => continue,
            }
        }
        if cfg!(feature="debug") && Self::validator(command_routes, state, cmd) == ValidatorResult::Invalid {
            eprintln!("[debug: DispatchCommandHandler] Invalid data passed to empty command handler.");
        }
        HandlerResult::ContinueLoop
    }
}

impl HandlerConfig for DispatchCommandHandler {
    fn config() -> Handler {
        Handler::configure()
            .validate(DispatchCommandHandler::validator)
            .handle(DispatchCommandHandler::handler)
            .configured()
    }
}
