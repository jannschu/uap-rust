//! # uap-rust
//!
//! This is a web browser user agent parser for Rust based on
//! [ua-parser](https://github.com/ua-parser).
//!
//! The crate offers parsers with optional thread
//! safety. The regular expressions for detecting the browser, device or os
//! are only run when requested. Also, parsers can be chosen to own or
//! borrow the user agent string. We try to avoid string allocation as much
//! as possible.
//!
//! ## Usage example
//!
//! ```rust
//! use uap_rust::unsync::BorrowingParser as Parser;
//! let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
//! let parser = Parser::new(agent);
//!
//! let browser = parser.browser();
//! assert_eq!(browser.family, "Mobile Safari");
//! let browser_version = browser.version().unwrap();
//! assert_eq!(browser_version.major, 5);
//! assert_eq!(browser_version.minor, 1);
//!
//! let os = parser.os();
//! assert_eq!(os.family, "iOS");
//! let os_version = os.version().unwrap();
//! assert_eq!(os_version.major, 5);
//! assert_eq!(os_version.minor, 1);
//!
//! let device = parser.device();
//! assert_eq!(device.family, "iPhone");
//! assert_eq!(device.brand.as_ref().unwrap(), "Apple");
//! ```
//!
//! To use a `Arc<str>` as a user agent:
//!
//! ```rust
//! # use std::sync::Arc;
//! use uap_rust::sync::OwningParser as Parser;
//! let agent: Arc<str> = Arc::from("Mozilla/5.0 ...");
//! let parser = Parser::new(agent.clone());
//! ```
//!
//! In the example above `agent` can also be a `String`. To use `Rc`,
//! additionally replace `unsync` by `sync`.
//!
//! The `OwningParser` variant is a convenience wrapper around
//! `BorrowingParser` to allow storing the user agent along the parser, which
//! is not trivial, since rust does not understand self-referential structs.
#[macro_use]
extern crate rental;

use semver_parser::version::{parse as parse_version, Version};
use std::borrow::Cow;
use std::str::FromStr;

mod parser;
mod ua_core;

pub use crate::parser::sync;
pub use crate::parser::unsync;
pub use crate::parser::UserAgentInformation;

/// `Browser` contains browser information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct Browser<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
}

/// `OS` contains the operating system information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct OS<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
    pub patch_minor: Option<Cow<'a, str>>,
}

/// `Device` contains the device information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct Device<'a> {
    pub family: Cow<'a, str>,
    pub brand: Option<Cow<'a, str>>,
    pub model: Option<Cow<'a, str>>,
}

static DEFAULT_NAME: &str = "Other";

impl<'a> Default for Browser<'a> {
    fn default() -> Browser<'a> {
        Browser {
            family: DEFAULT_NAME.into(),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

impl<'a> Default for Device<'a> {
    fn default() -> Device<'a> {
        Device {
            family: DEFAULT_NAME.into(),
            model: None,
            brand: None,
        }
    }
}

impl<'a> Default for OS<'a> {
    fn default() -> OS<'a> {
        OS {
            family: DEFAULT_NAME.into(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

macro_rules! default_parse {
    ($obj:ident, $name:ident, $default:ident) => {
        $obj.$name
            .as_ref()
            .and_then(|c| c.parse().ok())
            .unwrap_or($default)
    };
}

impl<'a> Browser<'a> {
    pub fn version(&self) -> Option<Version> {
        match (&self.major, &self.minor, &self.patch) {
            (Some(major), Some(minor), Some(patch)) => {
                parse_version(&format!("{}.{}.{}", major, minor, patch)).ok()
            }
            (Some(major), Some(minor), None) => {
                parse_version(&format!("{}.{}.0", major, minor)).ok()
            }
            (Some(major), None, None) => parse_version(&format!("{}.0.0", major)).ok(),
            _ => parse_version("").ok(),
        }
    }

    pub fn major_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, major, default)
    }

    pub fn minor_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, minor, default)
    }

    pub fn patch_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, patch, default)
    }
}

impl<'a> OS<'a> {
    pub fn version(&self) -> Option<Version> {
        match (&self.major, &self.minor, &self.patch, &self.patch_minor) {
            (Some(major), Some(minor), Some(patch), Some(patch_minor)) => {
                parse_version(&format!("{}.{}.{}-{}", major, minor, patch, patch_minor)).ok()
            }
            (Some(major), Some(minor), Some(patch), None) => {
                parse_version(&format!("{}.{}.{}", major, minor, patch)).ok()
            }
            (Some(major), Some(minor), None, None) => {
                parse_version(&format!("{}.{}.0", major, minor)).ok()
            }
            (Some(major), None, None, None) => parse_version(&format!("{}", major)).ok(),
            _ => parse_version("").ok(),
        }
    }

    pub fn major_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, major, default)
    }

    pub fn minor_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, minor, default)
    }

    pub fn patch_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, patch, default)
    }

    pub fn patch_minor_or<T: FromStr>(&self, default: T) -> T {
        default_parse!(self, patch_minor, default)
    }
}
