#![allow(unused_imports)]
#[cfg(feature="ansi_term")] extern crate ansi_term;
use super::style as style;

#[cfg(feature="ansi_term")] #[allow(non_upper_case_globals)] #[allow(unused_imports)]
pub mod colors {
    #[cfg(feature="ansi_term")] use ansi_term::{Colour as Color, Colour::{
        Blue
    }};
    // alias colors
    #[cfg(feature="ansi_term")] pub const GoodbyeColor: Color = Blue;
}

#[cfg(feature="ansi_term")] #[allow(non_snake_case)]
pub mod styles {
    #[cfg(feature="ansi_term")] use ansi_term::Style;
    #[cfg(feature="ansi_term")] use super::colors::{
        GoodbyeColor
    };
    // alias styles
    #[cfg(feature="ansi_term")] pub fn GoodbyeStyle() -> Style { Style::new().fg(GoodbyeColor).bold() }
}

use styles::GoodbyeStyle;

pub fn goodbye_noansi() -> String {
    format!("{}", style::plain("Later!"))
}

#[cfg(not(feature="ansi_term"))]
pub use goodbye_noansi as goodbye;

#[cfg(feature="ansi_term")]
pub fn goodbye() -> String {
    format!("{}", style::apply(&goodbye_noansi(), &GoodbyeStyle()))
}
