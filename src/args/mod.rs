use clap::{command, Parser};

/// Belt client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    ///Name of segment from config
    #[arg(short, long)]
    pub segment_name: String,

    ///Path to YAML config file
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[arg(short, long)]
    pub log_config_path: Option<String>,
}

/// Belt client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ServerArgs {
    ///Path to YAML config file
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[arg(short, long)]
    pub log_config_path: Option<String>,
}
