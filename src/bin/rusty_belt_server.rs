use clap::Parser;
use log::debug;
use rusty_belt::config::parse::parse_config;
use rusty_belt::config::AppConfig;
use rusty_belt::model::{Model, ModelHelper};

use rusty_belt::args::ServerArgs;
use rusty_belt::io::server::Server;
use rusty_belt::state::rehydrator::Rehydrator;
use rusty_belt::state::State;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = ServerArgs::parse();

    let path_to_config = if let Some(path) = args.config_path {
        PathBuf::from(path)
    } else {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("./config.yaml");
        d
    };

    log4rs::init_file(
        args.log_config_path.unwrap_or("".to_string()),
        Default::default(),
    )
    .unwrap();

    let config: AppConfig = parse_config(&path_to_config);
    let models: HashMap<String, Vec<Box<dyn Model>>> = config
        .segments
        .iter()
        .map(|s| {
            (
                s.name.clone(),
                ModelHelper::build_models(s.parts.iter().collect()),
            )
        })
        .collect();

    debug!("models = {:?}", models);
    let mut state = State::default();
    state.set_segments(models);
    let arc_state = Arc::new(state);

    debug!("Conf: {:?}", config);

    let rehydrator = Rehydrator::new(arc_state.clone());
    rehydrator.spawn_rehydration_task();

    let addr = env!("HOME").to_string() + "/.local/share/rusty-belt.socket";

    let server = Server::new(arc_state.clone(), addr);
    server.run().await;
}
