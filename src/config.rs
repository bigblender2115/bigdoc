use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tools: Option<HashMap<String, String>>,
    pub ports: Option<Ports>,
}

#[derive(Debug, Deserialize)]
pub struct Ports {
    pub required: Vec<u16>,
}

