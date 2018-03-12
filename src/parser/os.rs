use yaml_rust::Yaml;
use yaml;
use regex::Regex;

use {build_uap_regexp, get_or_none, replace_matches};

///`OS` contains the operating system information from the user agent.
#[derive(Debug, PartialEq, Eq)]
pub struct OS {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl OS {
    pub(crate) fn new() -> OS {
        OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

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
        self.regex.captures(agent).map(|c| {
            let family = self.family
                .clone()
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(&f, &c))
                .unwrap_or_else(|| "Other".to_string());
            let major = self.major
                .clone()
                .map_or_else(|| get_or_none(&c, 2), |m| replace_matches(&m, &c));
            let minor = self.minor
                .clone()
                .map_or_else(|| get_or_none(&c, 3), |m| replace_matches(&m, &c));
            let patch = self.patch
                .clone()
                .map_or_else(|| get_or_none(&c, 4), |p| replace_matches(&p, &c));
            let patch_minor = self.patch_minor
                .clone()
                .map_or_else(|| get_or_none(&c, 5), |p| replace_matches(&p, &c));

            OS {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
                patch_minor: patch_minor,
            }
        })
    }
}
