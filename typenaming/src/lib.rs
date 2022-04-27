#![deny(missing_docs)]
//! This crate allows to derive the name of a type, together with some more basic information

/// This module implements TypeNameable for some standard types
#[doc(hidden)]
pub mod std_impl;

mod type_info;
mod typenameable;
pub use crate::type_info::TypeInfo;
pub use crate::typenameable::TypeNameable;

/// This is used by the derive macro, but shall not be used otherwise
#[doc(hidden)]
pub use rustc_version::version as rustc_version;

/// This represents a semver-Version.
/// Re-Exported from semver [<https://docs.rs/semver>]
pub use semver::Version;
pub use typenaming_derive::TypeNameable;

/// This is a helper function which allows to easily produce a SemverVersion. It is used in the derived code.
#[doc(hidden)]
pub fn new_semver_version(
    major: &'static str,
    minor: &'static str,
    patch: &'static str,
    pre: &'static str,
) -> semver::Version {
    semver::Version {
        major: major
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse major version: {major}")),
        minor: minor
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse minor version: {minor}")),
        patch: patch
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse patch version: {patch}")),
        pre: <semver::Prerelease as std::str::FromStr>::from_str(pre)
            .unwrap_or_else(|_| panic!("Failed to parse pre version: {pre}")),
        build: semver::BuildMetadata::EMPTY,
    }
}
