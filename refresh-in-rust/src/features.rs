use std::env;

pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const FRIENDLY_NAME: &'static str = "Tilde Tabulator";
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
pub const VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
pub const VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");
pub const VERSION_PRE: &'static str = env!("CARGO_PKG_VERSION_PRE");

#[cfg(target_family="unix")]            pub const TARGET_FAMILY: &'static str = "unix";
#[cfg(target_family="windows")]         pub const TARGET_FAMILY: &'static str = "windows";
#[cfg(not(any(target_family="unix", target_family="windows")))]
                                        pub const TARGET_FAMILY: &'static str = "unknown";
                                
#[cfg(target_os="windows")]             pub const TARGET_OS: &'static str = "windows";
#[cfg(target_os="macos")]               pub const TARGET_OS: &'static str = "macos";
#[cfg(target_os="ios")]                 pub const TARGET_OS: &'static str = "ios";
#[cfg(target_os="linux")]               pub const TARGET_OS: &'static str = "linux";
#[cfg(target_os="android")]             pub const TARGET_OS: &'static str = "android";
#[cfg(target_os="freebsd")]             pub const TARGET_OS: &'static str = "freebsd";
#[cfg(target_os="dragonfly")]           pub const TARGET_OS: &'static str = "dragonfly";
#[cfg(target_os="openbsd")]             pub const TARGET_OS: &'static str = "openbsd";
#[cfg(target_os="netbsd")]              pub const TARGET_OS: &'static str = "netbsd";
#[cfg(not(any(  target_os="windows",    target_os="macos",      target_os="ios",
                target_os="linux",      target_os="android",    target_os="freebsd",
                target_os="dragonfly",  target_os="openbsd",    target_os="netbsd")))]
                                        pub const TARGET_OS: &'static str = "unknown";

#[cfg(feature="ansi")]                  pub const USE_FEATURE_ANSI: bool = true;
#[cfg(not(feature="ansi"))]             pub const USE_FEATURE_ANSI: bool = false;

#[cfg(feature="ansi_term")]             pub const USE_FEATURE_ANSI_TERM: bool = true;
#[cfg(not(feature="ansi_term"))]        pub const USE_FEATURE_ANSI_TERM: bool = false;

#[cfg(feature="async_io")]              pub const USE_FEATURE_ASYNC_IO: bool = true;
#[cfg(not(feature="async_io"))]         pub const USE_FEATURE_ASYNC_IO: bool = false;

#[cfg(feature="crossterm")]             pub const USE_FEATURE_CROSSTERM: bool = true;
#[cfg(not(feature="crossterm"))]        pub const USE_FEATURE_CROSSTERM: bool = false;

#[cfg(feature="csv")]                   pub const USE_FEATURE_CSV: bool = true;
#[cfg(not(feature="csv"))]              pub const USE_FEATURE_CSV: bool = false;

#[cfg(feature="http")]                  pub const USE_FEATURE_HTTP: bool = true;
#[cfg(not(feature="http"))]             pub const USE_FEATURE_HTTP: bool = false;

#[cfg(feature="json")]                  pub const USE_FEATURE_JSON: bool = true;
#[cfg(not(feature="json"))]             pub const USE_FEATURE_JSON: bool = false;

#[cfg(feature="default")]               pub const USE_FEATURE_DEFAULT: bool = true;
#[cfg(not(feature="default"))]          pub const USE_FEATURE_DEFAULT: bool = false;

#[cfg(feature="responsive_tui")]        pub const USE_FEATURE_RESPONSIVE_TUI: bool = true;
#[cfg(not(feature="responsive_tui"))]   pub const USE_FEATURE_RESPONSIVE_TUI: bool = false;

#[cfg(feature="serde")]                 pub const USE_FEATURE_SERDE: bool = true;
#[cfg(not(feature="serde"))]            pub const USE_FEATURE_SERDE: bool = false;

#[cfg(feature="serde_json")]            pub const USE_FEATURE_SERDE_JSON: bool = true;
#[cfg(not(feature="serde_json"))]       pub const USE_FEATURE_SERDE_JSON: bool = false;

#[cfg(feature="tokio")]                 pub const USE_FEATURE_TOKIO: bool = true;
#[cfg(not(feature="tokio"))]            pub const USE_FEATURE_TOKIO: bool = false;

#[cfg(feature="termion")]               pub const USE_FEATURE_TERMION: bool = true;
#[cfg(not(feature="termion"))]          pub const USE_FEATURE_TERMION: bool = false;

