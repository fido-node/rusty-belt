use std::collections::HashMap;

use crate::{
    config::VpnName,
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq, Eq)]
pub struct Vpn {
    pub search_map: HashMap<String, String>,
    pub connected_networks: Vec<String>,
}

impl Vpn {
    pub fn new(names: &Vec<VpnName>) -> Self {
        let search_map: HashMap<String, String> = names
            .into_iter()
            .map(|name| (name.name.to_string(), name.substring_matcher.to_string()))
            .collect();
        Vpn {
            search_map,
            connected_networks: Vec::new(),
        }
    }
}

impl Model for Vpn {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Networks)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::ConnectedVpNs::default();
        result.aliases = self.connected_networks.iter().map(|s| s.clone()).collect();
        segment_value::Segment::ConnectedVpns(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        let mut connected_networks: Vec<String> = Vec::new();
        if let Some(network_snapshot) = cache_snapshot.get(&CacheKey::Networks) {
            match network_snapshot {
                CacheValue::Networks(stats) => {
                    for stat in stats.into_iter() {
                        for (name, search_str) in self.search_map.iter() {
                            if let Some(ip_addr) = &stat.ip_addr {
                                if ip_addr.contains(search_str) {
                                    connected_networks.push(name.clone())
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        };
        self.connected_networks = connected_networks;

        Ok(())
    }
}

#[cfg(test)]
mod vpn_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_vpn_get_cache_key() {
        let vpn_model = Vpn::new(&vec![VpnName {
            name: "VPN1".to_string(),
            substring_matcher: "192.168.1.".to_string(),
        }]);
        assert_eq!(vpn_model.get_cache_key(), Some(CacheKey::Networks));
    }

    #[test]
    fn test_vpn_get_state() {
        let vpn_model = Vpn::new(&vec![VpnName {
            name: "VPN1".to_string(),
            substring_matcher: "192.168.1.".to_string(),
        }]);
        let context = Context::default(); // You may need to provide a suitable context

        let expected_segment = segment_value::Segment::ConnectedVpns(belt::ConnectedVpNs {
            aliases: Vec::new(),
            ..Default::default()
        });
        assert_eq!(vpn_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_vpn_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a Vpn model
        let mut vpn_model = Vpn::new(&vec![VpnName {
            name: "VPN1".to_string(),
            substring_matcher: "192.168.1.".to_string(),
        }]);

        // Test rehydrate method
        let result = vpn_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the ConnectedVpNs values have been updated after rehydration
        assert_eq!(vpn_model.connected_networks, vec!["VPN1"]);
    }
}
