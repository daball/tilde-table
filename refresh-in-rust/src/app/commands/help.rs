#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::shell::command::{Command, CommandConfig, HandlerResult};
use crate::app::state::AppState;
use crate::app::commands::version::print_version;
use crate::features;
use std::clone::Clone;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

const ALIAS_SEPARATOR: &'static str = "|";
const PARAM_SEPARATOR: &'static str = " ";
const PARAM_PREFIX: &'static str = "[";
const PARAM_SUFFIX: &'static str = "]";

const ALIAS_SEPARATOR_LENGTH: usize = ALIAS_SEPARATOR.len();
const PARAM_SEPARATOR_LENGTH: usize = PARAM_SEPARATOR.len();
const PARAM_PREFIX_LENGTH: usize = PARAM_PREFIX.len();
const PARAM_SUFFIX_LENGTH: usize = PARAM_SUFFIX.len();

#[derive(Clone)]
struct CommandHelp {
    aliases: Vec<String>,
    params: Vec<String>,
    short_desc: String,
}

struct Help {
    about: String,
    categories: HashMap<String, Vec<CommandHelp>>
}

impl Help {
    fn max_aliases_and_params_combined_length(&self) -> usize {
        let mut max_length_found: usize = 0;
        for (_, cmds) in &self.categories {
            for cmd in cmds {
                let mut length = cmd.params_combined_length();
                length += cmd.aliases_combined_length();
                max_length_found = cmp::max(max_length_found, length);
            }
        }
        max_length_found
    }

    fn print_command_list(&self) {
        let max_aliases_length = self.max_aliases_and_params_combined_length();
        for (name, cmds) in &self.categories {
            println!("{} commands:", name);
            for cmd in cmds {
                cmd.print_general_help_line(max_aliases_length);
            }
        }
    }    
}

impl CommandHelp {
    fn aliases_combined_length(&self) -> usize {
        let mut length = (self.aliases.len() - 1) * ALIAS_SEPARATOR_LENGTH;
        for alias in &self.aliases {
            length += alias.len();
        }
        length
    }
    
    fn params_combined_length(&self) -> usize {
        let mut length = self.params.len();
        if length > 0 {
            length = (length - 1) * PARAM_SEPARATOR_LENGTH;
            for param in &self.params {
                length += param.len() + PARAM_PREFIX_LENGTH + PARAM_SUFFIX_LENGTH;
            }
        }
        length
    }
    
    fn get_spaces_for_general_help_line(&self, max_aliases_length: usize) -> String {
        " ".repeat(max_aliases_length - self.aliases_combined_length() - self.params_combined_length() + 1)
    }
    
    fn get_aliases_for_general_help_line_noansi(&self) -> String {
        self.aliases.join(ALIAS_SEPARATOR)
    }
    
    #[cfg(feature="ansi_term")] 
    fn get_aliases_for_general_help_line(&self) -> String {
        let mut mod_aliases: Vec<String> = Vec::new();
        for alias in &self.aliases {
            mod_aliases.append(&mut vec![Colour::Blue.bold().paint(alias).to_string()]);
        }
        mod_aliases.join(ALIAS_SEPARATOR)
    }

    #[cfg(not(feature="ansi_term"))]
    fn get_aliases_for_general_help_line(&self) -> String {
        self.get_aliases_for_general_help_line_noansi()
    }

    fn get_params_for_general_help_line_noansi(&self) -> String {
        let mut mod_params: Vec<String> = Vec::new();
        for param in &self.params {
            mod_params.append(&mut vec![fmt::format(format_args!("{}{}{}", PARAM_PREFIX, param, PARAM_SUFFIX))]);
        }
        fmt::format(format_args!(" {}", mod_params.join(PARAM_SEPARATOR)))
    }

    #[cfg(feature="ansi_term")]
    fn get_params_for_general_help_line(&self) -> String {
        let mut mod_params: Vec<String> = Vec::new();
        for param in &self.params {
            let param = Colour::Green.bold().paint(param).to_string();
            mod_params.append(&mut vec![fmt::format(format_args!("{}{}{}", PARAM_PREFIX, param, PARAM_SUFFIX))]);
        }
        fmt::format(format_args!(" {}", mod_params.join(PARAM_SEPARATOR)))
    }

    #[cfg(not(feature="ansi_term"))]
    fn get_params_for_general_help_line(&self) -> String {
        self.get_params_for_general_help_line_noansi()
    }
    
    fn print_general_help_line(&self, max_aliases_length: usize) {
        let spaces = self.get_spaces_for_general_help_line(max_aliases_length);
        let aliases = self.get_aliases_for_general_help_line();
        let params = self.get_params_for_general_help_line();
        let short_desc = &self.short_desc;
        println!("  {}{}{}{}", aliases, params, spaces, short_desc);
    }
}

// HelpBuilder builder facet

struct HelpBuilder {
    help: Help,
}

impl HelpBuilder {
    fn about(mut self, about: &str) -> HelpBuilder {
        self.help.about = String::from(about);
        self
    }
    fn build(self) -> Help {
        self.help
    }
}

