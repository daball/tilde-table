#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use ansi_term::Colour;
use crate::app::state::AppState;
use crate::shell::handler::Handler;
use crate::features;

#[cfg(feature="ansi_term")]
pub fn print_version() {
    println!("{} ({}) by {}",
        Colour::Blue.bold().paint(features::friendly_name()),
        Colour::Cyan.bold().paint(features::name()),
        Colour::Green.bold().paint(features::authors())
    );
    println!("version: {}, os: {}; family: {}; features: {}",
        Colour::Blue.bold().paint(features::version()),
        Colour::Cyan.bold().paint(features::target_os()),
        Colour::Green.bold().paint(features::target_family()),
        Colour::Yellow.bold().paint(features::features_enabled())
    );
    println!("web: {}, repo: {}",
        Colour::Blue.bold().paint(features::homepage()),
        Colour::Purple.bold().paint(features::repository())
    );
    println!("");
}

pub fn print_version_noansi() {
    println!("{} ({}) by {}",
        features::friendly_name(),
        features::name(),
        features::authors()
    );
    println!("version: {}, os: {}; family: {}; features: {}",
        features::version(),
        features::target_os(),
        features::target_family(),
        features::features_enabled()
    );
    println!("web: {}, repo: {}",
        features::homepage(),
        features::repository()
    );
    println!("");
}

#[cfg(not(feature="ansi_term"))]
pub const print_version: fn() = print_version_noansi;

fn validate(_state: &mut AppState, cmd: &str) -> bool {
    cmd.eq("ver") || cmd.eq("version")
}

fn execute(_state: &mut AppState, cmd: &str) -> bool {
    print_version();
    true // continue read-eval-print-loop
}

pub const VersionHandler: Handler = Handler {
    validate,
    execute,
};
