use crate::shell::command::Command;
use crate::shell::handler::{Handler, HandlerConfig, HandlerResult, ValidatorResult};
use crate::app::state::{AppState, CommandRoutes};

pub struct DispatchCommandHandler {}

impl DispatchCommandHandler {
    pub fn validator(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            let command: &dyn Command = command.as_ref(); 
            if command.validate(state, cmd) {
                return ValidatorResult::Valid
            }
        }
        ValidatorResult::Invalid
    }
    
    pub fn handler(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult {
        let cmd = cmd.trim();
        for command in &mut command_routes.commands {
            let command: &dyn Command = command.as_ref(); 
            if command.validate(state, cmd) {
                let exec: bool = command.execute(state, cmd);
                state.command_history.append(&mut vec![cmd.to_string()]);
                if exec {
                    return HandlerResult::ContinueLoop
                } else {
                    return HandlerResult::Exit
                }
            }
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
