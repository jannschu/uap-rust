use client::Client;

mod browser;
mod os;
mod device;
mod regex;

pub use parser::os::OS;
pub use parser::device::Device;
pub use parser::browser::Browser;

pub use regex::{parse_browser, parse_device, parse_os};
