use systemstat::{Platform, System};

use crate::{
    fetch::DiskInfo,
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::{CacheKey, CacheValue},
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq)]
pub struct Disk {
    dev: String,
    disk_info: DiskInfo,
}

impl Disk {
    pub fn new(dev: String) -> Self {
        Disk {
            dev,
            disk_info: DiskInfo::default(),
        }
    }
}

impl Model for Disk {
    fn get_state(&self, _context: &Context) -> segment_value::Segment {
        let di = &self.disk_info;
        let mut result = belt::Disk::default();
        result.mount_point = di.mount_point.clone();
        result.device_path = di.dev.clone();
        result.available_space_b = di.free_space;
        result.total_space_b = di.total_space;
        segment_value::Segment::Disk(result)
    }

    fn get_cache_key(&self) -> Option<CacheKey> {
        Some(CacheKey::DiskStats)
    }

    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        let _sys = System::new();

        if let Some(disk_stats) = cache_snapshot.get(&CacheKey::DiskStats) {
            match disk_stats {
                CacheValue::DiskStats(stats) => {
                    for stat in stats.into_iter() {
                        if stat.dev == self.dev {
                            self.disk_info = stat.clone()
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
mod disk_tests {
    use crate::model::tests::create_cache_snapshot_with_resources;

    use super::*;

    #[test]
    fn test_disk_get_state() {
        // Create a Disk model
        let disk_model = Disk::new(String::from("/dev/sda"));

        // Create a context (You may need to provide a suitable context)
        let context = Context::default();

        // Test get_state method
        let expected_segment = segment_value::Segment::Disk(belt::Disk {
            device_path: String::from(""),
            mount_point: String::from(""),
            available_space_b: 0,
            total_space_b: 0,
            ..Default::default()
        });
        assert_eq!(disk_model.get_state(&context), expected_segment);
    }

    #[test]
    fn test_disk_get_cache_key() {
        // Create a Disk model
        let disk_model = Disk::new(String::from("/dev/sda"));

        // Test get_cache_key method
        assert_eq!(disk_model.get_cache_key(), Some(CacheKey::DiskStats));
    }

    #[test]
    fn test_disk_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a Disk model
        let mut disk_model = Disk::new(String::from("/dev/sda"));

        // Test rehydrate method
        let result = disk_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Verify that the disk_info has been updated after rehydration
        let expected_disk_info = DiskInfo {
            dev: String::from("/dev/sda"),
            mount_point: String::from("/"),
            total_space: 102400,
            free_space: 51200,
        };
        assert_eq!(disk_model.disk_info, expected_disk_info);
    }
}
