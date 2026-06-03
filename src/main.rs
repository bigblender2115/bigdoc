use clap::Parser;
use std::fs;
use std::path::Path;
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
                eprintln!("No .devspec.toml file found.\nMake sure .devspec.toml exists in current directory.\n\"bigdoc init\" to create one");
                std::process::exit(1);
            };
            let toml_content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("failed to read .devspec.toml: {}", e);
                    std::process::exit(1);
                }
            };
            let mut config: Config = match toml::from_str(&toml_content) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("failed to parse .devspec.toml: {}", e);
                    std::process::exit(1);
                }
            };
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
            match fs::write(".devspec.toml", "[tools]\n# node = \">=20\"\n") {
                Ok(_) => println!("created .devspec.toml"),
                Err(e) => {
                    eprintln!("failed to create .devspec.toml: {}", e);
                    std::process::exit(1);
                }
            }
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
            match fs::write(".devspec.toml", content) {
                Ok(_) => println!("generated .devspec.toml"),
                Err(e) => {
                    eprintln!("failed to write .devspec.toml: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Sync { url } => {
            if let Err(e) = sync::sync_spec(url) {
                eprintln!("failed to sync: {}", e);
                std::process::exit(1);
            }
        }
    }
}