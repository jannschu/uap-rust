use rmps;

use std::borrow::Cow;
use std::borrow::Cow::Borrowed;

use uap_rust::{Browser, Device, OS};

lazy_static! {
    static ref BROWSER_TEST: Vec<&'static [u8]> = {
        vec![
            include_bytes!("../../resources/test_resources/firefox_user_agent_strings.msgpack"),
            include_bytes!("../../resources/tests/test_ua.msgpack"),
            include_bytes!("../../resources/test_resources/pgts_browser_list.msgpack"),
            include_bytes!("../../resources/test_resources/opera_mini_user_agent_strings.msgpack"),
            include_bytes!("../../resources/test_resources/podcasting_user_agent_strings.msgpack"),
        ]
    };

    static ref OS_TEST: Vec<&'static [u8]> = {
        vec![
            include_bytes!("../../resources/tests/test_os.msgpack"),
            include_bytes!("../../resources/test_resources/additional_os_tests.msgpack"),
        ]
    };

    static ref DEVICE_TEST: Vec<&'static [u8]> = {
        vec![include_bytes!("../../resources/tests/test_device.msgpack")]
    };
}

fn borrowed(u: Option<&str>) -> Option<Cow<str>> {
    match u {
        None => None,
        Some(s) => Some(Borrowed(s)),
    }
}

#[derive(Debug, Deserialize)]
struct BrowserTestCase<'a> {
    user_agent_string: &'a str,
    #[serde(borrow)]
    family: &'a str,
    #[serde(borrow)]
    major: Option<&'a str>,
    #[serde(borrow)]
    minor: Option<&'a str>,
    #[serde(borrow)]
    patch: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct BrowserTestCases<'a> {
    #[serde(borrow)]
    test_cases: Vec<BrowserTestCase<'a>>,
}

pub fn parse_browser_test_cases() -> Vec<(&'static str, Browser<'static>)> {
    let mut all_cases = Vec::new();
    for cases in BROWSER_TEST.iter() {
        let mut cases = rmps::from_slice::<BrowserTestCases<'static>>(cases).unwrap();
        all_cases.append(&mut cases.test_cases);
    }
    all_cases
        .iter()
        .map(|case| {
            (
                case.user_agent_string,
                Browser {
                    family: Borrowed(case.family),
                    major: borrowed(case.major),
                    minor: borrowed(case.minor),
                    patch: borrowed(case.patch),
                },
            )
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct OSTestCase<'a> {
    user_agent_string: &'a str,
    #[serde(borrow)]
    family: &'a str,
    #[serde(borrow)]
    major: Option<&'a str>,
    #[serde(borrow)]
    minor: Option<&'a str>,
    #[serde(borrow)]
    patch: Option<&'a str>,
    #[serde(borrow)]
    patch_minor: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct OSTestCases<'a> {
    #[serde(borrow)]
    test_cases: Vec<OSTestCase<'a>>,
}

pub fn parse_os_test_cases() -> Vec<(&'static str, OS<'static>)> {
    let mut all_cases = Vec::new();
    for cases in OS_TEST.iter() {
        let mut cases = rmps::from_slice::<OSTestCases<'static>>(cases).unwrap();
        all_cases.append(&mut cases.test_cases);
    }
    all_cases
        .iter()
        .map(|case| {
            (
                case.user_agent_string,
                OS {
                    family: Borrowed(case.family),
                    major: borrowed(case.major),
                    minor: borrowed(case.minor),
                    patch: borrowed(case.patch),
                    patch_minor: borrowed(case.patch_minor),
                },
            )
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct DeviceTestCase<'a> {
    user_agent_string: &'a str,
    #[serde(borrow)]
    family: &'a str,
    #[serde(borrow)]
    brand: Option<&'a str>,
    #[serde(borrow)]
    model: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct DeviceTestCases<'a> {
    #[serde(borrow)]
    test_cases: Vec<DeviceTestCase<'a>>,
}

pub fn parse_device_test_cases() -> Vec<(&'static str, Device<'static>)> {
    let mut all_cases = Vec::new();
    for cases in DEVICE_TEST.iter() {
        let mut cases = rmps::from_slice::<DeviceTestCases<'static>>(cases).unwrap();
        all_cases.append(&mut cases.test_cases);
    }
    all_cases
        .iter()
        .map(|case| {
            (
                case.user_agent_string,
                Device {
                    family: Borrowed(case.family),
                    brand: borrowed(case.brand),
                    model: borrowed(case.model),
                },
            )
        })
        .collect()
}
