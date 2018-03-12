/*!
#ua-parser for rust
This is a user agent parser for Rust based on
[ua-parser](https://github.com/ua-parser).

##Usage example

```rust
use uap_rust::parser::Parser;
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
let p = Parser::new().unwrap();
let c = p.parse(agent);

println!("{:?}",c);
 //Output: Client { user_agent: UserAgent { family: "Mobile Safari", major: Some("5"), minor: Some("1"), patch: None }, os: OS { family: "iOS", major: Some("5"), minor: Some("1"), patch: Some("1"), patch_minor: None }, device: Device { family: "iPhone", brand: Some("Apple"), model: Some("iPhone") } }
```
*/

extern crate regex;
extern crate yaml_rust;

#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod client;
pub mod ua;
pub mod os;
pub mod device;
mod result;
mod yaml;

use regex::{Captures, Error, Regex, RegexBuilder};

lazy_static! {
    static ref RE_REPLACE: Regex = Regex::new(r"\$\d+").unwrap();
}

fn replace_matches(s: &str, m: &Captures) -> Option<String> {
    let s = RE_REPLACE
        .replace_all(&s, |c: &Captures| {
            if let Some(i) = c[0][1..].parse().ok() {
                m.get(i).map(|x| x.as_str()).unwrap_or("")
            } else {
                ""
            }.to_string()
        })
        .trim()
        .to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn get_or_none(c: &Captures, i: usize) -> Option<String> {
    if let Some(group) = c.get(i) {
        let s = group.as_str().to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    }
}

fn build_uap_regexp(pattern: &str, flags: Option<&String>) -> Result<Regex, Error> {
    let mut pattern = pattern
        .replace(r"\ ", r" ")
        .replace(r"\/", r"/")
        .replace(r"\!", r"!");
    if let Some(flags) = flags {
        pattern = format!("(?{}){}", flags, pattern);
    }
    let mut builder = RegexBuilder::new(&pattern);
    // We need to increase this limit for the bot
    // patterns used by uap-core.
    // Fixed by https://github.com/ua-parser/uap-core/pull/62.
    builder.nest_limit(100);
    builder.build()
}

#[test]
fn test_replace_matches() {
    let re = Regex::new(r"Ok (\d+) (\d+)").unwrap();
    let captures = re.captures("Ok 1 2").unwrap();
    assert_eq!(
        replace_matches("$2 $1 $2", &captures),
        Some("2 1 2".to_string())
    );
}

#[test]
fn test_regex() {
    let re = build_uap_regexp(r#"(?:\/[A-Za-z0-9\.]+)? *([A-Za-z0-9 \-_\!\[\]:]*(?:[Aa]rchiver|[Ii]ndexer|[Ss]craper|[Bb]ot|[Ss]pider|[Cc]rawl[a-z]*))/(\d+)(?:\.(\d+)(?:\.(\d+))?)?"#, None).unwrap();
    assert!(re.is_match(r"449 Overture-WebCrawler/3.8/Fresh (atw-crawler at fast dot no; http://fast.no/support/crawler.asp"));
}

#[cfg(test)]
mod test {
    use parser;
    use client::Client;
    use ua::UserAgent;
    use device::Device;
    use os::OS;
    use yaml::*;
    use yaml_rust::{Yaml, YamlLoader};
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn test_basic_au() {
        let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3".to_string();
        let p = parser::Parser::new().unwrap();
        let c = p.parse(agent);
        assert_eq!(
            Client {
                user_agent: UserAgent {
                    family: "Mobile Safari".to_string(),
                    major: Some("5".to_string()),
                    minor: Some("1".to_string()),
                    patch: None,
                },
                device: Device {
                    family: "iPhone".to_string(),
                    brand: Some("Apple".to_string()),
                    model: Some("iPhone".to_string()),
                    regex: Some("(iPhone)(?:;| Simulator;)".to_string()),
                },
                os: OS {
                    family: "iOS".to_string(),
                    major: Some("5".to_string()),
                    minor: Some("1".to_string()),
                    patch: Some("1".to_string()),
                    patch_minor: None,
                },
            },
            c
        );
    }

    #[test]
    fn test_device() {
        let p = parser::Parser::new().unwrap();
        assert!(!parser::DP.is_empty());
        let cases = load_cases("src/uap-core/tests/test_device.yaml");
        for case in cases.iter() {
            let uas = from_map(case, "user_agent_string")
                .unwrap()
                .as_str()
                .unwrap();
            let client = p.parse(uas.to_string());
            let dev = client.device;
            assert_eq!(Some(dev.family), case_get(&case, "family"));
            assert_eq!(dev.brand, case_get(&case, "brand"));
            assert_eq!(dev.model, case_get(&case, "model"));
        }
    }

    #[test]
    fn test_user_agent() {
        let p = parser::Parser::new().unwrap();
        assert!(!parser::UAP.is_empty());
        let cases = load_cases("src/uap-core/tests/test_ua.yaml");
        for case in cases.iter() {
            let uas = from_map(case, "user_agent_string")
                .unwrap()
                .as_str()
                .unwrap();
            let client = p.parse(uas.to_string());
            let ua = client.user_agent;
            println!("{}", uas);
            assert_eq!(Some(ua.family), case_get(&case, "family"));
            assert_eq!(ua.major, case_get(&case, "major"));
            assert_eq!(ua.minor, case_get(&case, "minor"));
            assert_eq!(ua.patch, case_get(&case, "patch"));
        }
    }

    #[test]
    fn test_os() {
        let p = parser::Parser::new().unwrap();
        assert!(!parser::OSP.is_empty());
        let cases = load_cases("src/uap-core/tests/test_os.yaml");
        for case in cases.iter() {
            let uas = case["user_agent_string"].as_str().unwrap();
            let client = p.parse(uas.to_string());
            let os = client.os;
            assert_eq!(Some(os.family), case_get(&case, "family"));
            assert_eq!(os.major, case_get(&case, "major"));
            assert_eq!(os.minor, case_get(&case, "minor"));
            assert_eq!(os.patch, case_get(&case, "patch"));
            assert_eq!(os.patch_minor, case_get(&case, "patch_minor"));
        }
    }

    fn case_get<'a>(yaml: &'a Yaml, key: &str) -> Option<String> {
        let val = from_map(yaml, key).unwrap();
        println!("key={} val={:?}", key, val);
        if val.is_null() {
            None
        } else {
            val.as_str().map(|c| c.to_string())
        }
    }

    fn load_cases(path: &str) -> Vec<Yaml> {
        let mut test_file = File::open(path).unwrap();
        let mut yaml_str = String::new();
        let _ = test_file.read_to_string(&mut yaml_str).unwrap();
        let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
        let cases = (&docs[0])["test_cases"].as_vec().unwrap();
        cases.clone()
    }

}
