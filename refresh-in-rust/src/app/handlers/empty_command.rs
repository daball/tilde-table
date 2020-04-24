use crate::shell::handler::Handler;
use crate::app::state::{AppState, CommandRoutes};

pub struct EmptyCommandHandler {}

impl Handler for EmptyCommandHandler {
    fn validate(&self, _command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> bool {
        cmd.trim().is_empty()
    }
        
    fn handle(&self, _command_routes: &mut CommandRoutes, _state: &mut AppState, cmd: &str) -> bool {
        cmd.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::state::{AppState, CommandRoutes};
    use crate::shell::handler::Handler;
    use super::EmptyCommandHandler;
    #[test]
    fn validate_empty_command_handler() {
        let mut routes = CommandRoutes::create();
        let mut state = AppState::new();
        let handler: EmptyCommandHandler = EmptyCommandHandler {};
        assert_eq!(handler.validate(&mut routes, &mut state, ""), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "      "), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "\n\n\n"), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "\t\t\t"), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "\r\r\r\r"), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "\r\n\r\n\r\n"), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "actualcommand"), false);
    }
    #[test]
    fn handle_empty_command_handler() {
        let mut routes = CommandRoutes::create();
        let mut state = AppState::new();
        let handler: EmptyCommandHandler = EmptyCommandHandler {};
        assert_eq!(handler.handle(&mut routes, &mut state, ""), true);
        assert_eq!(handler.handle(&mut routes, &mut state, "      "), true);
        assert_eq!(handler.handle(&mut routes, &mut state, "\n\n\n"), true);
        assert_eq!(handler.handle(&mut routes, &mut state, "\t\t\t"), true);
        assert_eq!(handler.handle(&mut routes, &mut state, "\r\r\r\r"), true);
        assert_eq!(handler.handle(&mut routes, &mut state, "\r\n\r\n\r\n"), true);
        assert_eq!(handler.validate(&mut routes, &mut state, "actualcommand"), false);
    }
}
