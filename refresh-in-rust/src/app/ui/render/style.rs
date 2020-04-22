#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use ansi_term::Style;

#[cfg(feature="ansi_term")]
pub fn apply(text: &str, style: &Style) -> String {
    style.paint(text).to_string()
}

pub fn plain(text: &str) -> String {
    String::from(text)
}
