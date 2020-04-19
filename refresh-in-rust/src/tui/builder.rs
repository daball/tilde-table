pub struct CommandBuilder {
    command: Command
}

impl Command {
    pub fn define() -> CommandBuilder {
        CommandBuilder {
            command: Command {
                aliases: Vec::new(),
                category: String::new(),
                short_description: String::new(),
                long_description: String::new(),
                params: Vec::new()
            }
        }
    }
}

impl CommandBuilder {
    pub fn alias(self, alias: &str) -> CommandBuilder {
        // let mut aliases: String 
        CommandBuilder {
            command: Command {
                // aliases,
                ..self.command
            },
            ..self
        }
    }
    pub fn category(self, category: &str) -> CommandBuilder {
        CommandBuilder {
            command: Command {
                category: String::from(category),
                ..self.command
            },
            ..self
        }
    }
    pub fn shortDescription(self, short_description: &str) -> CommandBuilder {
        CommandBuilder {
            command: Command {
                short_description: String::from(short_description),
                ..self.command
            },
            ..self
        }
    }
    pub fn longDescription(self, long_description: &str) -> CommandBuilder {
        CommandBuilder {
            command: Command {
                long_description: String::from(long_description),
                ..self.command
            },
            ..self
        }
    }
    pub fn param(self) -> CommandParamBuilder {
        CommandParamBuilder {
            commandBuilder: self,
            param: Param {
                name: String::new(),
                description: String::new(),
                optional: false
            }
        }
    }
}

pub struct CommandParamBuilder {
    commandBuilder: CommandBuilder,
    param: Param
}

pub impl CommandParamBuilder {
    fn name(self, name: &str) -> CommandParamBuilder {
        CommandParamBuilder {
            param: Param {
                name,
                ..self.param
            },
            ..self
        }
    }
    fn optional(self, optional: bool) -> CommandParamBuilder {
        CommandParamBuilder {
            param: Param {
                optional,
                ..self.param
            },
            ..self
        }
    }
    fn description(self, description: &str) -> CommandParamBuilder {
        CommandParamBuilder {
            param: Param {
                description,
                ..self.param
            },
            ..self
        }
    }
    fn done(self) -> CommandBuilder {
        let params: Vec<Param> = self.commandBuilder.command.params.clone();
        params.add(self);
        CommandBuilder { 
            command: Command {
                params,
                ..self.commandBuilder.command
            },
            ..self.commandBuilder
        }
    }
}
