#[cfg(feature="ansi_term")] extern crate ansi_term;
#[cfg(feature="ansi_term")] use super::style as style;
use std::fmt;

#[cfg(feature="ansi_term")]
pub mod colors {
    #[cfg(feature="ansi_term")] use ansi_term::{Colour as Color, Colour::{
        Red, Blue, Green, Yellow, Cyan, Purple, White
    }};
    // alias colors
    #[cfg(feature="ansi_term")] pub const SearchPathColor: Color = Red;
    #[cfg(feature="ansi_term")] pub const CountColor: Color = Yellow;
    #[cfg(feature="ansi_term")] pub const DirInfoColor: Color = Cyan;
    #[cfg(feature="ansi_term")] pub const DirNameColor: Color = Cyan;
    #[cfg(feature="ansi_term")] pub const FileInfoColor: Color = Green;
    #[cfg(feature="ansi_term")] pub const FileNameColor: Color = White;
    #[cfg(feature="ansi_term")] pub const SymlinkDirInfoColor: Color = Purple;
    #[cfg(feature="ansi_term")] pub const SymlinkDirNameColor: Color = Purple;
    #[cfg(feature="ansi_term")] pub const SymlinkFileInfoColor: Color = Purple;
    #[cfg(feature="ansi_term")] pub const SymlinkFileNameColor: Color = Purple;
}

#[cfg(feature="ansi_term")]
pub mod styles {
    #[cfg(feature="ansi_term")] use ansi_term::Style;
    #[cfg(feature="ansi_term")] use super::colors::{
        CountColor, DirInfoColor, DirNameColor, FileInfoColor, FileNameColor,
        SearchPathColor, SymlinkDirInfoColor, SymlinkDirNameColor,
        SymlinkFileInfoColor, SymlinkFileNameColor
    };
    // alias styles
    #[cfg(feature="ansi_term")] pub fn SearchPathStyle() -> Style { Style::new().fg(SearchPathColor).bold() }
    #[cfg(feature="ansi_term")] pub fn CountStyle() -> Style { Style::new().fg(CountColor).bold() }
    #[cfg(feature="ansi_term")] pub fn DirInfoStyle() -> Style { Style::new().fg(DirInfoColor).bold() }
    #[cfg(feature="ansi_term")] pub fn DirNameStyle() -> Style { Style::new().fg(DirNameColor).bold() }
    #[cfg(feature="ansi_term")] pub fn FileInfoStyle() -> Style { Style::new().fg(FileInfoColor).bold() }
    #[cfg(feature="ansi_term")] pub fn FileNameStyle() -> Style { Style::new().fg(FileNameColor).bold() }
    #[cfg(feature="ansi_term")] pub fn SymlinkDirInfoStyle() -> Style { Style::new().fg(SymlinkDirInfoColor).bold() }
    #[cfg(feature="ansi_term")] pub fn SymlinkDirNameStyle() -> Style { Style::new().fg(SymlinkDirNameColor).bold() }
    #[cfg(feature="ansi_term")] pub fn SymlinkFileInfoStyle() -> Style { Style::new().fg(SymlinkFileInfoColor).bold() }
    #[cfg(feature="ansi_term")] pub fn SymlinkFileNameStyle() -> Style { Style::new().fg(SymlinkFileNameColor).bold() }
}

pub mod search_path {
    #[cfg(feature="ansi_term")]
    use super::{style, styles::SearchPathStyle};
    
