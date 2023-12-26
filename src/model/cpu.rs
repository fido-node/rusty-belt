use std::collections::VecDeque;

use crate::{
    protocol::rusty::belt::{segment_value, Cpu},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq)]
pub struct CPU {
    consumption_series: VecDeque<f64>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            consumption_series: VecDeque::with_capacity(9),
        }
    }
}

impl Model for CPU {
    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::Resources)
    }

    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let mut cpu = Cpu::default();
        cpu.consumption_series = self.consumption_series.iter().map(|v| *v).collect();
        segment_value::Segment::Cpu(cpu)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        if let Some(resource_stats) = cache_snapshot.get(&CacheKey::Resources) {
            match resource_stats {
                CacheValue::Resources(stats) => {
                    for stat in stats.into_iter() {
                        self.consumption_series.push_back(stat.cpu.usage as f64);
                        if self.consumption_series.len() > 8 {
                            self.consumption_series.pop_front();
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
mod tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_cpu_get_cache_key() {
        let cpu_model = CPU::new();
        assert_eq!(cpu_model.get_cache_key(), Some(CacheKey::Resources));
    }

    #[test]
    fn test_cpu_get_state() {
        let cpu_model = CPU::new();
        let context = Context::default(); // You may need to provide a suitable context
        let expected_segment = segment_value::Segment::Cpu(Cpu {
            consumption_series: vec![],
        });
        assert_eq!(cpu_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_cpu_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a CPU model
        let mut cpu_model = CPU::new();

        // Test rehydrate method
        let result = cpu_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the consumption value has been updated after rehydration
        assert_eq!(cpu_model.consumption_series, vec![0.75]); // Assuming that the sample value in create_cache_snapshot_with_resources is 0.75
    }
}
