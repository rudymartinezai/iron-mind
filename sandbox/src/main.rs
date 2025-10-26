mod firewall;
mod packet;
mod config;
mod utils;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    utils::log_startup(&args.config);

    let rules = config::load_rules(&args.config);
    firewall::run(rules);
}