    pub fn from_noansi(path: &str) -> String {
        format!(
            "Search path: {}",
            style::plain(path)
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use from_noansi as from;

    #[cfg(feature="ansi_term")]
    pub fn from(path: &str) -> String {
        format!(
            "Search path: {}",
            style::apply(path, &SearchPathStyle())
        )
    }
    
    pub fn with_canonical_path_noansi(path: &str, canonical_path: &str) -> String {
        format!(
            "Search path: {} -> {}",
            style::plain(path),
            style::plain(canonical_path)
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use with_canonical_path_noansi as with_canonical_path;

    #[cfg(feature="ansi_term")]
    pub fn with_canonical_path(path: &str, canonical_path: &str) -> String {
        format!(
            "Search path: {} -> {}",
            style::apply(path, &SearchPathStyle()),
            style::apply(canonical_path, &SearchPathStyle())
        )
    }
}

pub mod count {
    #[cfg(feature="ansi_term")]
    use super::{style, styles::CountStyle};

    pub fn from_noansi(count: usize) -> String {
        format!(
            "count {}",
            style::plain(&String::from(format!("{}", count)))
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use from_noansi as from;

    #[cfg(feature="ansi_term")]
    pub fn from(count: usize) -> String {
        format!(
            "count {}",
            style::apply(&String::from(format!("{}", count)), &CountStyle())
        )
    }
}

pub mod dir {
    #[cfg(feature="ansi_term")]
    use super::{style, styles};
    
    pub mod name {
        #[cfg(feature="ansi_term")]
        use super::{style, styles::DirNameStyle};
        
        pub fn from_noansi(name: &str) -> String {
            format!(
                "{}",
                style::plain(name)
            )
        }

        #[cfg(not(feature="ansi_term"))]
        pub use from_noansi as from;

        #[cfg(feature="ansi_term")]
        pub fn from(name: &str) -> String {
            format!(
                "{}",
                style::apply(name, &DirNameStyle())
            )
        }
    }
    
    pub mod info {
        #[cfg(feature="ansi_term")]
        use super::{style, styles::DirInfoStyle};
        const DIR_INFO: &'static str = "DIR";
        
        pub fn from_noansi() -> String {
            format!(
                "<{}>",
                style::plain(DIR_INFO)
            )
        }

        #[cfg(not(feature="ansi_term"))]
        pub use from_noansi as from;

        #[cfg(feature="ansi_term")]
        pub fn from() -> String {
            format!(
                "<{}>",
                style::apply(DIR_INFO, &DirInfoStyle())
            )
        }
    }
}

pub mod file {
    #[cfg(feature="ansi_term")]
    use super::{style, styles};
    
    pub mod name {
        #[cfg(feature="ansi_term")]
        use super::{style, styles::FileNameStyle};

        pub fn from_noansi(name: &str) -> String {
            format!(
                "{}",
                style::plain(name)
            )
        }

        #[cfg(not(feature="ansi_term"))]
        pub use from as from_noansi;

        #[cfg(feature="ansi_term")]
        pub fn from(name: &str) -> String {
            format!(
                "{}",
                style::apply(name, &FileNameStyle())
            )
        }
    }
    pub mod info {
        #[cfg(feature="ansi_term")]
        use super::{style, styles::FileInfoStyle};
        
        pub fn from_noansi(extension: &str) -> String {
            format!(
                "{}",
                style::plain(&extension.to_lowercase())
            )
        }

        #[cfg(not(feature="ansi_term"))]
        pub use from_noansi as from;

        #[cfg(feature="ansi_term")]
        pub fn from(extension: &str) -> String {
            format!(
                "{}",
                style::apply(&extension.to_lowercase(), &FileInfoStyle())
            )
        }
    }
}

pub mod symlink {
    #[cfg(feature="ansi_term")]
    use super::{style, styles};

    pub mod dir {
        #[cfg(feature="ansi_term")]
        use super::{style, styles};

        pub mod name {
            #[cfg(feature="ansi_term")]
            use super::{style, styles::SymlinkDirNameStyle};

            pub fn from_noansi(name: &str) -> String {
                format!(
                    "{}",
                    style::plain(name)
                )
            }
        
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;
        
            #[cfg(feature="ansi_term")]
            pub fn from(name: &str) -> String {
                format!(
                    "{}",
                    style::apply(name, &SymlinkDirNameStyle())
                )
            }
        }
        
        pub mod info {
            #[cfg(feature="ansi_term")]
            use super::{style, styles::SymlinkDirInfoStyle};
            const SYMD_INFO: &'static str = "SYMD";
            
            pub fn from_noansi() -> String {
                // format!(
                //     "<{}>:{}",
                //     style::plain(SYMD_INFO),
                //     super::super::super::dir::info::from_noansi()
                // )
                format!(
                    "<{}>",
                    style::plain(SYMD_INFO)
                )
            }
    
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;

            #[cfg(feature="ansi_term")]
            pub fn from() -> String {
                // format!(
                //     "<{}>:{}",
                //     style::apply(SYMD_INFO, &SymlinkDirInfoStyle()),
                //     super::super::super::dir::info::from()
                // )
                format!(
                    "<{}>",
                    style::apply(SYMD_INFO, &SymlinkDirInfoStyle())
                )
            }
        }
    }
    
    pub mod file {
        #[cfg(feature="ansi_term")]
        use super::{style, styles};

        pub mod name {
            #[cfg(feature="ansi_term")]
            use super::{style, styles::SymlinkFileNameStyle};

            pub fn from_noansi(name: &str) -> String {
                format!(
                    "{}",
                    style::plain(name)
                )
            }
        
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;
        
            #[cfg(feature="ansi_term")]
            pub fn from(name: &str) -> String {
                format!(
                    "{}",
                    style::apply(name, &SymlinkFileNameStyle())
                )
            }
        }
        
        pub mod info {
            #[cfg(feature="ansi_term")]
            use super::{style, styles::SymlinkFileInfoStyle};
            pub const SYM_INFO: &'static str = "SYM";
            
            pub fn from_noansi(extension: &str) -> String {
                format!(
                    "{}:{}",
                    style::plain(SYM_INFO),
                    super::super::super::file::info::from_noansi(extension)
                )
            }
    
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;
    
            #[cfg(feature="ansi_term")]
            pub fn from(extension: &str) -> String {
                format!(
                    "{}:{}",
                    style::apply(SYM_INFO, &SymlinkFileInfoStyle()),
                    super::super::super::file::info::from(extension)
                )
            }
        }
    }
    
    pub mod to_dir {
        pub mod name {
            pub fn from_noansi(name: &str, canonical_path: &str) -> String {
                format!(
                    "{} -> {}",
                    super::super::super::symlink::dir::name::from_noansi(name),
                    super::super::super::dir::name::from_noansi(canonical_path)
                )
            }
            
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;
        
            #[cfg(feature="ansi_term")]
            pub fn from(name: &str, canonical_path: &str) -> String {
                format!(
                    "{} -> {}",
                    super::super::super::symlink::dir::name::from(name),
                    super::super::super::dir::name::from(canonical_path)
                )
            }
        }
    }
    
    pub mod to_file {
        pub mod name {
            pub fn from_noansi(name: &str, canonical_path: &str) -> String {
                format!(
                    "{} -> {}",
                    super::super::super::symlink::file::name::from_noansi(name),
                    super::super::super::file::name::from_noansi(canonical_path)
                )
            }
            
            #[cfg(not(feature="ansi_term"))]
            pub use from_noansi as from;
        
            #[cfg(feature="ansi_term")]
            pub fn from(name: &str, canonical_path: &str) -> String {
                format!(
                    "{} -> {}",
                    super::super::super::symlink::file::name::from(name),
                    super::super::super::file::name::from(canonical_path)
                )
            }
        }
    }
}
