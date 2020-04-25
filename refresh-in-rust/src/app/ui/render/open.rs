#![allow(unused_imports)]
#[cfg(feature="ansi_term")] extern crate ansi_term;
use super::style as style;

#[cfg(feature="ansi_term")] #[allow(non_upper_case_globals)] #[allow(unused_imports)]
pub mod colors {
    #[cfg(feature="ansi_term")] use ansi_term::{Colour as Color, Colour::{
        Red,
        Yellow
    }};
    // alias colors
    #[cfg(feature="ansi_term")] pub const InvalidColor: Color = Red;
    #[cfg(feature="ansi_term")] pub const InvalidParamColor: Color = Yellow;
}

#[cfg(feature="ansi_term")] #[allow(non_snake_case)]
pub mod styles {
    #[cfg(feature="ansi_term")] use ansi_term::Style;
    #[cfg(feature="ansi_term")] use super::colors::{
        InvalidColor, InvalidParamColor
    };
    // alias styles
    #[cfg(feature="ansi_term")] pub fn InvalidStyle() -> Style { Style::new().fg(InvalidColor).bold() }
    #[cfg(feature="ansi_term")] pub fn InvalidParamStyle() -> Style { Style::new().fg(InvalidParamColor).bold() }
}

#[cfg(feature="ansi_term")]
use styles::{InvalidStyle, InvalidParamStyle};

pub fn no_path_noansi() -> String {
    format!(
        "{}",
        style::plain("No path specified.")
    )
}

#[cfg(not(feature="ansi_term"))]
pub use no_path_noansi as no_path;

#[cfg(feature="ansi_term")]
pub fn no_path() -> String {
    format!(
        "{}",
        style::apply("No path specified.", &InvalidStyle())
    )
}

pub fn non_existing_path_noansi(path: &str) -> String {
    format!(
        "Path {} does not exist.",
        style::plain(path)
    )
}

#[cfg(not(feature="ansi_term"))]
pub use non_existing_path_noansi as non_existing_path;

#[cfg(feature="ansi_term")]
pub fn non_existing_path(path: &str) -> String {
    let path = format!(
        "{}",
        style::apply(path, &InvalidParamStyle())
    );
    format!(
        "{}",
        style::apply(&format!("Path {} does not exist.", path), &InvalidStyle())
    )
}

pub fn not_a_file_path_noansi(path: &str) -> String {
    format!(
        "Path {} is not a file.",
        style::plain(path)
    )
}

#[cfg(not(feature="ansi_term"))]
pub use not_a_file_path_noansi as not_a_file_path;

#[cfg(feature="ansi_term")]
pub fn not_a_file_path(path: &str) -> String {
    let path = format!(
        "{}",
        style::apply(path, &InvalidParamStyle())
    );
    format!(
        "{}",
        style::apply(&format!("Path {} is not a file.", path), &InvalidStyle())
    )
}
