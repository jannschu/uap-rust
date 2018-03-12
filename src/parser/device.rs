use yaml_rust::Yaml;
use yaml;
use regex::Regex;

use {build_uap_regexp, get_or_none, replace_matches};

///`Device` contains the device information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct Device {
    pub family: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub regex: Option<String>,
}

impl Device {
    pub(crate) fn new() -> Device {
        Device {
            family: "Other".to_string(),
            model: None,
            brand: None,
            regex: None,
        }
    }
}

#[derive(Debug)]
pub(super) struct DeviceParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl DeviceParser {
    pub fn from_yaml(y: &Yaml) -> Option<DeviceParser> {
        let regex_flag = yaml::string_from_map(y, "regex_flag");
        yaml::string_from_map(y, "regex")
            .and_then(|r| build_uap_regexp(&r, regex_flag.as_ref()).ok())
            .map(|r| DeviceParser {
                regex: r,
                family: yaml::string_from_map(y, "device_replacement"),
                brand: yaml::string_from_map(y, "brand_replacement"),
                model: yaml::string_from_map(y, "model_replacement"),
            })
    }

    pub fn parse(&self, agent: &str) -> Option<Device> {
        self.regex.captures(agent).map(|c| {
            let family = self.family
                .clone()
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(&f, &c))
                .unwrap_or_else(|| "Other".to_string());
            let brand = self.brand.clone().and_then(|m| replace_matches(&m, &c));
            let model = self.model
                .clone()
                .map_or_else(|| get_or_none(&c, 1), |m| replace_matches(&m, &c));
            Device {
                family: family,
                brand: brand,
                model: model,
                regex: Some(format!("{}", self.regex)),
            }
        })
    }
}
