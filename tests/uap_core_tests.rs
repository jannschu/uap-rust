#[macro_use]
extern crate lazy_static;
extern crate rmp_serde as rmps;
#[macro_use]
extern crate serde_derive;

extern crate uap_rust;

use uap_rust::{Browser, Client, Device, OS};

mod test_data;

#[test]
fn test_simple_case() {
    let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
    let client = Client::new(agent);
    assert_eq!(
        client.browser(),
        &Browser {
            family: "Mobile Safari".into(),
            major: Some("5".into()),
            minor: Some("1".into()),
            patch: None,
        }
    );
    assert_eq!(
        client.device(),
        &Device {
            family: "iPhone".into(),
            brand: Some("Apple".into()),
            model: Some("iPhone".into()),
        }
    );
    assert_eq!(
        client.os(),
        &OS {
            family: "iOS".into(),
            major: Some("5".into()),
            minor: Some("1".into()),
            patch: Some("1".into()),
            patch_minor: None,
        }
    );
}

#[test]
fn test_device() {
    let cases = test_data::parse_device_test_cases();
    for &(uas, ref test_device) in cases.iter() {
        let client = Client::new(uas);
        let dev = client.device();
        assert_eq!(dev, test_device);
    }
}

#[test]
fn test_browser() {
    let cases = test_data::parse_browser_test_cases();
    for &(uas, ref test_browser) in cases.iter() {
        let client = Client::new(uas);
        let browser = client.browser();
        assert_eq!(browser, test_browser);
    }
}

#[test]
fn test_os() {
    let cases = test_data::parse_os_test_cases();
    for &(uas, ref test_os) in cases.iter() {
        let client = Client::new(uas);
        let os = client.os();
        assert_eq!(os, test_os);
    }
}

#[test]
fn test_is_bot() {
    let cases = test_data::parse_device_test_cases();
    for &(uas, ref test_device) in cases.iter() {
        assert_eq!(Client::new(uas).is_bot(), &*test_device.family == "Spider");
    }
}
