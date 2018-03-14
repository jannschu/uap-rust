use std::borrow::Cow;

use regex::{Captures, Regex, RegexBuilder};
use regex;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use rmps;

use {Browser, Device, DEFAULT_NAME, OS};

static UA_PARSER_REGEX_DATA: &'static [u8] = include_bytes!("../resources/regexes.msgpack");

lazy_static! {
    pub(super) static ref UA_PARSER_REGEXES: UARegexes<'static> = {
        rmps::from_slice(UA_PARSER_REGEX_DATA).unwrap()
    };
}

#[derive(Debug, Deserialize)]
pub(super) struct UARegexes<'a> {
    #[serde(borrow, rename = "b")]
    browser_parsers: Vec<UABrowserRegex<'a>>,
    #[serde(borrow, rename = "d")]
    device_parsers: Vec<UADeviceRegex<'a>>,
    #[serde(borrow, rename = "o")]
    os_parsers: Vec<UAOSRegex<'a>>,
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
    	struct $name<'a> {
    		regex: Regex,
    		$($field: $field_type),*
    	}


    	impl<'a> PartialEq for $name<'a> {
    		fn eq(&self, other: &$name) -> bool {
    			$(self.$field == other.$field && )*
    			self.regex.as_str() == other.regex.as_str()
    		}
    	}

    	impl<'a> Eq for $name<'a> { }

    	$(#[$meta])*
    	#[derive(Deserialize)]
    	// Why not call this Raw and use macro hygene?
	    struct $name_raw<'a> {
	    	#[serde(borrow, rename="r")]
	    	regex: &'a str,
	    	$(
	    		$(#[$field_meta])*
	    		$field: $field_type
	    	),*
	    }

    	impl<'de: 'a, 'a> Deserialize<'de> for $name<'a> {
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
        #[serde(borrow, default, rename="f")]
        family_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="1")]
        v1_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="2")]
        v2_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="3")]
        v3_replacement: Option<&'a str>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UAOSRegex UAOSRegexRaw {
        #[serde(borrow, default, rename="o")]
        os_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="1")]
        os_v1_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="2")]
        os_v2_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="3")]
        os_v3_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="4")]
        os_v4_replacement: Option<&'a str>
    }
}

derive_with_regex_field! {
    #[derive(Debug)]
    struct UADeviceRegex UADeviceRegexRaw {
        #[serde(borrow, default, rename="d")]
        device_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="b")]
        brand_replacement: Option<&'a str>,
        #[serde(borrow, default, rename="m")]
        model_replacement: Option<&'a str>
    }
}

fn replace_matches<'a>(s: &'a str, caps: &Captures<'a>) -> Option<Cow<'a, str>> {
    let s = match s.as_bytes().contains(&b'$') {
        true => {
            let mut dst = String::with_capacity(2 * s.len());
            caps.expand(s, &mut dst);
            Cow::Owned(dst)
        }
        false => Cow::Borrowed(s),
    };

    // FIXME: Can this be improved with non-lexical
    // lifetimes (and the trim method)?
    // cf. https://github.com/rust-lang/rust-roadmap/issues/16
    // Or `move into guards`?

    if s.is_empty() {
        return None;
    }

    let start = s.find(|c: char| c != ' ').unwrap_or(s.len());
    let end = s.rfind(|c: char| c != ' ').unwrap_or(s.len() - 1) + 1;

    if start == end {
        return None;
    }

    if start == 0 && end == s.len() {
        return Some(s);
    }

    match s {
        Cow::Borrowed(s) => Some(Cow::Borrowed(&s[start..end])),
        Cow::Owned(s) => Some(Cow::Owned(s[start..end].to_string())),
    }
}

fn get_or_none<'a>(c: &Captures<'a>, i: usize) -> Option<Cow<'a, str>> {
    if let Some(group) = c.get(i) {
        let s = Cow::Borrowed(group.as_str());
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    }
}

