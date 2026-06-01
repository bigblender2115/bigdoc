use std::path::Path;
use std::fs;
use toml;
use clap::Parser;

use crate::config::Config;
use crate::args::Commands;

mod checker;
mod config;
mod types;
mod tools;
mod args;

fn main() {
    let args = args::Args::parse();
    
    let path = if Path::new(".devspec.toml").exists() {
        Path::new(".devspec.toml")
    } else {
        println!("No .devspec.toml file found. \nMake sure .devspec.toml exists in current directory. \n\"bigdoc init\" to create one");
        std::process::exit(1);
    };

    match &args.command {
        Commands::Check => {
            let toml_content = fs::read_to_string(path)
                .expect("Failed to read .devspec.toml file. Please make sure it exists");
            let config: Config =
                toml::from_str(&toml_content).expect("Failed to parse TOML. Check formatting");
            checker::check(config);
        },
        Commands::Init => {
            if Path::new(".devspec.toml").exists() {
                println!("already exists");
                std::process::exit(1);
            }
            
            fs::write(".devspec.toml", "[tools]\n# node = \">=20\"\n# python = \">=3.11\"\n")
                .expect("failed to create .devspec.toml");
            println!("created .devspec.toml");
        },
    }
}
