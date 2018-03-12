use yaml_rust::Yaml;
use yaml;
use regex::Regex;

use {build_uap_regexp, get_or_none};

///`UserAgent` contains the user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Debug)]
pub struct UserAgentParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl UserAgentParser {
    pub fn from_yaml(y: &Yaml) -> Option<UserAgentParser> {
        let regex_flag = yaml::string_from_map(y, "regex_flag");
        yaml::string_from_map(y, "regex")
            .and_then(|r| build_uap_regexp(&r, regex_flag.as_ref()).ok())
            .map(|r| UserAgentParser {
                regex: r,
                family: yaml::string_from_map(y, "family_replacement"),
                major: yaml::string_from_map(y, "v1_replacement"),
                minor: yaml::string_from_map(y, "v2_replacement"),
                patch: yaml::string_from_map(y, "v3_replacement"),
            })
    }

    pub fn parse(&self, agent: String) -> Option<UserAgent> {
        self.regex.captures(&agent).map(|c| {
            let family = self.family
                .clone()
                .and_then(|f| {
                    if let Some(group1) = c.get(1) {
                        Some(f.replace("$1", group1.as_str()))
                    } else {
                        Some(f)
                    }
                })
                .or_else(|| c.get(1).map(|c| c.as_str().to_string()))
                .unwrap_or_else(|| "Other".to_string());

            let major = self.major.clone().or_else(|| get_or_none(&c, 2));
            let minor = self.minor.clone().or_else(|| get_or_none(&c, 3));
            let patch = self.patch.clone().or_else(|| get_or_none(&c, 4));

            UserAgent {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
            }
        })
    }
}
