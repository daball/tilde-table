use crate::app::state::AppState;
use crate::shell::command::{Command, CommandDefinition};
use crate::app::ui::render::version;

pub fn print_version_noansi() {
    println!("{} ({}) by {}",
        version::friendly_name_noansi(),
        version::package_name_noansi(),
        version::authors_noansi()
    );
    println!("version: {}, os: {}; family: {}",
        version::version_noansi(),
        version::target_os_noansi(),
        version::target_family_noansi()
    );
    println!("features: {}",
        version::features_noansi()
    );
    println!("dependencies: {}",
        version::dependencies_noansi()
    );
    println!("web: {}, repo: {}",
        version::homepage_noansi(),
        version::repository_noansi()
    );
    println!("");
}

#[cfg(feature="ansi_term")]
pub fn print_version() {
    println!("{} ({}) by {}",
        version::friendly_name(),
        version::package_name(),
        version::authors()
    );
    println!("version: {}, os: {}; family: {}",
        version::version(),
        version::target_os(),
        version::target_family()
    );
    println!("features: {}",
        version::features()
    );
    println!("dependencies: {}",
        version::dependencies()
    );
    println!("web: {}, repo: {}",
        version::homepage(),
        version::repository()
    );
    println!("");
}

#[cfg(not(feature="ansi_term"))]
pub const print_version: fn() = print_version_noansi;

pub struct VersionCommand { }

impl Command for VersionCommand {
    fn definition(&self) -> CommandDefinition {
        CommandDefinition::define("version").alias("ver")
            .short_desc("Prints version information.")
            .category("Basic")
            .definition()
    }
    fn validate(&self, _state: &mut AppState, cmd: &str) -> bool {
        cmd.eq("ver") || cmd.eq("version")
    }
    fn execute(&self, _state: &mut AppState, _cmd: &str) -> bool {
        print_version();
        true // continue read-eval-print-loop
    }
}
