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
mod scanner;
mod tools;
mod types;
mod sync;

fn main() {
    let args = args::Args::parse();

    match &args.command {
        Commands::Check { fix } => {
            let path = if Path::new(".devspec.toml").exists() {
                Path::new(".devspec.toml")
            } else {
                println!(
                    "No .devspec.toml file found. \nMake sure .devspec.toml exists in current directory. \n\"bigdoc init\" to create one"
                );
                std::process::exit(1);
            };

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
        Commands::Scan => {
            let result = scanner::scan();
            if result.tools.is_empty() && result.ports.is_empty() {
                println!("no recognizable project files found");
                std::process::exit(1);
            }
            let mut content = String::from("[tools]\n");
            for tool in result.tools {
                content.push_str(&format!("{} = \"{}\"\n", tool.name, tool.constraint));
            }
            if !result.ports.is_empty() {
                content.push_str("\n[ports]\nrequired = [");
                let port_strs: Vec<String> = result.ports.iter().map(|p| p.to_string()).collect();
                content.push_str(&port_strs.join(", "));
                content.push_str("]\n");
            }
            fs::write(".devspec.toml", content).expect("failed to write .devspec.toml");
            println!("generated .devspec.toml");
        }
        Commands::Sync { url } => {
            sync::sync_spec(url).expect("failed to sync");
            println!("synced with {}", url);
        }
    }
}
