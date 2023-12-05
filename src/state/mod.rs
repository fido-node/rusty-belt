pub mod rehydrator;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::{
    fetch::{self, UpdateFn},
    model::Model,
    protocol::{
        helpers::{build_cli_response, get_context},
        rusty::belt::{segment_value, Request, Response},
    },
    state::rehydrator::CacheKey,
};

use self::rehydrator::CacheSnapshot;

pub type ModelVec = Vec<Box<dyn Model>>;
pub type Db = Arc<Mutex<HashMap<String, ModelVec>>>;

#[derive(Debug)]
pub struct State {
    db: Db,
}

impl State {
    pub fn default() -> State {
        let db: Db = Arc::new(Mutex::new(HashMap::new()));
        State { db }
    }

    pub fn set_segments(&mut self, segments_to_model: HashMap<String, Vec<Box<dyn Model>>>) {
        self.db = Arc::new(Mutex::new(segments_to_model));
    }

    pub fn get_updater(&self) -> Vec<UpdateFn> {
        let mut cache_update_fn = HashSet::new();

        let values = self.db.lock().unwrap();

        for value_cont in values.values().into_iter() {
            for model in value_cont {
                if let Some(cache_key) = model.get_cache_key() {
                    let updater = match cache_key {
                        CacheKey::DiskStats => fetch::fetch_disk_info,
                        CacheKey::Networks => fetch::fetch_networks_info,
                        CacheKey::Resources => fetch::fetch_resources_info,
                    };
                    cache_update_fn.insert(updater);
                };
            }
        }

        cache_update_fn.into_iter().collect()
    }

    pub fn update_db(&self, cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        let mut db = self.db.lock().unwrap();
        for values in db.iter_mut().map(|v| v.1) {
            for model in values.iter_mut() {
                let _ = model.rehydrate(cache_snapshot);
            }
        }
        Ok(())
    }

    pub fn fetch_info(&self, request: Request) -> Result<Response, ()> {
        let ctx = get_context(&request);
        let segment = &request.segment_name;
        let db = self.db.lock().unwrap();
        let segments = db.get(segment);
        let segments_data = segments.map(|segment_state| {
            segment_state
                .iter()
                .map(|model| model.get_state(&ctx))
                .collect::<Vec<segment_value::Segment>>()
        });

        let client_response = segments_data.map(|data| build_cli_response(data));

        let mut response = Response::default();
        response.client_response = client_response;
        Ok(response)
    }
}
