use crate::app::state::AppState;
// pub use super::builder;

pub struct ActualParam {
    name: String,
    value: String,
}

pub struct FormalParam {
    name: String,
    desc: String,
    is_optional: bool,
}

pub struct FormalParamBuilder {
    command_definition_builder: CommandDefinitionBuilder,
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
    pub fn done(mut self) -> CommandDefinitionBuilder {
        self.command_definition_builder.command_definition.params.append(
            &mut vec![self.formal_param]
        );
        self.command_definition_builder
    }
    pub fn alias(mut self, alias: &str) -> CommandDefinitionBuilder {
        self.done().alias(alias)
    }
    pub fn param(mut self, name: &str) -> FormalParamBuilder {
        self.done().param(name)
    }
    pub fn short_desc(mut self, short_desc: &str) -> CommandDefinitionBuilder {
        self.done().short_desc(short_desc)
    }
    pub fn long_desc(mut self, long_desc: &str) -> CommandDefinitionBuilder {
        self.done().short_desc(long_desc)
    }
    pub fn category(mut self, category: &str) -> CommandDefinitionBuilder {
        self.done().category(category)
    }
    pub fn definition(mut self) -> CommandDefinition {
        self.done().definition()
    }
}

pub struct CommandDefinition {
    aliases: Vec<String>,
    category: String,
    short_desc: String,
    long_desc: String,
    params: Vec<FormalParam>,
}

pub struct CommandDefinitionBuilder {
    command_definition: CommandDefinition,
}

impl CommandDefinition {
    pub fn define(name: &str) -> CommandDefinitionBuilder {
        CommandDefinitionBuilder {
            command_definition: CommandDefinition {
                aliases: vec![String::from(name)],
                category: String::default(),
                short_desc: String::default(),
                long_desc: String::default(),
                params: Vec::default(),
            },
        }
    }
}

impl CommandDefinitionBuilder {
    pub fn alias(mut self, alias: &str) -> CommandDefinitionBuilder {
        self.command_definition.aliases.append(&mut vec![String::from(alias)]);
        self
    }
    pub fn param(mut self, param: &str) -> FormalParamBuilder {
        FormalParamBuilder {
            command_definition_builder: self,
            formal_param: FormalParam {
                name: String::from(param),
                desc: String::default(),
                is_optional: false,
            }
        }
    }
    pub fn short_desc(mut self, short_desc: &str) -> CommandDefinitionBuilder {
        self.command_definition.short_desc = String::from(short_desc);
        if self.command_definition.long_desc.is_empty() {
            self.command_definition.long_desc = String::from(short_desc);
        }
        self
    }
    pub fn long_desc(mut self, long_desc: &str) -> CommandDefinitionBuilder {
        self.command_definition.long_desc = String::from(long_desc);
        if self.command_definition.short_desc.is_empty() {
            self.command_definition.short_desc = String::from(long_desc);
        }
        self
    }
    pub fn category(mut self, category: &str) -> CommandDefinitionBuilder {
        self.command_definition.category = String::from(category);
        self
    }
    pub fn definition(mut self) -> CommandDefinition {
        self.command_definition
    }
}

pub trait Command {
// pub trait CommandHandler {
    fn validate(&self, state: &mut AppState, cmd: &str) -> bool;
    fn execute(&self, state: &mut AppState, cmd: &str) -> bool;
// }

// pub trait Command<CH: CommandHandler> {
    fn definition(&self) -> CommandDefinition;
    // fn handler() -> CH;
}
