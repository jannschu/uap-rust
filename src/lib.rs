///! # ua-parser for rust
///! This is a user agent parser for Rust based on
///! [ua-parser](https://github.com/ua-parser).
///!
///! ## Usage example
///!
///! ```rust
///! use uap_rust::Client;
///! let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
///! let client = Client::new(agent);
///!
///! let browser = client.browser();
///! let os = client.os();
///! let device = client.device();
///!
///! println!("{:?}", browser);
///! println!("{:?}", os);
///! println!("{:?}", device);
///! ```
extern crate regex;

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate yaml_rust;

mod parser;
mod client;
mod result;

pub use client::Client;

/// `Browser` contains browser information.
#[derive(Debug, PartialEq, Eq)]
pub struct Browser {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

/// `OS` contains the operating system information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct OS {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

/// `Device` contains the device information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct Device {
    pub family: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub regex: Option<String>,
}

impl Default for Browser {
    fn default() -> Browser {
        Browser {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

impl Default for Device {
    fn default() -> Device {
        Device {
            family: "Other".to_string(),
            model: None,
            brand: None,
            regex: None,
        }
    }
}

impl Default for OS {
    fn default() -> OS {
        OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

#[cfg(test)]
mod yaml;

#[cfg(test)]
mod test {
    use yaml::*;
    use yaml_rust::{Yaml, YamlLoader};
    use std::io::prelude::*;
    use std::fs::File;

    use client::Client;
    use {Browser, Device, OS};

    #[test]
    fn test_simple_case() {
        let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
        let mut client = Client::new(&agent);
        assert_eq!(
            client.browser(),
            &Browser {
                family: "Mobile Safari".to_string(),
                major: Some("5".to_string()),
                minor: Some("1".to_string()),
                patch: None,
            }
        );
        assert_eq!(
            client.device(),
            &Device {
                family: "iPhone".to_string(),
                brand: Some("Apple".to_string()),
                model: Some("iPhone".to_string()),
                regex: Some("(iPhone)(?:;| Simulator;)".to_string()),
            }
        );
        assert_eq!(
            client.os(),
            &OS {
                family: "iOS".to_string(),
                major: Some("5".to_string()),
                minor: Some("1".to_string()),
                patch: Some("1".to_string()),
                patch_minor: None,
            }
        );
    }

    #[test]
    fn test_device() {
        let cases = load_test_data("uap-core/tests/test_device.yaml");
        for case in cases.iter() {
            let uas = from_map(case, "user_agent_string")
                .unwrap()
                .as_str()
                .unwrap();
            let mut client = Client::new(uas);
            let dev = client.device();
            assert_eq!(Some(&dev.family), case_get(&case, "family").as_ref());
            assert_eq!(dev.brand, case_get(&case, "brand"));
            assert_eq!(dev.model, case_get(&case, "model"));
        }
    }

    #[test]
    fn test_user_agent() {
        let cases = load_test_data("uap-core/tests/test_ua.yaml");
        for case in cases.iter() {
            let uas = from_map(case, "user_agent_string")
                .unwrap()
                .as_str()
                .unwrap();
            let mut client = Client::new(uas);
            let browser = client.browser();
            assert_eq!(Some(&browser.family), case_get(&case, "family").as_ref());
            assert_eq!(browser.major, case_get(&case, "major"));
            assert_eq!(browser.minor, case_get(&case, "minor"));
            assert_eq!(browser.patch, case_get(&case, "patch"));
        }
    }

    #[test]
    fn test_os() {
        let cases = load_test_data("uap-core/tests/test_os.yaml");
        for case in cases.iter() {
            let uas = case["user_agent_string"].as_str().unwrap();
            let mut client = Client::new(uas);
            let os = client.os();
            assert_eq!(Some(&os.family), case_get(&case, "family").as_ref());
            assert_eq!(os.major, case_get(&case, "major"));
            assert_eq!(os.minor, case_get(&case, "minor"));
            assert_eq!(os.patch, case_get(&case, "patch"));
            assert_eq!(os.patch_minor, case_get(&case, "patch_minor"));
        }
    }

    fn case_get<'a>(yaml: &'a Yaml, key: &str) -> Option<String> {
        let val = from_map(yaml, key).unwrap();
        if val.is_null() {
            None
        } else {
            val.as_str().map(|v| v.to_string() )
        }
    }

    fn load_test_data(path: &str) -> Vec<Yaml> {
        let mut test_file = File::open(path).unwrap();
        let mut yaml_str = String::new();
        let _ = test_file.read_to_string(&mut yaml_str).unwrap();
        let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
        let cases = (&docs[0])["test_cases"].as_vec().unwrap();
        cases.clone()
    }

}
