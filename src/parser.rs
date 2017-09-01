use std::io::prelude::*;
use std::fs::{File};
use yaml_rust::{YamlLoader};

use client::{Client};
use ua::{UserAgent, UserAgentParser};
use device::{Device, DeviceParser};
use os::{OS, OSParser};
use result::*;
use yaml;

///The `Parser` type, used for parsing user agent strings into `Client` structs.
pub struct Parser {
}

lazy_static! {
    static ref UAP: Vec<UserAgentParser> =  {
        let s = include_str!("uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        yaml::from_map(&docs[0],"user_agent_parsers")
            .map(|y| yaml::filter_map_over_arr(y, UserAgentParser::from_yaml)).unwrap()
    };
    static ref DP: Vec<DeviceParser> =  {
        let s = include_str!("uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let r: Vec<DeviceParser> = yaml::from_map(&docs[0],"device_parsers")
            .map(|y| yaml::filter_map_over_arr(y, DeviceParser::from_yaml)).unwrap();
        r
    };
    static ref OSP: Vec<OSParser> =  {
        let s = include_str!("uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        yaml::from_map(&docs[0],"os_parsers")
            .map(|y| yaml::filter_map_over_arr(y, OSParser::from_yaml)).unwrap()
    };
}

impl Parser {

    ///Constructs a `Parser` from a file path to a regexes file.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///file format.
    pub fn from_file(regexes_file: &str) -> Result<Parser> {
        let mut file = try!(File::open(regexes_file));
        let mut yaml = String::new();
        let _ = file.read_to_string(&mut yaml);
        Parser::from_str(&yaml)
    }

    ///Constructs a `Parser` from an str containing regexes.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///format.
    pub fn from_str(_: &str) -> Result<Parser> {
        //Parse the yaml.
        let p = Parser {
        };
        Ok(p)
    }

    ///Constructs a `Parser` from the staticly complied regexes file data.
    ///
    ///See [uap-core](https://github.com/ua-parser/uap-core/) documentation for information on the
    ///format.
    pub fn new() -> Result<Parser> {
         let s = include_str!("uap-core/regexes.yaml");
         Parser::from_str(&s)
    }

    ///Parses a user agent string into a `Client` struct.
    pub fn parse(&self, agent: String) -> Client {
        //For each of the attributes, we find the first regex that matches and use that. Otherwise
        //we use a default value.

        let ua = UAP.iter().filter_map(|u| u.parse(agent.clone())).next();
        let u = ua.unwrap_or(
            UserAgent {
                family: "Other".to_string(),
                major: None,
                minor: None,
                patch: None,
            });
        let dev = DP.iter().filter_map(|d| d.parse(agent.clone())).next();
        let d = dev.unwrap_or(Device {
            family: "Other".to_string(),
            model: None,
            brand: None,
            regex: None,
        });
        let oss = OSP.iter().filter_map(|d| d.parse(agent.clone())).next();
        let o = oss.unwrap_or(OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        });
        Client {
            user_agent: u,
            os: o,
            device: d,
        }
    }
}
