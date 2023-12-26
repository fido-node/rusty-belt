use std::collections::VecDeque;

use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

// Total / Used / Avail
// Mem(u64, u64, u64),

#[derive(Debug, PartialEq)]
pub struct Mem {
    total: u64,
    used: u64,
    available: u64,
    used_percents_series: VecDeque<f64>,
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            total: 1,
            used: 1,
            available: 1,
            used_percents_series: VecDeque::with_capacity(9),
        }
    }
}

impl Model for Mem {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::Mem::default();
        result.available = self.available;
        result.used = self.used;
        result.total = self.total;
        result.used_percents_series = self.used_percents_series.iter().map(|v| *v).collect();
        segment_value::Segment::Memory(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        let m = &stat.memory;

                        self.available = m.available;
                        self.total = m.total;
                        self.used = m.used;
                        let used_percents = (m.used as f64 / m.total as f64) * 100.0;
                        self.used_percents_series.push_back(used_percents);
                        if self.used_percents_series.len() > 8 {
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
mod mem_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_mem_get_cache_key() {
        let mem_model = Mem::new();
        assert_eq!(mem_model.get_cache_key(), Some(CacheKey::Resources));
    }

    #[test]
    fn test_mem_get_state() {
        let mem_model = Mem::new();
        let context = Context::default(); // You may need to provide a suitable context

        let expected_segment = segment_value::Segment::Memory(belt::Mem {
            available: 1,
            used: 1,
            total: 1,
            ..Default::default()
        });
        assert_eq!(mem_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_mem_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a Mem model
        let mut mem_model = Mem::new();

        // Test rehydrate method
        let result = mem_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the Mem values have been updated after rehydration
        assert_eq!(mem_model.available, 8192); // Assuming that the sample value in create_cache_snapshot_with_resources is 8192
        assert_eq!(mem_model.total, 16384); // Assuming that the sample value in create_cache_snapshot_with_resources is 16384
        assert_eq!(mem_model.used, 8192); // Assuming that the sample value in create_cache_snapshot_with_resources is 8192
    }
}
