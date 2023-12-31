use std::ops::Deref;
use std::time::Duration;

use serde::{de, Deserialize};

use self::parse::{DurationVisitor, PartVisitor};

#[derive(Deserialize, Debug)]
pub struct Server {
    pub update_interval: Option<ConfDuration>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VpnName {
    pub name: String,
    pub substring_matcher: String,
}

pub trait GetTemplate {
    fn template(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    Session(String),
    Vpn(String, Vec<VpnName>),
    ShellCommand(String, String, bool),
    Disk(String, String),
    Swap(String),
    Memory(String),
    CPU(String),
    LoadAverage(String),
}

impl GetTemplate for Part {
    fn template(&self) -> String {
        let tpl = match self {
            Part::CPU(tpl) => tpl,
            Part::Disk(tpl, _) => tpl,
            Part::LoadAverage(tpl) => tpl,
            Part::Memory(tpl) => tpl,
            Part::ShellCommand(tpl, _, _) => tpl,
            Part::Session(tpl) => tpl,
            Part::Vpn(tpl, _) => tpl,
            Part::Swap(tpl) => tpl,
        };

        tpl.clone()
    }
}

impl<'de> de::Deserialize<'de> for Part {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PartVisitor)
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Segment {
    pub name: String,
    #[serde(default = "String::new")]
    pub separator: String,
    #[serde(default = "Vec::new")]
    pub fg_palette: Vec<String>,
    #[serde(default = "Vec::new")]
    pub bg_palette: Vec<String>,
    #[serde(default = "default_direction")]
    pub direction: String,
    pub parts: Vec<Part>,
}

fn default_direction() -> String {
    "ltr".to_string()
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: Server,
    pub segments: Vec<Segment>,
}

#[derive(Debug)]
pub struct ConfDuration(Duration);

impl Deref for ConfDuration {
    type Target = Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> de::Deserialize<'de> for ConfDuration {
    fn deserialize<D>(deserializer: D) -> Result<ConfDuration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(DurationVisitor)
    }
}

pub mod parse {
    use serde::de::{self, Error, Visitor};
    use serde_yaml::Value;
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::path::Path;
    use std::str::FromStr;
    use std::time::Duration;

    use crate::config::VpnName;

    use super::{AppConfig, ConfDuration, Part};

    pub fn parse_config(path: &Path) -> AppConfig {
        let f = std::fs::File::open(path).expect("Could not open file.");
        let scrape_config: AppConfig = serde_yaml::from_reader(f).expect("Could not read values.");
        scrape_config
    }

    #[derive(Debug)]
    pub enum ConfigParsingError {
        UnknownPartType,
    }

