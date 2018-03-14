//! # ua-parser for rust
//!
//! This is a web browser user agent parser for Rust based on
//! [ua-parser](https://github.com/ua-parser).
//!
//! ## Usage example
//!
//! ```rust
//! use uap_rust::Client;
//! let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
//! let client = Client::new(agent);
//!
//! let browser = client.browser();
//! let os = client.os();
//! let device = client.device();
//!
//! println!("{:?}", browser);
//! println!("{:?}", os);
//! println!("{:?}", device);
//! ```
extern crate regex;

extern crate lazy_init;
#[macro_use]
extern crate lazy_static;
extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

mod parser;
mod client;

pub use client::Client;

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
            family: Cow::Borrowed(DEFAULT_NAME),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

impl<'a> Default for Device<'a> {
    fn default() -> Device<'a> {
        Device {
            family: Cow::Borrowed(DEFAULT_NAME),
            model: None,
            brand: None,
        }
    }
}

impl<'a> Default for OS<'a> {
    fn default() -> OS<'a> {
        OS {
            family: Cow::Borrowed(DEFAULT_NAME),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}
