use clap::{Parser, Subcommand};
use log::error;
use std::path::PathBuf;
use std::process::exit;

mod commands;
mod config;
mod logging;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Get current ip.
    Ip,

    /// Update host(s) with current ip.
    Update {
        /// The host(s) to update
        #[arg(required = true, long, short = 'H')]
        host: Vec<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GlobalOpts {
    /// The configuration file
    #[arg(long, short, default_value = "ydns.yaml")]
    config: String,

    /// Optional log file
    #[arg(long, short)]
    logfile: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = logging::setup(&cli.global_opts) {
        error!("Could not setup logging: {}", e);
        exit(1)
    }

    let config = match config::load_and_validate(&cli.global_opts.config) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Could not read the configuration file {}: {}",
                &cli.global_opts.config, e
            );
            exit(1)
        }
    };

    match cli.command {
        Command::Ip => commands::get_ip(&config).await,
        Command::Update { host } => commands::update(&config, host).await,
    }
}
