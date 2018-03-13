use std::str::FromStr;
use std::string::ParseError;
use std::borrow::Cow;
use regex::{Captures, Regex, RegexBuilder};
use regex;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use serde_json;

use {Browser, Device, OS};

static UA_PARSER_REGEX_YAML: &'static str = include_str!("../resources/regexes.json");

lazy_static! {
    pub(super) static ref UA_PARSER_REGEXES: UARegexes = {
        serde_json::from_str(UA_PARSER_REGEX_YAML).unwrap()
    };
}

#[derive(Debug, Deserialize)]
pub(super) struct UARegexes {
    #[serde(rename = "user_agent_parsers")]
    browser_parsers: Vec<UABrowserRegex>,
    device_parsers: Vec<UADeviceRegex>,
    os_parsers: Vec<UAOSRegex>,
}

macro_rules! derive_with_regex_field {
    (
    	$(#[ $meta:meta ])*
    	struct $name:ident $name_raw:ident {
    		$(
    			$(#[$field_meta:meta])*
    			$field:ident : $field_type:ty
    		),*
    	}
    ) => {
    	$(#[$meta])*
    	struct $name {
    		regex: Regex,
    		$($field: $field_type),*
    	}


    	impl PartialEq for $name {
    		fn eq(&self, other: &$name) -> bool {
    			$(self.$field == other.$field && )* 
    			self.regex.as_str() == other.regex.as_str()
    		}
    	}

    	impl Eq for $name { }

    	$(#[$meta])*
    	#[derive(Deserialize)]
    	// Why not call this Raw and use macro hygene?
	    struct $name_raw {
	    	regex: String,
	    	#[serde(default)]
	    	regex_flag: Option<String>,
	    	$(
	    		$(#[$field_meta])*
	    		$field: $field_type
	    	),*
	    }

    	impl<'de: 'a, 'a> Deserialize<'de> for $name {
    	    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    	        where D: Deserializer<'de>
    	    {
    	        let raw = $name_raw::deserialize(deserializer)?;
    	        let flags = raw.regex_flag.as_ref().map(String::as_str);
    	        let regex = match compile_regex(&raw.regex, flags) {
    	        	Ok(regex) => regex,
    	        	Err(err) => {
    	        		let err = D::Error::custom(
    	        			format!("Error compiling regex pattern.\npattern: {}\nerror: {}",
    	        				    raw.regex, err));
    	        		return Err(err);
    	        	}
    	        };
    	        Ok($name {
    	        	regex: regex,
    	        	$($field: raw.$field),*
    	        })
    	    }
    	}
    }
}

fn compile_regex(pattern: &str, flags: Option<&str>) -> Result<Regex, regex::Error> {
    let rust_pattern = {
        if let Some(flags) = flags {
            Cow::Owned(format!("(?{}){}", flags, pattern))
        } else {
            Cow::Borrowed(pattern)
        }
    };
    let mut builder = RegexBuilder::new(&rust_pattern);
    // We need to increase this limit for the bot
    // patterns used by uap-core.
    // Fixed by https://github.com/ua-parser/uap-core/pull/62.
    builder.nest_limit(100);
    builder.build()
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UABrowserRegex UABrowserRegexRaw {
        #[serde(default)]
        family_replacement: Option<String>,
        #[serde(default)]
        v1_replacement: Option<String>,
        #[serde(default)]
        v2_replacement: Option<String>,
        #[serde(default)]
        v3_replacement: Option<String>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UAOSRegex UAOSRegexRaw {
        #[serde(default)]
        os_replacement: Option<String>,
        #[serde(default)]
        os_v1_replacement: Option<String>,
        #[serde(default)]
        os_v2_replacement: Option<String>,
        #[serde(default)]
        os_v3_replacement: Option<String>,
        #[serde(default)]
        os_v4_replacement: Option<String>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UADeviceRegex UADeviceRegexRaw {
        #[serde(default)]
        device_replacement: Option<String>,
        #[serde(default)]
        brand_replacement: Option<String>,
        #[serde(default)]
        model_replacement: Option<String>
    }
}

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

impl UABrowserRegex {
    fn parse(&self, agent: &str) -> Option<Browser> {
        self.regex.captures(agent).map(|c| {
            let family = self.family_replacement
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

            let major = self.v1_replacement
            	.clone()
                .or_else(|| get_or_none(&c, 2));
            let minor = self.v2_replacement
                .clone()
                .or_else(|| get_or_none(&c, 3));
            let patch = self.v3_replacement
                .clone()
                .or_else(|| get_or_none(&c, 4));

            Browser {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
            }
        })
    }
}

impl UAOSRegex {
    fn parse(&self, agent: &str) -> Option<OS> {
        self.regex.captures(agent).map(|c| {
            let family = self.os_replacement
                .clone()
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(&f, &c))
                .unwrap_or_else(|| "Other".to_string());
            let major = self.os_v1_replacement
                .clone()
                .map_or_else(|| get_or_none(&c, 2), |m| replace_matches(&m, &c));
            let minor = self.os_v2_replacement
                .clone()
                .map_or_else(|| get_or_none(&c, 3), |m| replace_matches(&m, &c));
            let patch = self.os_v3_replacement
                .clone()
                .map_or_else(|| get_or_none(&c, 4), |p| replace_matches(&p, &c));
            let patch_minor = self.os_v4_replacement
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

impl UADeviceRegex {
    fn parse(&self, agent: &str) -> Option<Device> {
        self.regex.captures(agent).map(|c| {
            let family = self.device_replacement
                .clone()
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(&f, &c))
                .unwrap_or_else(|| "Other".to_string());
            let brand = self.brand_replacement
                .clone()
            	.and_then(|m| replace_matches(&m, &c));
            let model = self.model_replacement
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

impl FromStr for Browser {
    type Err = ParseError;
    fn from_str(agent: &str) -> Result<Self, Self::Err> {
        Ok(UA_PARSER_REGEXES
            .browser_parsers
            .iter()
            .filter_map(|b| b.parse(agent))
            .next()
            .unwrap_or_else(|| Browser::default()))
    }
}

impl FromStr for OS {
    type Err = ParseError;
    fn from_str(agent: &str) -> Result<Self, Self::Err> {
        Ok(UA_PARSER_REGEXES
            .os_parsers
            .iter()
            .filter_map(|o| o.parse(agent))
            .next()
            .unwrap_or_else(|| Self::default()))
    }
}

impl FromStr for Device {
    type Err = ParseError;
    fn from_str(agent: &str) -> Result<Self, Self::Err> {
        Ok(UA_PARSER_REGEXES
            .device_parsers
            .iter()
            .filter_map(|d| d.parse(agent))
            .next()
            .unwrap_or_else(|| Device::default()))
    }
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
fn test_deserialize() {
	assert_eq!(UA_PARSER_REGEXES.browser_parsers[0], UABrowserRegex {
		regex: Regex::new(r"(ESPN)[%20| ]+Radio/(\d+)\.(\d+)\.(\d+) CFNetwork").unwrap(),
		family_replacement: None,
		v1_replacement: None,
		v2_replacement: None,
		v3_replacement: None
	});
}