impl Help {
    fn define() -> HelpBuilder {
        HelpBuilder {
            help: Help {
                about: String::new(),
                categories: HashMap::new()
            }
        }
    }
}

// HelpCategoryBuilder builder facet

struct HelpCategoryBuilder {
    help_builder: HelpBuilder,
    name: String,
    commands: Vec<CommandHelp>,
}

impl HelpBuilder {
    fn category(self, name: &str) -> HelpCategoryBuilder {
        if self.help.categories.contains_key(name) {
            let name = String::from(name);
            let commands = self.help.categories.get(&name).unwrap().to_vec();
            HelpCategoryBuilder {
                help_builder: self,
                name: name,
                commands: commands,
            }
        } else {
            HelpCategoryBuilder {
                help_builder: self,
                name: String::from(name),
                commands: Vec::default(),
            }
        }
    }
}

impl HelpCategoryBuilder {
    fn done(mut self) -> HelpBuilder {
        self.help_builder.help.categories.insert(self.name, self.commands);
        self.help_builder
    }
    fn about(self, about: &str) -> HelpBuilder {
        self.done().about(about)
    }
    fn category(self, name: &str) -> HelpCategoryBuilder {
        self.done().category(name)
    }
    fn build(self) -> Help {
        self.done().build()
    }
}

// CommandHelpBuilder builder facet

struct CommandHelpBuilder {
    help_category_builder: HelpCategoryBuilder,
    cmd: CommandHelp,
}

impl HelpCategoryBuilder {
    fn command(self, alias: &str) -> CommandHelpBuilder {
        CommandHelpBuilder {
            help_category_builder: self,
            cmd: CommandHelp {
                aliases: vec![String::from(alias)],
                params: Vec::new(),
                short_desc: String::default(),
            },
        }
    }
}

impl CommandHelpBuilder {
    fn alias(mut self, alias: &str) -> CommandHelpBuilder {
        self.cmd.aliases.append(&mut vec![String::from(alias)]);
        self
    }
    fn param(mut self, param: &str) -> CommandHelpBuilder {
        self.cmd.params.append(&mut vec![String::from(param)]);
        self
    }
    fn short_desc(mut self, short_desc: &str) -> CommandHelpBuilder {
        self.cmd.short_desc = String::from(short_desc);
        self
    }
    fn done(mut self) -> HelpCategoryBuilder {
        self.help_category_builder.commands.append(&mut vec![self.cmd]);
        self.help_category_builder
    }
    fn command(self, name: &str) -> CommandHelpBuilder {
        self.done().command(name)
    }
    fn about(self, about: &str) -> HelpBuilder {
        self.done().about(about)
    }
    fn category(self, name: &str) -> HelpCategoryBuilder {
        self.done().category(name)
    }
    fn build(self) -> Help {
        self.done().build()
    }
}

fn build_general_command_help() -> Help {
    Help::define()
        .about(&fmt::format(format_args!("{} Type a command using the syntax below.", features::description())))
        .category("Basic")
            .command("?")
                .alias("help")
                .short_desc("Prints this help page.")
            .command("clear")
                .alias("cls")
                .short_desc("Clears the screen.")
            .command("exit")
                .alias("quit").alias("qui").alias("qu").alias("q")
                .short_desc("Exits/quits the application.")
            .command("history")
                .param("n")
                .short_desc("Displays up to optional number of history items.")
            .command("sample")
                .short_desc("Runs the sample routine.")
            .command("version")
                .alias("ver")
                .short_desc("Prints version information.")
        .category("Relation")
            .command("ls")
                .alias("list").alias("dir")
                .param("path")
                .short_desc("Lists all valid (*.txt) files to read at optional path.")
            .command("open")
                .param("path")
                .short_desc("Opens file at required path to a named relation.")
            .command("show")
                .short_desc("Lists all loaded relations by name.")
            .command("describe")
                .alias("desc")
                .param("rel")
                .short_desc("Describes a relation, including name, path, and filter chain.")
            .command("rename")
                .alias("rn")
                .param("rel").param("new")
                .short_desc("Renames a relation.")
            .command("filter")
                .param("rel")
                .short_desc("Applies filters on a relation as a separate relation.")
            .command("project")
                .param("rel")
                .short_desc("Prints the contents of the relation.")
            .command("close")
                .param("rel")
                .short_desc("Unloads a relation.")
    .build()
}

pub fn print_help() {
    let help: Help = build_general_command_help();
    print_version();
    println!("{}", help.about);
    println!();
    help.print_command_list();
    println!();
}

pub struct HelpCommand { }

impl HelpCommand {
    pub fn handler(_state: &mut AppState, _cmd: &str) -> HandlerResult {
        print_help();
        HandlerResult::ContinueLoop
    }
}

impl CommandConfig for HelpCommand {
    fn config() -> Command {
        Command::configure("?").alias("help")
            .arg("cmd").desc("The command to display help about").optional()
            .short_desc("Prints this help page.")
            .category("Basic")
            .handle(HelpCommand::handler)
            .configured()
    }
}
