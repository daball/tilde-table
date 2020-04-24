use crate::app::state::{AppState, CommandRoutes};
use crate::shell::handler::{Handler, HandlerConfig, HandlerResult, ValidatorResult};
use crate::app::ui::render::invalid as render;

pub struct InvalidCommandHandler {}

impl InvalidCommandHandler {
    pub fn validator(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> ValidatorResult {
        ValidatorResult::Valid
    }
    pub fn handler(_command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> HandlerResult {
        println!("{}", render::invalid(cmd.trim()));
        HandlerResult::ContinueLoop
    }    
}

impl HandlerConfig for InvalidCommandHandler {
    fn config() -> Handler {
        Handler::configure()
            .validate(InvalidCommandHandler::validator)
            .handle(InvalidCommandHandler::handler)
            .configured()
    }
}
