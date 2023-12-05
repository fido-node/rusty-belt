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

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    Session,
    Vpn(Vec<VpnName>),
    ShellCommand(String, bool),
    Disk(String),
    Swap,
    SwapPercents,
    Memory,
    MemoryPercents,
    CPU,
    LoadAverage,
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
    #[serde(default = "Vec::new")]
    pub fg_palet: Vec<String>,
    pub parts: Vec<Part>,
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
        UnknownResource,
    }

    impl Display for ConfigParsingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                ConfigParsingError::UnknownPartType => {
                    write!(f, "part.type field contains unkownd value")
                }
                ConfigParsingError::UnknownResource => {
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
        SwapPercents,
        Memory,
        MemoryPercents,
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
                PartType::MemoryPercents => write!(f, "mem_percent"),
                PartType::Swap => write!(f, "swap"),
                PartType::SwapPercents => write!(f, "swap_percent"),
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
                "mem_percent" => Ok(PartType::MemoryPercents),
                "swap" => Ok(PartType::Swap),
                "swap_percent" => Ok(PartType::SwapPercents),
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

    fn construct_vpn(m: HashMap<String, Value>) -> Result<Part, serde_yaml::Error> {
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

            Ok(Part::Vpn(vpn_names))
        } else {
            Err(Error::missing_field("list"))
        }
    }

    fn construct_shell(m: HashMap<String, Value>) -> Result<Part, serde_yaml::Error> {
        if let Some(Value::String(str)) = m.get("cmd") {
            let use_pwd = m
                .get("use_pwd")
                .map(|v| match v {
                    Value::Bool(b) => b,
                    _ => &false,
                })
                .unwrap_or(&false);
            Ok(Part::ShellCommand(str.to_string(), *use_pwd))
        } else {
            Err(Error::missing_field("cmd"))
        }
    }

    fn construct_disk(m: HashMap<String, Value>) -> Result<Part, serde_yaml::Error> {
        if let Some(Value::String(str)) = m.get("dev") {
            Ok(Part::Disk(str.to_string()))
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
                if let Ok(part_type) =
                    PartType::from_str(part_type_str).map_err(serde_yaml::Error::custom)
                {
                    match part_type {
                        PartType::Session => Ok(Part::Session),
                        PartType::Shell => construct_shell(hash_map).map_err(Error::custom),
                        PartType::Disk => construct_disk(hash_map).map_err(Error::custom),
                        PartType::VPN => construct_vpn(hash_map).map_err(Error::custom),
                        PartType::CPU => Ok(Part::CPU),
                        PartType::LoadAverage => Ok(Part::LoadAverage),
                        PartType::Memory => Ok(Part::Memory),
                        PartType::MemoryPercents => Ok(Part::MemoryPercents),
                        PartType::Swap => Ok(Part::Swap),
                        PartType::SwapPercents => Ok(Part::SwapPercents),
                    }
                } else {
                    Err(Error::missing_field("type"))
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
            let app_config: AppConfig = parse_config(&d);
            // Define the path to the example YAML configuration file

            // Parse the configuration file

            // Assertions for the first segment
            assert_eq!(app_config.segments.len(), 2);
            assert_eq!(app_config.segments[0].name, "left");

            // Assertions for parts in the first segment
            assert_eq!(app_config.segments[0].fg_palet.len(), 10);
            assert_eq!(app_config.segments[0].parts.len(), 10);
            assert_eq!(app_config.segments[0].parts[0], Part::Session);

            if let Part::Vpn(vpn_names) = &app_config.segments[0].parts[1] {
                // Assertions for VPN names
                assert_eq!(vpn_names.len(), 2);
                assert_eq!(
                    vpn_names[0],
                    VpnName {
                        name: "prod".to_string(),
                        substring_matcher: "10.154.1.".to_string()
                    }
                );
                assert_eq!(
                    vpn_names[1],
                    VpnName {
                        name: "dev".to_string(),
                        substring_matcher: "10.154.154.".to_string()
                    }
                );
            } else {
                panic!("Expected VPN part in the first segment");
            }

            // Assertions for remaining parts in the first segment
            assert_eq!(app_config.segments[0].parts[2], Part::Memory);
            assert_eq!(app_config.segments[0].parts[3], Part::MemoryPercents);
            assert_eq!(app_config.segments[0].parts[4], Part::CPU);
            assert_eq!(app_config.segments[0].parts[5], Part::LoadAverage);
            assert_eq!(app_config.segments[0].parts[6], Part::Swap);
            assert_eq!(app_config.segments[0].parts[7], Part::SwapPercents);

            // Assertions for the second segment
            assert_eq!(app_config.segments[1].name, "right");

            // Assertions for parts in the second segment
            assert_eq!(app_config.segments[1].parts.len(), 1);
            if let Part::ShellCommand(shell_cmd, use_pwd) = &app_config.segments[1].parts[0] {
                assert_eq!(shell_cmd, "gitmux -cfg ~/.config/tmux/gitmux.yaml");
                assert_eq!(*use_pwd, true);
            } else {
                panic!("Expected ShellCommand part in the second segment");
            }
        }
    }
}
