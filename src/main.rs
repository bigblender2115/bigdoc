use clap::Parser;
use std::fs;
use std::path::Path;
use toml;

use crate::args::Commands;
use crate::config::Config;

mod args;
mod checker;
mod config;
mod port_checker;
mod tools;
mod types;

fn main() {
    let args = args::Args::parse();

    let path = if Path::new(".devspec.toml").exists() {
        Path::new(".devspec.toml")
    } else {
        println!(
            "No .devspec.toml file found. \nMake sure .devspec.toml exists in current directory. \n\"bigdoc init\" to create one"
        );
        std::process::exit(1);
    };

    match &args.command {
        Commands::Check { fix } => {
            let toml_content = fs::read_to_string(path)
                .expect("Failed to read .devspec.toml file. Please make sure it exists");
            let mut config: Config =
                toml::from_str(&toml_content).expect("Failed to parse TOML. Check formatting");
            if let Some(ports) = config.ports.take() {
                port_checker::check_ports(ports.required);
            }
            checker::check(config, *fix);
        }
        Commands::Init => {
            if Path::new(".devspec.toml").exists() {
                println!(".devspec.toml already exists");
                std::process::exit(1);
            }

            fs::write(".devspec.toml", "[tools]\n# node = \">=20\"\n")
                .expect("failed to create .devspec.toml");
            println!("created .devspec.toml with default node version requirement");
        }
    }
}
