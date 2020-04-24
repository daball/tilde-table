#![allow(unused_imports)]
#[cfg(feature="ansi_term")] extern crate ansi_term;
use super::style as style;

#[cfg(feature="ansi_term")] #[allow(non_upper_case_globals)] #[allow(unused_imports)]
pub mod colors {
    #[cfg(feature="ansi_term")] use ansi_term::{Colour as Color, Colour::{
        Red, Green
    }};
    // alias colors
    #[cfg(feature="ansi_term")] pub const InvalidColor: Color = Red;
    #[cfg(feature="ansi_term")] pub const ValidColor: Color = Green;
}

#[cfg(feature="ansi_term")] #[allow(non_snake_case)]
pub mod styles {
    #[cfg(feature="ansi_term")] use ansi_term::Style;
    #[cfg(feature="ansi_term")] use super::colors::{
        InvalidColor, ValidColor
    };
    // alias styles
    #[cfg(feature="ansi_term")] pub fn InvalidStyle() -> Style { Style::new().fg(InvalidColor).bold() }
    #[cfg(feature="ansi_term")] pub fn ValidStyle() -> Style { Style::new().fg(ValidColor).bold() }
}

#[cfg(feature="ansi_term")]
use styles::{InvalidStyle, ValidStyle};

const HELP_CMD: &'static str = "help";

pub fn invalid_noansi(cmd: &str) -> String {
    format!(
        "Invalid command {}. Enter {} to list possible commands.",
        style::plain(cmd),
        style::plain(HELP_CMD)
    )
}

#[cfg(not(feature="ansi_term"))]
pub use invalid_noansi as invalid;

#[cfg(feature="ansi_term")]
pub fn invalid(cmd: &str) -> String {
    format!(
        "Invalid command {}. Enter {} to list possible commands.",
        style::apply(cmd, &InvalidStyle()),
        style::apply(HELP_CMD, &ValidStyle())
    )
}
