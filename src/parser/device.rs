use yaml_rust::Yaml;
use yaml;
use regex::Regex;

use {build_uap_regexp, get_or_none, replace_matches};




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
    }
}
