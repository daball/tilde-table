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
    ExitLoop,
}

impl HandlerResult {
    // returns true if ContinueLoop, false if ExitLoop
    pub fn should_continue_loop(&self) -> bool {
        self == &HandlerResult::ContinueLoop
    }
    // returns true if ExitLoop, false if ContinueLoop
    pub fn should_exit_loop(&self) -> bool {
        self == &HandlerResult::ExitLoop
    }
}

pub type ValidatorFn = fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult;
pub type HandlerFn = fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult;

// Stores configuration for a Handler
pub struct Handler {
    // return Invalid to try the next handler, return Valid if handler will handle it
    // by returning Invalid, the handler function will not be called
    // by returning Valid, the handler function will be called
    pub validator: ValidatorFn, //fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult,
    // return Exit to exit the application, return Ok to keep running
    // by returning Exit, the application will exit normally
    // by returning Loop, the application will continue to loop
    pub handler: HandlerFn, //fn (command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult,
}

fn default_validator(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> ValidatorResult {
    if cfg!(feature="debug") {
        eprintln!("[error: Handler] This handler has not been configured with a validator function.");
    }
    ValidatorResult::Invalid
}

fn default_handler(_command_routes: &mut CommandRoutes, _state: &mut AppState, _cmd: &str) -> HandlerResult {
    if cfg!(feature="debug") {
        eprintln!("[error: Handler] This handler has not been configured with a handler function.");
    }
    HandlerResult::ContinueLoop
}

// Implements Handler
impl Handler {
    pub fn validate(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> ValidatorResult {
        // validator => λ anonymous function
        let validator: ValidatorFn = self.validator;
        validator(command_routes, state, &cmd)
    }
    pub fn handle(&self, command_routes: &mut CommandRoutes, state: &mut AppState, cmd: &str) -> HandlerResult {
        // handler => λ anonymous function
        let handler: HandlerFn = self.handler;
        handler(command_routes, state, &cmd)
    }
}

pub struct HandlerBuilder {
    handler: Handler,
}

impl HandlerBuilder {
    pub fn validate(mut self, validator_fn: ValidatorFn) -> HandlerBuilder {
        self.handler.validator = validator_fn;
        self
    }
    pub fn handle(mut self, handler_fn: HandlerFn) -> HandlerBuilder {
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
                validator: default_validator,
                handler: default_handler,
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
