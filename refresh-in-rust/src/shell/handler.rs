use crate::app::state::{AppState, CommandRoutes};

#[derive(PartialEq)]
#[derive(Eq)]
pub enum ValidatorResult {
    Valid,
    Invalid,
}

impl ValidatorResult {
    // returns true if Valid, false if Invalid
    pub fn is_valid(&self) -> bool {
        self == &ValidatorResult::Valid
    }
    // returns true if Invalid, false if Valid
    pub fn is_invalid(&self) -> bool {
        self == &ValidatorResult::Invalid
    }
}

#[derive(PartialEq)]
#[derive(Eq)]
pub enum HandlerResult {
    ContinueLoop,
    Exit,
}

impl HandlerResult {
    // returns true if ContinueLoop, false if Exit
    pub fn should_continue_loop(&self) -> bool {
        self == &HandlerResult::ContinueLoop
    }
    // returns true if Exit, false if ContinueLoop
    pub fn should_exit(&self) -> bool {
        self == &HandlerResult::Exit
    }
}

pub type ValidatorFunction = fn(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult;
pub type HandlerFunction = fn(command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult;

// Stores configuration for a Handler
pub struct Handler {
    // return Invalid to try the next handler, return Valid if handler will handle it
    // by returning Invalid, the handler function will not be called
    // by returning Valid, the handler function will be called
    validator: ValidatorFunction, //fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult,
    // return Exit to exit the application, return Ok to keep running
    // by returning Exit, the application will exit normally
    // by returning Loop, the application will continue to loop
    handler: HandlerFunction, //fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult,
}

// Implement Handler helpers for ValidatorResult
impl ValidatorResult {
    pub fn never_valid(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> ValidatorResult {
        ValidatorResult::Invalid
    }
    pub fn always_valid(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> ValidatorResult {
        ValidatorResult::Valid
    }
}

// Implement Handler helpers for ValidatorResult
impl HandlerResult {
    pub fn do_continue_loop(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> HandlerResult {
        HandlerResult::ContinueLoop
    }
    pub fn do_exit(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> HandlerResult {
        HandlerResult::Exit
    }
}

// Implements Handler
impl Handler {
    pub fn validate(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult {
        // validator => λ anonymous function
        let validator: ValidatorFunction = self.validator;
        validator(command_routes, state, &cmd)
    }
    pub fn handle(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult {
        // handler => λ anonymous function
        let handler: HandlerFunction = self.handler;
        handler(command_routes, state, &cmd)
    }
}

pub struct HandlerBuilder {
    handler: Handler,
}

impl HandlerBuilder {
    pub fn validate(mut self, validator_fn: ValidatorFunction) -> HandlerBuilder {
        self.handler.validator = validator_fn;
        self
    }
    pub fn handle(mut self, handler_fn: HandlerFunction) -> HandlerBuilder {
        self.handler.handler = handler_fn;
        self
    }
    pub fn configured(self) -> Handler {
        self.handler
    }
}

impl Handler {
    pub fn configure() -> HandlerBuilder {
        HandlerBuilder {
            handler: Handler {
                validator: ValidatorResult::never_valid,
                handler: HandlerResult::do_exit,
            }
        }
    }
}

// This trait will be used as an inversion of control.
// The config function, when implemented will return a configured Handler struct.
// Use Handler::build() to use a builder to construct a Handler.
pub trait HandlerConfig {
    fn config() -> Handler;
}
