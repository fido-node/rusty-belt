mod cpu;
mod disk;
mod load_average;
mod mem;
mod session;
mod shell;
mod swap;
mod vpn;

use crate::{
    config::Part,
    protocol::rusty::belt::segment_value,
    state::rehydrator::{CacheKey, CacheSnapshot},
};
use std::fmt::Debug;

use self::{
    cpu::CPU,
    disk::Disk,
    load_average::LoadAverage,
    mem::Mem,
    session::Session,
    shell::{DefaultExecutor, Shell},
    swap::Swap,
    vpn::Vpn,
};

pub trait Model: Debug + Send + 'static {
    fn get_state(&self, context: &Context) -> segment_value::Segment;
    fn get_cache_key(&self) -> Option<CacheKey>;
    fn rehydrate(&mut self, cache_snapshot: &CacheSnapshot) -> Result<(), ()>;
}

pub struct ModelHelper {}

impl ModelHelper {
    fn new(part: &Part) -> Option<Box<dyn Model>> {
        match part {
            Part::Disk(_, dev) => Some(Box::new(Disk::new(dev.to_string()))),
            Part::Session(_) => Some(Box::new(Session::new())),
            Part::ShellCommand(_, cmd, use_pwd) => Some(Box::new(Shell::new(
                cmd.to_string(),
                *use_pwd,
                DefaultExecutor::default(),
            ))),
            Part::CPU(_) => Some(Box::new(CPU::new())),
            Part::Memory(_) => Some(Box::new(Mem::new())),
            Part::Swap(_) => Some(Box::new(Swap::new())),
            Part::LoadAverage(_) => Some(Box::new(LoadAverage::new())),
            Part::Vpn(_, vpns) => Some(Box::new(Vpn::new(vpns))),
        }
    }

    pub fn build_models(parts: Vec<&Part>) -> Vec<Box<dyn Model>> {
        parts
            .into_iter()
            .filter_map(|part| ModelHelper::new(part))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    pub tmux_session_name: Option<String>,
    pub current_directory: Option<String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            tmux_session_name: None,
            current_directory: None,
        }
    }

    pub fn default() -> Context {
        Context {
            tmux_session_name: None,
            current_directory: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fetch::{Cpu, DiskInfo, LoadAverage, Memory, NetworkInfo, ResourceInfo, Swap},
        state::rehydrator::{CacheKey, CacheSnapshot, CacheValue},
    };

    use super::Context;

    pub fn generate_sample_context() -> Context {
        let tmux_session_name = Some(String::from("my_tmux_session"));
        let current_directory = Some(String::from("/path/to/current_directory"));

        Context {
            tmux_session_name,
            current_directory,
        }
    }

    // Helper function to create a CacheSnapshot with Resources
    pub fn create_cache_snapshot_with_resources() -> CacheSnapshot {
        let network_info = NetworkInfo {
            name: String::from("eth0"),
            ip_addr: Some(String::from("192.168.1.1")),
        };

        let swap_info = Swap {
            total: 8192,
            used: 2048,
        };

        let memory_info = Memory {
            total: 16384,
            used: 8192,
            available: 8192,
        };

        let cpu_info = Cpu { usage: 0.75 };

        let load_average_info = LoadAverage {
            one: 1.5,
            five: 2.0,
            fifteen: 1.8,
        };

        let resource_info = ResourceInfo {
            swap: swap_info,
            memory: memory_info,
            cpu: cpu_info,
            la: load_average_info,
        };

        let disk_info = DiskInfo {
            dev: String::from("/dev/sda"),
            mount_point: String::from("/"),
            total_space: 102400,
            free_space: 51200,
        };

        // Sample CacheValue variants
        let networks_cache_value = CacheValue::Networks(vec![network_info.clone()]);
        let resources_cache_value = CacheValue::Resources(vec![resource_info.clone()]);
        let disk_stats_cache_value = CacheValue::DiskStats(vec![disk_info.clone()]);

        // Sample CacheSnapshot
        let mut cache_snapshot = CacheSnapshot::new();
        cache_snapshot.insert(CacheKey::Networks, networks_cache_value);
        cache_snapshot.insert(CacheKey::Resources, resources_cache_value);
        cache_snapshot.insert(CacheKey::DiskStats, disk_stats_cache_value);

        cache_snapshot
    }
}
