use clap::Parser;
use rusty_belt::args::CliArgs;
use rusty_belt::config::parse::parse_config;
use rusty_belt::config::AppConfig;
use rusty_belt::fs::{get_config_path, get_data_path, handle_file_presence};
use rusty_belt::io::cli_client::CliClient;
use rusty_belt::protocol::rusty::belt::{self};
use rusty_belt::render::render_response;
use rusty_belt::util::{fetch_tmux_current_path, fetch_tmux_name};

use std::io::{self, Write};
use std::path::PathBuf;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let config_folder = get_config_path().ok_or_else(|| "Can't find path for config")?;

    let mut socket_path = get_data_path().ok_or_else(|| "Can't find path for socket")?;
    socket_path.push("server.socket");

    let socket_file = socket_path.as_path().display().to_string();

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
    let segment_conf = config
        .segments
        .iter()
        .find(|v| v.name == args.segment_name)
        .unwrap();

    let mut request_context = belt::Tmux::default();

    if let Ok(session_name) = fetch_tmux_name() {
        request_context.session_name = session_name;
    }

    if let Ok(pwd) = fetch_tmux_current_path() {
        request_context.pwd = pwd;
    }

    let mut request = belt::Request::default();
    request.segment_name = args.segment_name;
    request.context = Some(belt::request::Context::Tmux(request_context));

    let cli_client = CliClient::new(socket_file);

    if let Ok(response) = cli_client.make_request(request).await {
        let part_str = render_response(response, segment_conf);
        io::stdout().write_all(part_str.as_bytes())?;
    }

    Ok(())
}