#[cfg(feature="web_client")]            pub const USE_FEATURE_WEB_CLIENT: bool = true;
#[cfg(not(feature="web_client"))]       pub const USE_FEATURE_WEB_CLIENT: bool = false;

#[cfg(all(feature="none", not(any(feature="ansi", feature="default", feature="web_client", feature="async_io", feature="responsive_tui"))))]
                                        pub const USE_FEATURE_NONE: bool = true;
#[cfg(any(not(feature="none"), feature="ansi", feature="default", feature="web_client", feature="async_io", feature="responsive_tui"))]
                                        pub const USE_FEATURE_NONE: bool = false;

pub enum Feature {
    Defaults,
    Ansi,
    AnsiTerm,
    AsyncIo,
    Crossterm,
    Csv,
    Http,
    Json,
    None,
    ResponsiveTui,
    Serde,
    SerdeJson,
    Termion,
    Tokio,
    WebClient,
}

impl Feature {
    pub fn to_str(&self) -> &str {
        match *self {
            Feature::Defaults => "default",
            Feature::Ansi => "ansi",
            Feature::AnsiTerm => "ansi_term",
            Feature::AsyncIo => "async_io",
            Feature::Crossterm => "crossterm",
            Feature::Csv => "csv",
            Feature::Http => "http",
            Feature::Json => "json",
            Feature::None => "none",
            Feature::ResponsiveTui => "responsive_tui",
            Feature::Serde => "serde",
            Feature::SerdeJson => "serde_json",
            Feature::Termion => "termion",
            Feature::Tokio => "tokio",
            Feature::WebClient => "web_client",
        }
    }
    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

pub fn authors() -> String {
    String::from(AUTHORS)
}

pub fn description() -> String {
    String::from(DESCRIPTION)
}

pub fn homepage() -> String {
    String::from(HOMEPAGE)
}

pub fn friendly_name() -> String {
    String::from(FRIENDLY_NAME)
}

pub fn name() -> String {
    String::from(NAME)
}

pub fn repository() -> String {
    String::from(REPOSITORY)
}

pub fn version() -> String {
    String::from(VERSION)
}

pub fn target_family() -> String {
    String::from(TARGET_FAMILY)
}

pub fn target_os() -> String {
    String::from(TARGET_OS)
}

pub fn vec_features_enabled() -> Vec<Feature> {
    let mut v_f: Vec<Feature> = Vec::new();
    if USE_FEATURE_DEFAULT {
        v_f.append(&mut vec![Feature::Defaults]);
    }
    if USE_FEATURE_ANSI {
        v_f.append(&mut vec![Feature::Ansi]);
    }
    if USE_FEATURE_ANSI_TERM {
        v_f.append(&mut vec![Feature::AnsiTerm]);
    }
    if USE_FEATURE_ASYNC_IO {
        v_f.append(&mut vec![Feature::AsyncIo]);
    }
    if USE_FEATURE_CROSSTERM {
        v_f.append(&mut vec![Feature::Crossterm]);
    }
    if USE_FEATURE_CSV {
        v_f.append(&mut vec![Feature::Csv]);
    }
    if USE_FEATURE_HTTP {
        v_f.append(&mut vec![Feature::Http]);
    }
    if USE_FEATURE_JSON {
        v_f.append(&mut vec![Feature::Json]);
    }
    if USE_FEATURE_RESPONSIVE_TUI {
        v_f.append(&mut vec![Feature::ResponsiveTui]);
    }
    if USE_FEATURE_SERDE {
        v_f.append(&mut vec![Feature::Serde]);
    }
    if USE_FEATURE_SERDE_JSON {
        v_f.append(&mut vec![Feature::SerdeJson]);
    }
    if USE_FEATURE_TERMION {
        v_f.append(&mut vec![Feature::Termion]);
    }
    if USE_FEATURE_TOKIO {
        v_f.append(&mut vec![Feature::Tokio]);
    }
    if USE_FEATURE_WEB_CLIENT {
        v_f.append(&mut vec![Feature::WebClient]);
    }
    if USE_FEATURE_NONE {
        v_f.append(&mut vec![Feature::None]);
    }
    v_f
}

pub fn vec_string_features_enabled() -> Vec<String> {
    let v_f = vec_features_enabled();
    let mut v_s: Vec<String> = Vec::with_capacity(v_f.len());
    for f in v_f {
        v_s.append(&mut vec![f.to_string()]);
    }
    v_s.sort();
    v_s
}
