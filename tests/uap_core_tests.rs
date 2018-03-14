extern crate uap_rust;
extern crate yaml_rust;

mod yaml;

use yaml::*;
use yaml_rust::{Yaml, YamlLoader};
use std::io::prelude::*;
use std::fs::File;

use uap_rust::{Browser, Client, Device, OS};

use std::borrow::Cow;
use std::borrow::Cow::Borrowed;

#[test]
fn test_simple_case() {
    let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
    let client = Client::new(&agent);
    assert_eq!(
        client.browser(),
        &Browser {
            family: Borrowed("Mobile Safari"),
            major: Some(Borrowed("5")),
            minor: Some(Borrowed("1")),
            patch: None,
        }
    );
    assert_eq!(
        client.device(),
        &Device {
            family: Borrowed("iPhone"),
            brand: Some(Borrowed("Apple")),
            model: Some(Borrowed("iPhone")),
        }
    );
    assert_eq!(
        client.os(),
        &OS {
            family: Borrowed("iOS"),
            major: Some(Borrowed("5")),
            minor: Some(Borrowed("1")),
            patch: Some(Borrowed("1")),
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
        let client = Client::new(uas);
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
        let client = Client::new(uas);
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
        let client = Client::new(uas);
        let os = client.os();
        assert_eq!(Some(&os.family), case_get(&case, "family").as_ref());
        assert_eq!(os.major, case_get(&case, "major"));
        assert_eq!(os.minor, case_get(&case, "minor"));
        assert_eq!(os.patch, case_get(&case, "patch"));
        assert_eq!(os.patch_minor, case_get(&case, "patch_minor"));
    }
}

fn case_get<'a>(yaml: &'a Yaml, key: &str) -> Option<Cow<'a, str>> {
    let val = from_map(yaml, key).unwrap();
    if val.is_null() {
        None
    } else {
        val.as_str().map(|v| Borrowed(v))
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