    impl Display for ConfigParsingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                ConfigParsingError::UnknownPartType => {
                    write!(f, "part.type field contains unkownd value")
                }
            }
        }
    }

    enum PartType {
        Session,
        Shell,
        VPN,
        Disk,
        Swap,
        Memory,
        CPU,
        LoadAverage,
    }

    impl Display for PartType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                PartType::Session => write!(f, "session_name"),
                PartType::Shell => write!(f, "shell"),
                PartType::VPN => write!(f, "vpn"),
                PartType::Disk => write!(f, "disk"),
                PartType::CPU => write!(f, "cpu"),
                PartType::LoadAverage => write!(f, "load_averag"),
                PartType::Memory => write!(f, "mem"),
                PartType::Swap => write!(f, "swap"),
            }
        }
    }

    impl FromStr for PartType {
        type Err = ConfigParsingError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "session_name" => Ok(PartType::Session),
                "shell" => Ok(PartType::Shell),
                "vpn" => Ok(PartType::VPN),
                "disk" => Ok(PartType::Disk),
                "mem" => Ok(PartType::Memory),
                "swap" => Ok(PartType::Swap),
                "cpu" => Ok(PartType::CPU),
                "load_average" => Ok(PartType::LoadAverage),

                _ => Err(ConfigParsingError::UnknownPartType),
            }
        }
    }

    pub struct DurationVisitor;

    impl<'de> Visitor<'de> for DurationVisitor {
        type Value = ConfDuration;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an string consisting of some number and unit")
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConfDuration(Duration::new(v, 0)))
        }
    }

    fn construct_vpn(m: HashMap<String, Value>, tpl: String) -> Result<Part, serde_yaml::Error> {
        if let Some(Value::Sequence(s)) = m.get("names") {
            let vpn_names: Vec<VpnName> = s
                .chunks(2)
                .filter(|c| c.len() == 2)
                .flat_map(|chunk| {
                    chunk.into_iter().filter_map(|p| match p {
                        Value::Mapping(m) => Some(m),
                        _ => None,
                    })
                })
                .filter_map(|m| {
                    if let Some(Value::String(name)) = m.get("name") {
                        if let Some(Value::String(substring_matcher)) = m.get("substring_matcher") {
                            Some(VpnName {
                                name: name.to_string(),
                                substring_matcher: substring_matcher.to_string(),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            Ok(Part::Vpn(tpl, vpn_names))
        } else {
            Err(Error::missing_field("list"))
        }
    }

    fn construct_shell(m: HashMap<String, Value>, tpl: String) -> Result<Part, serde_yaml::Error> {
        if let Some(Value::String(str)) = m.get("cmd") {
            let use_pwd = m
                .get("use_pwd")
                .map(|v| match v {
                    Value::Bool(b) => b,
                    _ => &false,
                })
                .unwrap_or(&false);
            Ok(Part::ShellCommand(tpl, str.to_string(), *use_pwd))
        } else {
            Err(Error::missing_field("cmd"))
        }
    }

    fn construct_disk(m: HashMap<String, Value>, tpl: String) -> Result<Part, serde_yaml::Error> {
        if let Some(Value::String(str)) = m.get("dev") {
            Ok(Part::Disk(tpl, str.to_string()))
        } else {
            Err(Error::missing_field("dev"))
        }
    }

    pub struct PartVisitor;

    impl<'de> Visitor<'de> for PartVisitor {
        type Value = Part;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Meaningfull error =(")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut hash_map: HashMap<String, Value> =
                HashMap::with_capacity(map.size_hint().unwrap_or(0));

            while let Some((key, value)) = map.next_entry()? {
                hash_map.insert(key, value);
            }

            if let Some(Value::String(part_type_str)) = hash_map.get("type") {
                if let Some(Value::String(template)) = hash_map.get("template") {
                    if let Ok(part_type) =
                        PartType::from_str(part_type_str).map_err(serde_yaml::Error::custom)
                    {
                        let tpl = template.clone();
                        match part_type {
                            PartType::Session => Ok(Part::Session(tpl)),
                            PartType::Shell => {
                                construct_shell(hash_map, tpl).map_err(Error::custom)
                            }
                            PartType::Disk => construct_disk(hash_map, tpl).map_err(Error::custom),
                            PartType::VPN => construct_vpn(hash_map, tpl).map_err(Error::custom),
                            PartType::CPU => Ok(Part::CPU(tpl)),
                            PartType::LoadAverage => Ok(Part::LoadAverage(tpl)),
                            PartType::Memory => Ok(Part::Memory(tpl)),
                            PartType::Swap => Ok(Part::Swap(tpl)),
                        }
                    } else {
                        Err(Error::custom("Cannot decode type field"))
                    }
                } else {
                    Err(Error::missing_field("template"))
                }
            } else {
                Err(Error::missing_field("type"))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::config::AppConfig;

        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;
        use std::path::PathBuf;

        #[test]
        fn parse_example_config() {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("examples/config.yaml");
            let config: AppConfig = parse_config(&d);

            // Assuming there are two segments, and the left segment is at index 0
            let left_segment = &config.segments[0];

            // Assert parts of the left segment
            assert_eq!(left_segment.parts.len(), 4); // Assuming there are three parts

            // Assert Session part
            let session_part = &left_segment.parts[0];
            assert!(matches!(session_part, Part::Session(_)));

            // Assert Memory part
            let memory_part = &left_segment.parts[1];
            assert!(matches!(memory_part, Part::Memory(_)));

            // Assert CPU part
            let cpu_part = &left_segment.parts[2];
            assert!(matches!(cpu_part, Part::CPU(_)));

            // Assuming there are two segments, and the right segment is at index 1
            let right_segment = &config.segments[1];

            // Assert parts of the right segment
            assert_eq!(right_segment.parts.len(), 1); // Assuming there is one part

            // Assert LoadAverage part
            let load_average_part = &right_segment.parts[0];
            assert!(matches!(load_average_part, Part::LoadAverage(_)));
        }
    }
}
