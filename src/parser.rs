use std::str::FromStr;
use std::string::ParseError;
use regex::{Captures, Regex, RegexBuilder};
use regex;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use rmps;

use {Browser, Device, OS};

static UA_PARSER_REGEX_DATA: &'static [u8] = include_bytes!("../resources/regexes.msgpack");

lazy_static! {
    pub(super) static ref UA_PARSER_REGEXES: UARegexes = {
        rmps::from_slice(UA_PARSER_REGEX_DATA).unwrap()
    };
}

#[derive(Debug, Deserialize)]
pub(super) struct UARegexes {
    #[serde(rename = "b")]
    browser_parsers: Vec<UABrowserRegex>,
    #[serde(rename = "d")]
    device_parsers: Vec<UADeviceRegex>,
    #[serde(rename = "o")]
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
	    	#[serde(rename="r")]
	    	regex: String,
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
    	        let regex = match compile_regex(&raw.regex) {
    	        	Ok(regex) => regex,
    	        	Err(err) => {
    	        		let err = D::Error::custom(
    	        			format!("Error compiling regex pattern.\n  pattern: {}\n  error: {}",
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

fn compile_regex(pattern: &str) -> Result<Regex, regex::Error> {
    let mut builder = RegexBuilder::new(&pattern);
    // We need to increase this limit for the bot
    // patterns used by uap-core.
    // Fixed by https://github.com/ua-parser/uap-core/pull/62.
    builder.nest_limit(100);
    builder.build()
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UABrowserRegex UABrowserRegexRaw {
        #[serde(default, rename="f")]
        family_replacement: Option<String>,
        #[serde(default, rename="1")]
        v1_replacement: Option<String>,
        #[serde(default, rename="2")]
        v2_replacement: Option<String>,
        #[serde(default, rename="3")]
        v3_replacement: Option<String>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UAOSRegex UAOSRegexRaw {
        #[serde(default, rename="o")]
        os_replacement: Option<String>,
        #[serde(default, rename="1")]
        os_v1_replacement: Option<String>,
        #[serde(default, rename="2")]
        os_v2_replacement: Option<String>,
        #[serde(default, rename="3")]
        os_v3_replacement: Option<String>,
        #[serde(default, rename="4")]
        os_v4_replacement: Option<String>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UADeviceRegex UADeviceRegexRaw {
        #[serde(default, rename="d")]
        device_replacement: Option<String>,
        #[serde(default, rename="b")]
        brand_replacement: Option<String>,
        #[serde(default, rename="m")]
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
