use std::path::Path;
use std::fs;
use toml;

use crate::config::Config;

mod checker;
mod config;
mod types;
mod tools;

fn main() {
    let path = Path::new(".devspec.toml");
    let toml_content = fs::read_to_string(path)
        .expect("Failed to read .devspec.toml file. Please make sure it exists");
    let config: Config =
        toml::from_str(&toml_content).expect("Failed to parse TOML. Check formatting");

    checker::check(config);
}
