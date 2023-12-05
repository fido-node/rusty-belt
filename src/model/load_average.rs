use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

// 1m,5m,15m
// LA(f64, f64, f64),

#[derive(Debug, PartialEq)]
pub struct LoadAverage {
    one: f64,
    five: f64,
    fifteen: f64,
}

impl LoadAverage {
    pub fn new() -> Self {
        LoadAverage {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
        }
    }
}

impl Model for LoadAverage {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(self: &Self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::LoadAverage::default();
        result.one = self.one;
        result.five = self.five;
        result.fifteen = self.fifteen;
        segment_value::Segment::LoadAverage(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        let la = &stat.la;
                        self.one = la.one;
                        self.five = la.five;
                        self.fifteen = la.fifteen;
                    }
                }
                _ => (),
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod load_average_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_load_average_get_cache_key() {
        let load_average_model = LoadAverage::new();
        assert_eq!(
            load_average_model.get_cache_key(),
            Some(CacheKey::Resources)
        );
    }

    #[test]
    fn test_load_average_get_state() {
        let load_average_model = LoadAverage::new();
        let context = Context::default(); // You may need to provide a suitable context

        let expected_segment = segment_value::Segment::LoadAverage(belt::LoadAverage {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
            ..Default::default()
        });
        assert_eq!(load_average_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_load_average_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a LoadAverage model
        let mut load_average_model = LoadAverage::new();

        // Test rehydrate method
        let result = load_average_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the LoadAverage values have been updated after rehydration
        assert_eq!(load_average_model.one, 1.5); // Assuming that the sample value in create_cache_snapshot_with_resources is 1.5
        assert_eq!(load_average_model.five, 2.0); // Assuming that the sample value in create_cache_snapshot_with_resources is 2.0
        assert_eq!(load_average_model.fifteen, 1.8); // Assuming that the sample value in create_cache_snapshot_with_resources is 1.8
    }
}
