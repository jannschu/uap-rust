use yaml_rust::Yaml;

pub fn from_map<'a>(map: &'a Yaml, key: &str) -> Option<&'a Yaml> {
    map.as_hash()
        .and_then(|h| h.get(&Yaml::String(key.to_string())))
}
