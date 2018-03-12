use yaml_rust::YamlLoader;

use client::Client;
use yaml;

mod browser;
mod os;
mod device;

use parser::browser::BrowserParser;
use parser::device::DeviceParser;
use parser::os::OSParser;

pub use parser::os::OS;
pub use parser::device::Device;
pub use parser::browser::Browser;

lazy_static! {
    static ref UAP: Vec<BrowserParser> =  {
        let s = include_str!("../../uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        yaml::from_map(&docs[0],"user_agent_parsers")
            .map(|y| yaml::filter_map_over_arr(y, BrowserParser::from_yaml)).unwrap()
    };
    static ref DP: Vec<DeviceParser> =  {
        let s = include_str!("../../uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        yaml::from_map(&docs[0],"device_parsers")
            .map(|y| yaml::filter_map_over_arr(y, DeviceParser::from_yaml)).unwrap()
    };
    static ref OSP: Vec<OSParser> =  {
        let s = include_str!("../../uap-core/regexes.yaml");
        let docs = YamlLoader::load_from_str(&s).unwrap();
        yaml::from_map(&docs[0],"os_parsers")
            .map(|y| yaml::filter_map_over_arr(y, OSParser::from_yaml)).unwrap()
    };
}

pub fn parse_browser(agent: &str) -> Browser {
    UAP.iter()
        .filter_map(|u| u.parse(agent))
        .next()
        .unwrap_or_else(|| Browser::new())
}

pub fn parse_os(agent: &str) -> OS {
    OSP.iter()
        .filter_map(|o| o.parse(agent))
        .next()
        .unwrap_or_else(|| OS::new())
}

pub fn parse_device(agent: &str) -> Device {
    DP.iter()
        .filter_map(|d| d.parse(agent))
        .next()
        .unwrap_or_else(|| Device::new())
}

///The `Parser` type, used for parsing user agent strings into `Client` structs.
pub struct Parser {}

impl Parser {
    ///Parses a user agent string into a `Client` struct.
    pub fn parse(agent: &str) -> Client {
        //For each of the attributes, we find the first regex that matches and use that. Otherwise
        //we use a default value.
        Client {
            user_agent: agent.to_string(),
            browser: parse_browser(agent),
            os: parse_os(agent),
            device: parse_device(agent),
        }
    }
}
