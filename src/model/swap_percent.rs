use std::ops::Div;

use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq)]
pub struct SwapPercent {
    used: u64,
}

impl SwapPercent {
    pub fn new() -> Self {
        SwapPercent { used: 0 }
    }
}

impl Model for SwapPercent {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut result = belt::SwapPercent::default();
        result.used = self.used;
        segment_value::Segment::SwapPercent(result)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        let s = &stat.swap;
                        let divided = (s.used as f64).div(s.total as f64);
                        self.used = (divided * 100.0) as u64;
                    }
                }
                _ => (),
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;
    

    #[test]
    fn test_new_swap_percent() {
        let swap_percent = SwapPercent::new();
        assert_eq!(swap_percent.used, 0);
    }

    #[test]
    fn test_get_cache_key() {
        let swap_percent = SwapPercent::new();
        assert_eq!(swap_percent.get_cache_key(), Some(CacheKey::Resources));
    }

    #[test]
    fn test_get_state() {
        let swap_percent = SwapPercent { used: 42 };
        let context = Context::default();
        let segment = swap_percent.get_state(&context);
        if let segment_value::Segment::SwapPercent(result) = segment {
            assert_eq!(result.used, 42);
        } else {
            panic!("Expected SwapPercent segment");
        }
    }

    #[test]
    fn test_rehydrate() {
        let mut swap_percent = SwapPercent::new();
        let cache_snapshot = create_cache_snapshot_with_resources();
        assert_eq!(swap_percent.used, 0);

        swap_percent.rehydrate(&cache_snapshot).unwrap();

        assert_eq!(swap_percent.used, 25); // Assuming your cache_snapshot has swap.used = 50 and swap.total = 100
    }
}
