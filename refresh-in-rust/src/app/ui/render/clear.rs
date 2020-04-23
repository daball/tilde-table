use std::io::{stdout, Write};
pub use crate::app::handlers::utils::REQUIRES_ANSI;

pub fn clear_screen_noansi() -> String {
    format!("{}\n", String::from(REQUIRES_ANSI))
}

#[cfg(feature="ansi_term")]
pub fn clear_screen() -> String {
    stdout().write(&[0o033, b'c'][..]);
    String::from("")
}

#[cfg(not(feature="ansi_term"))]
pub use clear_screen_noansi as clear_screen;
