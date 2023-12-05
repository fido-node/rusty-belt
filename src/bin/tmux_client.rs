use clap::Parser;
use rusty_belt::args::CliArgs;
use rusty_belt::config::parse::parse_config;
use rusty_belt::config::AppConfig;
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
    let path_to_config = if let Some(path) = args.config_path {
        PathBuf::from(path)
    } else {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("./config.yaml");
        d
    };

    log4rs::init_file(
        args.log_config_path
            .unwrap_or("/home/michey/Projects/personal/rusty-belt/log4rs.yaml".to_string()),
        Default::default(),
    )
    .unwrap();

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

    let addr = env!("HOME").to_string() + "/.local/share/rusty-belt.socket";

    let cli_client = CliClient::new(addr);

    if let Ok(response) = cli_client.make_request(request).await {
        let part_str = render_response(response, segment_conf);
        io::stdout().write_all(part_str.as_bytes())?;
    }

    Ok(())
}
