use clap::Parser;
use log::debug;
use rusty_belt::config::parse::parse_config;
use rusty_belt::config::AppConfig;
use rusty_belt::fs::{get_config_path, handle_file_presence};
use rusty_belt::model::{Model, ModelHelper};

use rusty_belt::args::ServerArgs;
use rusty_belt::io::server::Server;
use rusty_belt::state::rehydrator::Rehydrator;
use rusty_belt::state::State;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = ServerArgs::parse();

    let config_folder = get_config_path().ok_or_else(|| "Can't find path for config")?;

    let mut config_file = config_folder.clone();
    config_file.push("config.yaml");

    let mut log_config_file = PathBuf::from(config_folder.clone());
    log_config_file.push("log4rs.yaml");

    let path_to_config = if let Some(path) = args.config_path {
        PathBuf::from(path)
    } else {
        config_file
    };

    handle_file_presence(&path_to_config)?;

    let path_to_log = args
        .log_config_path
        .map(|cp| PathBuf::from(cp))
        .unwrap_or(log_config_file);

    handle_file_presence(&path_to_log)?;

    log4rs::init_file(path_to_log, Default::default()).unwrap();

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
    Ok(())
}
