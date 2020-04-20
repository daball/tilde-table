extern crate ansi_term;
use ansi_term::Colour;
use crate::tui::handler::Handler;
use crate::app::handlers::version::print_version;
use std::clone::Clone;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

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
    fn max_aliases_and_params_combined_length(&self, separator_length: usize, prefix_length: usize, suffix_length: usize) -> usize {
        let mut max_length_found: usize = 0;
        for (_, cmds) in &self.categories {
            for cmd in cmds {
                let mut length = cmd.params_combined_length(prefix_length, suffix_length);
                length += cmd.aliases_combined_length(1);
                max_length_found = cmp::max(max_length_found, length);
            }
        }
        max_length_found
    }
}

impl CommandHelp {
    fn aliases_combined_length(&self, separator_length: usize) -> usize {
        let mut length = (self.aliases.len() - 1) * separator_length;
        for alias in &self.aliases {
            length += alias.len();
        }
        length
    }
    fn params_combined_length(&self, prefix_length: usize, suffix_length: usize) -> usize {
        let mut length = self.params.len();
        if length > 0 {
            length = (length - 1) * 1;
            for param in &self.params {
                length += param.len() + prefix_length + suffix_length;
            }
        }
        length
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
    helpBuilder: HelpBuilder,
    name: String,
    commands: Vec<CommandHelp>,
}

impl HelpBuilder {
    fn category(self, name: &str) -> HelpCategoryBuilder {
        if self.help.categories.contains_key(name) {
            let name = String::from(name);
            let commands = self.help.categories.get(&name).unwrap().to_vec();
            HelpCategoryBuilder {
                helpBuilder: self,
                name: name,
                commands: commands,
            }
        } else {
            HelpCategoryBuilder {
                helpBuilder: self,
                name: String::from(name),
                commands: Vec::default(),
            }
        }
    }
}

impl HelpCategoryBuilder {
    fn done(mut self) -> HelpBuilder {
        self.helpBuilder.help.categories.insert(self.name, self.commands);
        self.helpBuilder
    }
    fn about(mut self, about: &str) -> HelpBuilder {
        self.done().about(about)
    }
    fn category(mut self, name: &str) -> HelpCategoryBuilder {
        self.done().category(name)
    }
    fn build(mut self) -> Help {
        self.done().build()
    }
}

// CommandHelpBuilder builder facet

struct CommandHelpBuilder {
    helpCategoryBuilder: HelpCategoryBuilder,
    cmd: CommandHelp,
}

impl HelpCategoryBuilder {
    fn command(self, alias: &str) -> CommandHelpBuilder {
        CommandHelpBuilder {
            helpCategoryBuilder: self,
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
        self.helpCategoryBuilder.commands.append(&mut vec![self.cmd]);
        self.helpCategoryBuilder
    }
    fn command(mut self, name: &str) -> CommandHelpBuilder {
        self.done().command(name)
    }
    fn about(mut self, about: &str) -> HelpBuilder {
        self.done().about(about)
    }
    fn category(mut self, name: &str) -> HelpCategoryBuilder {
        self.done().category(name)
    }
    fn build(mut self) -> Help {
        self.done().build()
    }
}

fn build_general_command_help() -> Help {
    Help::define()
        .about("Processes spreadsheet data files. To use this application, type a command using the syntax below.")
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
                .short_desc("Applies filters on a relationn as a separate relation.")
            .command("project")
                .param("rel")
                .short_desc("Prints the contents of the relation.")
            .command("close")
                .param("rel")
                .short_desc("Unloads a relation.")
    .build()
}

fn print_command_list(help: &Help) {
    let separator = "|";
    let param_separator = " ";
    let param_prefix = "[";
    let param_suffix = "]";
    let max_aliases_length = help.max_aliases_and_params_combined_length(separator.len(), param_prefix.len(), param_suffix.len());
    for (name, cmds) in &help.categories {
        println!("{} commands:", name);
        for cmd in cmds {
            let spaces = " ".repeat(max_aliases_length - cmd.aliases_combined_length(separator.len()) - cmd.params_combined_length(param_prefix.len(), param_suffix.len()) + 1);
            let mut modAliases: Vec<String> = Vec::new();
            for alias in &cmd.aliases {
                modAliases.append(&mut vec![Colour::Blue.bold().paint(alias).to_string()]);
            }
            let aliases = modAliases.join(separator);
            let mut modParams: Vec<String> = Vec::new();
            for param in &cmd.params {
                let param = Colour::Green.bold().paint(param).to_string();
                modParams.append(&mut vec![fmt::format(format_args!("{}{}{}", param_prefix, param, param_suffix))]);
            }
            let params = fmt::format(format_args!(" {}", modParams.join(param_separator)));
            println!("  {}{}{}{}", aliases, params, spaces, cmd.short_desc);
        }
    }
}

pub fn print_help() {
    let help: Help = build_general_command_help();
    print_version();
    println!("{}", help.about);
    println!();
    print_command_list(&help);
    println!();
}

fn validate(cmd: &str) -> bool {
    cmd.eq("?") || cmd.eq("help")
}

fn execute(cmd: &str) -> bool {
    print_help();
    true // continue read-eval-print-loop
}

pub const HelpHandler: Handler = Handler {
    validate,
    execute,
};
