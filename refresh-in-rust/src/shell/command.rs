use crate::app::state::AppState;
pub use super::handler::{ValidatorResult, HandlerResult};

pub struct ActualParam {
    pub name: String,
    pub value: String,
}

pub struct FormalParam {
    pub name: String,
    pub desc: String,
    pub is_optional: bool,
}

pub struct FormalParamBuilder {
    command_builder: CommandBuilder,
    formal_param: FormalParam,
}

impl FormalParamBuilder {
    pub fn desc(mut self, desc: &str) -> FormalParamBuilder {
        self.formal_param.desc = String::from(desc);
        self
    }
    pub fn optional(mut self) -> FormalParamBuilder {
        self.formal_param.is_optional = true;
        self
    }
    pub fn not_optional(mut self) -> FormalParamBuilder {
        self.formal_param.is_optional = false;
        self
    }
    pub fn done(mut self) -> CommandBuilder {
        self.command_builder.command.params.append(
            &mut vec![self.formal_param]
        );
        self.command_builder
    }
    pub fn alias(self, alias: &str) -> CommandBuilder {
        self.done().alias(alias)
    }
    pub fn param(self, name: &str) -> FormalParamBuilder {
        self.done().param(name)
    }
    pub fn short_desc(self, short_desc: &str) -> CommandBuilder {
        self.done().short_desc(short_desc)
    }
    pub fn long_desc(self, long_desc: &str) -> CommandBuilder {
        self.done().short_desc(long_desc)
    }
    pub fn category(self, category: &str) -> CommandBuilder {
        self.done().category(category)
    }
    pub fn validate(self, validator_fn: ValidatorFn) -> CommandBuilder {
        self.done().validate(validator_fn)
    }
    pub fn handle(self, handler_fn: HandlerFn) -> CommandBuilder {
        self.done().handle(handler_fn)
    }
    pub fn configured(self) -> Command {
        self.done().configured()
    }
}

pub type ValidatorFn = fn (state: &mut AppState, cmd: &str) -> ValidatorResult;
pub type HandlerFn = fn (state: &mut AppState, cmd: &str) -> HandlerResult;

pub struct Command {
    pub aliases: Vec<String>,
    pub category: String,
    pub short_desc: String,
    pub long_desc: String,
    pub params: Vec<FormalParam>,
    pub validator: ValidatorFn,
    pub handler: HandlerFn,
}

impl Command {
    pub fn validate(&self, state: &mut AppState, cmd: &str) -> ValidatorResult {
        // validator => λ anonymous function
        let validator: ValidatorFn = self.validator;
        validator(state, &cmd)
    }
    pub fn handle(&self, state: &mut AppState, cmd: &str) -> HandlerResult {
        // handler => λ anonymous function
        let handler: HandlerFn = self.handler;
        handler(state, &cmd)
    }
}

pub struct CommandBuilder {
    command: Command,
}

impl CommandBuilder {
    pub fn alias(mut self, alias: &str) -> CommandBuilder {
        self.command.aliases.append(&mut vec![String::from(alias)]);
        self
    }
    pub fn param(self, param: &str) -> FormalParamBuilder {
        FormalParamBuilder {
            command_builder: self,
            formal_param: FormalParam {
                name: String::from(param),
                desc: String::default(),
                is_optional: false,
            }
        }
    }
    pub fn short_desc(mut self, short_desc: &str) -> CommandBuilder {
        self.command.short_desc = String::from(short_desc);
        if self.command.long_desc.is_empty() {
            self.command.long_desc = String::from(short_desc);
        }
        self
    }
    pub fn long_desc(mut self, long_desc: &str) -> CommandBuilder {
        self.command.long_desc = String::from(long_desc);
        if self.command.short_desc.is_empty() {
            self.command.short_desc = String::from(long_desc);
        }
        self
    }
    pub fn category(mut self, category: &str) -> CommandBuilder {
        self.command.category = String::from(category);
        self
    }
    pub fn validate(mut self, validator_fn: ValidatorFn) -> CommandBuilder {
        self.command.validator = validator_fn;
        self
    }
    pub fn handle(mut self, handler_fn: HandlerFn) -> CommandBuilder {
        self.command.handler = handler_fn;
        self
    }
    pub fn configured(self) -> Command {
        self.command
    }
}

fn default_validator(_state: &mut AppState, _cmd: &str) -> ValidatorResult {
    ValidatorResult::Invalid
}

fn default_handler(_state: &mut AppState, _cmd: &str) -> HandlerResult {
    if cfg!(feature="debug") {
        eprintln!("[error: Command] This command has not been configured with a handler function.");
    }
    HandlerResult::ContinueLoop
}

impl Command {
    pub fn configure(name: &str) -> CommandBuilder {
        CommandBuilder {
            command: Command {
                aliases: vec![String::from(name)],
                category: String::default(),
                short_desc: String::default(),
                long_desc: String::default(),
                params: Vec::default(),
                validator: default_validator,
                handler: default_handler,
            },
        }
    }
}


// This trait will be used as an inversion of control.
// The config function, when implemented will return a configured Handler struct.
// Use Handler::build() to use a builder to construct a Handler.
pub trait CommandConfig {
    fn config() -> Command;
}
