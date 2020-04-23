#[cfg(feature="ansi_term")] extern crate ansi_term;
use super::style as style;

#[cfg(feature="ansi_term")] #[allow(non_upper_case_globals)] #[allow(unused_imports)]
pub mod colors {
    #[cfg(feature="ansi_term")] use ansi_term::{Colour as Color, Colour::{
        Red, Blue, Green, Yellow, Cyan, Purple, White
    }};
    // alias colors
    #[cfg(feature="ansi_term")] pub const FriendlyNameColor: Color = Blue;
    #[cfg(feature="ansi_term")] pub const PackageNameColor: Color = Cyan;
    #[cfg(feature="ansi_term")] pub const AuthorsColor: Color = Green;
    #[cfg(feature="ansi_term")] pub const VersionColor: Color = Blue;
    #[cfg(feature="ansi_term")] pub const TargetOsColor: Color = Cyan;
    #[cfg(feature="ansi_term")] pub const TargetFamilyColor: Color = Green;
    #[cfg(feature="ansi_term")] pub const FeaturesColor: Color = Yellow;
    #[cfg(feature="ansi_term")] pub const FeaturesSeparatorColor: Color = White;
    #[cfg(feature="ansi_term")] pub const HomepageColor: Color = Blue;
    #[cfg(feature="ansi_term")] pub const RepositoryColor: Color = Purple;
}

#[cfg(feature="ansi_term")] #[allow(non_snake_case)]
pub mod styles {
    #[cfg(feature="ansi_term")] use ansi_term::Style;
    #[cfg(feature="ansi_term")] use super::colors::{
        FriendlyNameColor, PackageNameColor, AuthorsColor,
        VersionColor, TargetOsColor, TargetFamilyColor,
        FeaturesColor, FeaturesSeparatorColor, HomepageColor, RepositoryColor
    };
    // alias styles
    #[cfg(feature="ansi_term")] pub fn FriendlyNameStyle() -> Style { Style::new().fg(FriendlyNameColor).bold() }
    #[cfg(feature="ansi_term")] pub fn PackageNameStyle() -> Style { Style::new().fg(PackageNameColor).bold() }
    #[cfg(feature="ansi_term")] pub fn AuthorsStyle() -> Style { Style::new().fg(AuthorsColor).bold() }
    #[cfg(feature="ansi_term")] pub fn VersionStyle() -> Style { Style::new().fg(VersionColor).bold() }
    #[cfg(feature="ansi_term")] pub fn TargetOsStyle() -> Style { Style::new().fg(TargetOsColor).bold() }
    #[cfg(feature="ansi_term")] pub fn TargetFamilyStyle() -> Style { Style::new().fg(TargetFamilyColor).bold() }
    #[cfg(feature="ansi_term")] pub fn FeaturesStyle() -> Style { Style::new().fg(FeaturesColor).bold() }
    #[cfg(feature="ansi_term")] pub fn FeaturesSeparatorStyle() -> Style { Style::new().fg(FeaturesSeparatorColor).bold() }
    #[cfg(feature="ansi_term")] pub fn HomepageStyle() -> Style { Style::new().fg(HomepageColor).bold() }
    #[cfg(feature="ansi_term")] pub fn RepositoryStyle() -> Style { Style::new().fg(RepositoryColor).bold() }
}

mod friendy_name {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::FriendlyNameStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::friendly_name())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &FriendlyNameStyle())
        )
    }
}

pub use friendy_name::to_string_noansi as friendly_name_noansi;
pub use friendy_name::to_string as friendly_name;

mod package_name {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::PackageNameStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::name())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &PackageNameStyle())
        )
    }
}

pub use package_name::to_string_noansi as package_name_noansi;
pub use package_name::to_string as package_name;

mod authors {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::AuthorsStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::authors())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &AuthorsStyle())
        )
    }
}

pub use authors::to_string_noansi as authors_noansi;
pub use authors::to_string as authors;

mod version {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::VersionStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::version())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &VersionStyle())
        )
    }
}

pub use version::to_string_noansi as version_noansi;
pub use version::to_string as version;

mod target_os {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::TargetOsStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::target_os())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &TargetOsStyle())
        )
    }
}

pub use target_os::to_string_noansi as target_os_noansi;
pub use target_os::to_string as target_os;

mod target_family {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::TargetFamilyStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::target_family())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &TargetFamilyStyle())
        )
    }
}

pub use target_family::to_string_noansi as target_family_noansi;
pub use target_family::to_string as target_family;

mod features {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::{FeaturesStyle, FeaturesSeparatorStyle};
    pub const FEATURES_SEPARATOR: &'static str = ", ";
    
    pub fn collate_features_noansi() -> String {
        format!(
            "{}",
            style::plain(
                &features::vec_string_features_enabled()
                    .join(FEATURES_SEPARATOR)
            )
        )
    }
    
    #[cfg(not(feature="ansi_term"))]
    pub use collate_features_noansi as collate_features;

    #[cfg(feature="ansi_term")]
    pub fn collate_features() -> String {
        let mut v_s: Vec<String> = features::vec_string_features_enabled();
        for sf in v_s.iter_mut() {
            *sf = format!(
                "{}",
                style::apply(&sf, &FeaturesStyle())
            );
        }
        v_s.join(", ")
    }

    pub fn to_string_noansi() -> String {
        collate_features_noansi()
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        collate_features()
    }
}

pub use features::to_string_noansi as features_noansi;
pub use features::to_string as features;

mod homepage {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::HomepageStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::homepage())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &HomepageStyle())
        )
    }
}

pub use homepage::to_string_noansi as homepage_noansi;
pub use homepage::to_string as homepage;

mod repository {
    use crate::features;
    use super::style;
    #[cfg(feature="ansi_term")]
    use super::styles::RepositoryStyle;
    
    pub fn to_string_noansi() -> String {
        format!(
            "{}",
            style::plain(&features::repository())
        )
    }

    #[cfg(not(feature="ansi_term"))]
    pub use to_string_noansi as to_string;

    #[cfg(feature="ansi_term")]
    pub fn to_string() -> String {
        format!(
            "{}",
            style::apply(&to_string_noansi(), &RepositoryStyle())
        )
    }
}

pub use repository::to_string_noansi as repository_noansi;
pub use repository::to_string as repository;
