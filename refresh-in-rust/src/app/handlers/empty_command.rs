use crate::shell::handler::{Handler, HandlerConfig, HandlerResult, ValidatorResult};
use crate::app::state::{AppState, CommandRoutes};

pub struct EmptyCommandHandler {}

impl EmptyCommandHandler {
    pub fn validator(_command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> ValidatorResult {
        if cmd.trim().is_empty() {
            ValidatorResult::Valid
        } else {
            ValidatorResult::Invalid
        }
    }
    pub fn handler(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult {
        if cfg!(feature="debug") && Self::validator(command_routes, state, cmd) == ValidatorResult::Invalid {
            eprintln!("[debug: EmptyCommandHandler] Invalid data passed to empty command handler.");
        }
        HandlerResult::ContinueLoop
    }
}

impl HandlerConfig for EmptyCommandHandler {
    fn config() -> Handler {
        Handler::configure()
            .validate(EmptyCommandHandler::validator)
            .handle(EmptyCommandHandler::handler)
            .configured()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::state::{AppState, CommandRoutes};
    use crate::shell::handler::{HandlerConfig, HandlerResult, ValidatorResult};
    use super::EmptyCommandHandler;
    #[test]
    fn validate_empty_command_handler() {
        let mut routes = CommandRoutes::create();
        let mut state = AppState::new();
        let handler = EmptyCommandHandler::config();
        assert!(handler.validate(&mut routes, &mut state, "").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "      ").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "\n\n\n").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "\t\t\t").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "\r\r\r\r").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "\r\n\r\n\r\n").is_valid());
        assert!(handler.validate(&mut routes, &mut state, "actualcommand").is_invalid());
    }
    #[test]
    fn handle_empty_command_handler() {
        let mut routes = CommandRoutes::create();
        let mut state = AppState::new();
        let handler = EmptyCommandHandler::config();
        assert!(handler.handle(&mut routes, &mut state, "").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "      ").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "\n\n\n").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "\t\t\t").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "\r\r\r\r").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "\r\n\r\n\r\n").should_continue_loop());
        assert!(handler.handle(&mut routes, &mut state, "actualcommand").should_continue_loop());
    }
}
