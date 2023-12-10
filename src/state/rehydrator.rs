use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    time::Duration,
};

use sysinfo::{System, SystemExt};
use tokio::{
    task::{self, JoinHandle},
    time,
};

use crate::{
    fetch::{DiskInfo, NetworkInfo, ResourceInfo},
    state::State,
};

// let STATE: Mutex<CacheSnapshot> = Mutex::new(HashMap::new());

pub struct Rehydrator {
    state: Arc<State>,
    handle: Arc<Option<JoinHandle<()>>>,
    cache: Arc<Mutex<CacheSnapshot>>,
    system: Arc<Mutex<System>>,
}

impl Rehydrator {
    pub fn new(state: Arc<State>) -> Rehydrator {
        Rehydrator {
            state,
            handle: Arc::new(None),
            cache: Arc::new(Mutex::new(HashMap::new())),
            system: Arc::new(Mutex::new(System::new_all())),
        }
    }

    pub fn spawn_rehydration_task(mut self) {
        let forever = task::spawn(async move {
            let state = self.state.clone();
            let cache = self.cache.clone();
            let mut interval = time::interval(Duration::from_secs(2));
            let system = self.system.clone();
            loop {
                interval.tick().await;
                {
                    system.lock().unwrap().refresh_all();
                }
                let r = task(state.clone(), cache.clone(), system.clone()).await;
            }
        });
        self.handle = Arc::new(Some(forever));
    }
}

pub async fn task(
    state: Arc<State>,
    cache_snapshot: Arc<Mutex<CacheSnapshot>>,
    system_state: Arc<Mutex<System>>,
) -> Result<(), ()> {
    let mut cache_snapshot_acquired = cache_snapshot.lock().unwrap();
    let cache = cache_snapshot_acquired.deref_mut();
    let updaters = state.clone().get_updater();
    let system_acquired = system_state.lock().unwrap();
    let system = system_acquired.deref();

    for updater in updaters {
        let _ = updater(cache, system);
    }

    state.update_db(cache);

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum CacheValue {
    Networks(Vec<NetworkInfo>),
    Resources(Vec<ResourceInfo>),
    DiskStats(Vec<DiskInfo>),
}

#[derive(Debug, Hash, PartialEq)]
pub enum CacheKey {
    Networks,
    Resources,
    DiskStats,
}

impl Eq for CacheKey {}

pub type CacheSnapshot = HashMap<CacheKey, CacheValue>;