impl<'a: 'b, 'b> UABrowserRegex<'a> {
    fn parse(&self, agent: &'b str) -> Option<Browser<'b>> {
        self.regex.captures(agent).map(|c| {
            let family = self.family_replacement
                .and_then(|f| {
                    if let Some(group1) = c.get(1) {
                        Some(Cow::Owned(f.replace("$1", group1.as_str())))
                    } else {
                        Some(Cow::Borrowed(f))
                    }
                })
                .or_else(|| c.get(1).map(|c| Cow::Borrowed(c.as_str())))
                .unwrap_or_else(|| Cow::Borrowed(DEFAULT_NAME));

            let major = self.v1_replacement
                .map(|v1| Cow::Borrowed(v1))
                .or_else(|| get_or_none(&c, 2));
            let minor = self.v2_replacement
                .map(|v2| Cow::Borrowed(v2))
                .or_else(|| get_or_none(&c, 3));
            let patch = self.v3_replacement
                .map(|v2| Cow::Borrowed(v2))
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

impl<'a> UAOSRegex<'a> {
    fn parse(&self, agent: &'a str) -> Option<OS> {
        self.regex.captures(agent).map(|c| {
            let family: Cow<str> = self.os_replacement
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(f, &c))
                .unwrap_or_else(|| Cow::Borrowed(DEFAULT_NAME));
            let major = self.os_v1_replacement
                .map_or_else(|| get_or_none(&c, 2), |m| replace_matches(m, &c));
            let minor = self.os_v2_replacement
                .map_or_else(|| get_or_none(&c, 3), |m| replace_matches(m, &c));
            let patch = self.os_v3_replacement
                .map_or_else(|| get_or_none(&c, 4), |p| replace_matches(p, &c));
            let patch_minor = self.os_v4_replacement
                .map_or_else(|| get_or_none(&c, 5), |p| replace_matches(p, &c));

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

impl<'a> UADeviceRegex<'a> {
    fn parse(&self, agent: &'a str) -> Option<Device> {
        self.regex.captures(agent).map(|c| {
            let family = self.device_replacement
                .map_or_else(|| get_or_none(&c, 1), |f| replace_matches(f, &c))
                .unwrap_or_else(|| Cow::Borrowed(DEFAULT_NAME));
            let brand = self.brand_replacement.and_then(|m| replace_matches(m, &c));
            let model = self.model_replacement
                .map_or_else(|| get_or_none(&c, 1), |m| replace_matches(m, &c));
            Device {
                family: family,
                brand: brand,
                model: model,
            }
        })
    }
}

impl<'a> From<&'a str> for Browser<'a> {
    fn from(agent: &'a str) -> Self {
        UA_PARSER_REGEXES
            .browser_parsers
            .iter()
            .filter_map(|b| b.parse(agent))
            .next()
            .unwrap_or_else(|| Browser::default())
    }
}

impl<'a> From<&'a str> for OS<'a> {
    fn from(agent: &'a str) -> Self {
        UA_PARSER_REGEXES
            .os_parsers
            .iter()
            .filter_map(|o| o.parse(agent))
            .next()
            .unwrap_or_else(|| Self::default())
    }
}

impl<'a> From<&'a str> for Device<'a> {
    fn from(agent: &'a str) -> Self {
        UA_PARSER_REGEXES
            .device_parsers
            .iter()
            .filter_map(|d| d.parse(agent))
            .next()
            .unwrap_or_else(|| Device::default())
    }
}

#[test]
fn test_replace_matches() {
    let re = Regex::new(r"Ok (\d+) (\d+)").unwrap();
    let captures = re.captures("Ok 1 2").unwrap();
    assert_eq!(
        replace_matches("$2 $1 $2", &captures),
        Some(Cow::Borrowed("2 1 2"))
    );
}

#[test]
fn test_deserialize() {
    assert_eq!(
        UA_PARSER_REGEXES.browser_parsers[0],
        UABrowserRegex {
            regex: Regex::new(r"(ESPN)[%20| ]+Radio/(\d+)\.(\d+)\.(\d+) CFNetwork").unwrap(),
            family_replacement: None,
            v1_replacement: None,
            v2_replacement: None,
            v3_replacement: None,
        }
    );
}
