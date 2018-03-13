use yaml_rust::Yaml;
use yaml;
use regex::Regex;

use {build_uap_regexp, get_or_none, replace_matches};




#[derive(Debug)]
pub(super) struct OSParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl OSParser {
    pub fn from_yaml(y: &Yaml) -> Option<OSParser> {
        let regex_flag = yaml::string_from_map(y, "regex_flag");
        yaml::string_from_map(y, "regex")
            .and_then(|r| build_uap_regexp(&r, regex_flag.as_ref()).ok())
            .map(|r| OSParser {
                regex: r,
                family: yaml::string_from_map(y, "os_replacement"),
                major: yaml::string_from_map(y, "os_v1_replacement"),
                minor: yaml::string_from_map(y, "os_v2_replacement"),
                patch: yaml::string_from_map(y, "os_v3_replacement"),
                patch_minor: yaml::string_from_map(y, "os_v4_replacement"),
            })
    }

    pub fn parse(&self, agent: &str) -> Option<OS> {
        
    }
}
