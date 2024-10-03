use clap::Parser;
use config::args;

pub mod config;

fn main() {
    let config = args::Cli::parse();
    println!("{:#?}", config);
}
