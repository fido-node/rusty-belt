use std::collections::VecDeque;

use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq)]
pub struct Swap {
    used: u64,
    total: u64,
    used_percents_series: VecDeque<f64>,
}

impl Swap {
    pub fn new() -> Self {
        Self {
            used: 0,
            total: 0,
            used_percents_series: VecDeque::with_capacity(9),
        }
    }
}

impl Model for Swap {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::Swap::default();
        result.used = self.used;
        result.total = self.total;
        result.used_percents_series = self.used_percents_series.iter().map(|v| *v).collect();

        segment_value::Segment::Swap(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        let s = &stat.swap;
                        self.used = s.used;
                        self.total = s.total;
                        let used_percents = (s.used as f64 / s.total as f64) * 100.0;
                        self.used_percents_series.push_back(used_percents);
                        if self.used_percents_series.len() >= 8 {
                            self.used_percents_series.pop_front();
                        }
                    }
                }
                _ => (),
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod swap_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_swap_get_cache_key() {
        let swap_model = Swap::new();
        assert_eq!(swap_model.get_cache_key(), Some(CacheKey::Resources));
    }

    #[test]
    fn test_swap_get_state() {
        let swap_model = Swap::new();
        let context = Context::default(); // You may need to provide a suitable context

        let expected_segment = segment_value::Segment::Swap(belt::Swap {
            used: 0,
            total: 0,
            ..Default::default()
        });
        assert_eq!(swap_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_swap_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a Swap model
        let mut swap_model = Swap::new();

        // Test rehydrate method
        let result = swap_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the Swap values have been updated after rehydration
        assert_eq!(swap_model.used, 2048); // Assuming that the sample value in create_cache_snapshot_with_resources is 2048
        assert_eq!(swap_model.total, 8192); // Assuming that the sample value in create_cache_snapshot_with_resources is 8192
    }
}
