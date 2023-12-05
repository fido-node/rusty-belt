use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

// Total / Used / Avail
// MemP(u64),

#[derive(Debug, PartialEq)]
pub struct MemPercent {
    used: u64,
}

impl MemPercent {
    pub fn new() -> Self {
        MemPercent { used: 0 }
    }
}

impl Model for MemPercent {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::MemPercent::default();
        result.used = self.used;
        segment_value::Segment::MemoryPercent(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        let m = &stat.memory;

                        self.used = ((m.used as f64 / m.total as f64) * 100.0) as u64;
                    }
                }
                _ => (),
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod mem_percent_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_mem_percent_get_cache_key() {
        let mem_percent_model = MemPercent::new();
        assert_eq!(mem_percent_model.get_cache_key(), Some(CacheKey::Resources));
    }

    #[test]
    fn test_mem_percent_get_state() {
        let mem_percent_model = MemPercent::new();
        let context = Context::default(); // You may need to provide a suitable context

        let expected_segment = segment_value::Segment::MemoryPercent(belt::MemPercent {
            used: 0,
            ..Default::default()
        });
        assert_eq!(mem_percent_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_mem_percent_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a MemPercent model
        let mut mem_percent_model = MemPercent::new();

        // Test rehydrate method
        let result = mem_percent_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the MemPercent value has been updated after rehydration
        assert_eq!(mem_percent_model.used, 50); // Assuming that the sample values in create_cache_snapshot_with_resources result in 50% usage
    }
}
