use crate::state::rehydrator::{CacheKey, CacheSnapshot, CacheValue};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use systemstat::Platform;

pub type UpdateFn = fn(&mut CacheSnapshot, &System, &systemstat::System) -> Result<(), ()>;

#[derive(Debug, PartialEq, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub ip_addr: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Swap {
    pub total: u64,
    pub used: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Memory {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cpu {
    pub usage: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResourceInfo {
    pub swap: Swap,
    pub memory: Memory,
    pub cpu: Cpu,
    pub la: LoadAverage,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DiskInfo {
    pub dev: String,
    pub mount_point: String,
    pub total_space: u64,
    pub free_space: u64,
}

impl DiskInfo {
    pub fn default() -> DiskInfo {
        DiskInfo {
            dev: "".to_string(),
            mount_point: "".to_string(),
            total_space: 0,
            free_space: 0,
        }
    }
}

pub fn fetch_disk_info(
    cache_snapshot: &mut CacheSnapshot,
    _sys: &System,
    platform: &systemstat::System,
) -> Result<(), ()> {
    let mounts = platform.mounts().unwrap_or(Vec::default());
    let mut disks: Vec<DiskInfo> = Vec::new();
    for disk in mounts {
        disks.push(DiskInfo {
            dev: disk.fs_mounted_from.clone(),
            mount_point: disk.fs_mounted_on.clone(),
            free_space: disk.avail.0,
            total_space: disk.total.0,
        });
    }
    cache_snapshot.insert(CacheKey::DiskStats, CacheValue::DiskStats(disks));
    Ok(())
}

pub fn fetch_resources_info(
    cache_snapshot: &mut CacheSnapshot,
    sys: &System,
    _platform: &systemstat::System,
) -> Result<(), ()> {
    // Swap,
    // SwapPercents,
    // Memory,
    // MemoryPercents,
    // CPU,
    // LoadAverage,
    let mut resources: Vec<ResourceInfo> = Vec::new();

    let swap = Swap {
        total: sys.total_swap(),
        used: sys.used_swap(),
    };

    let memory = Memory {
        total: sys.total_memory(),
        used: sys.used_memory(),
        available: sys.available_memory(),
    };

    let cpu = Cpu {
        usage: sys.global_cpu_info().cpu_usage(),
    };

    let la = LoadAverage {
        one: sys.load_average().one,
        five: sys.load_average().five,
        fifteen: sys.load_average().fifteen,
    };

    let resource_info = ResourceInfo {
        cpu,
        la,
        memory,
        swap,
    };

    resources.push(resource_info);
    cache_snapshot.insert(CacheKey::Resources, CacheValue::Resources(resources));
    Ok(())
}

pub fn fetch_networks_info(
    cache_snapshot: &mut CacheSnapshot,
    _sys: &System,
    _platform: &systemstat::System,
) -> Result<(), ()> {
    let mut networks: Vec<NetworkInfo> = Vec::new();

    let interfaces = default_net::get_interfaces();
    for interface in interfaces {
        networks.push(NetworkInfo {
            name: interface.name,
            ip_addr: interface.ipv4.get(0).map(|addr| addr.to_string()),
        })
    }
    cache_snapshot.insert(CacheKey::Networks, CacheValue::Networks(networks));
    Ok(())
}
