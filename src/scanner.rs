use std::fs;
use std::path::Path;
use regex::Regex;

pub struct DetectedTool {
    pub name: String,
    pub constraint: String,
}

pub struct ScanResult {
    pub tools: Vec<DetectedTool>,
    pub ports: Vec<u16>,
}

pub fn scan() -> ScanResult {
    let mut tools = Vec::new();
    let mut ports = Vec::new();

    tools.extend(scan_package_json());
    tools.extend(scan_cargo_toml());
    tools.extend(scan_go_mod());
    tools.extend(scan_requirements());
    tools.extend(scan_dockerfile());
    tools.extend(scan_docker_compose(&mut ports));
    tools.extend(scan_pom_xml());
    tools.extend(scan_build_gradle());

    tools.reverse();
    dedup(&mut tools);
    tools.reverse();

    ScanResult { tools, ports }
}

fn dedup(tools: &mut Vec<DetectedTool>) {
    let mut seen = std::collections::HashSet::new();
    tools.retain(|t| seen.insert(t.name.clone()));
}

fn scan_package_json() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("package.json").exists() { return tools; }
    let content = fs::read_to_string("package.json").unwrap_or_default();

    // detect package manager from lockfiles
    if Path::new("yarn.lock").exists() {
        tools.push(DetectedTool { name: "yarn".to_string(), constraint: ">=1".to_string() });
    } else if Path::new("pnpm-lock.yaml").exists() {
        tools.push(DetectedTool { name: "pnpm".to_string(), constraint: ">=1".to_string() });
    } else {
        tools.push(DetectedTool { name: "npm".to_string(), constraint: ">=1".to_string() });
    }

    // extract node version from engines field
    let node_re = Regex::new(r#""node"\s*:\s*"([^"]+)""#).unwrap();
    if let Some(caps) = node_re.captures(&content) {
        tools.push(DetectedTool { name: "node".to_string(), constraint: caps[1].to_string() });
    } else {
        tools.push(DetectedTool { name: "node".to_string(), constraint: ">=18".to_string() });
    }

    // extract npm version from engines field
    let npm_re = Regex::new(r#""npm"\s*:\s*"([^"]+)""#).unwrap();
    if let Some(caps) = npm_re.captures(&content) {
        tools.push(DetectedTool { name: "npm".to_string(), constraint: caps[1].to_string() });
    }

    tools
}

fn scan_cargo_toml() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("Cargo.toml").exists() { return tools; }
    let content = fs::read_to_string("Cargo.toml").unwrap_or_default();

    let ver_re = Regex::new(r#"rust-version\s*=\s*"([^"]+)""#).unwrap();
    let constraint = if let Some(caps) = ver_re.captures(&content) {
        format!(">={}", &caps[1])
    } else {
        ">=1".to_string()
    };

    tools.push(DetectedTool { name: "rustc".to_string(), constraint: constraint.clone() });
    tools.push(DetectedTool { name: "cargo".to_string(), constraint });
    tools
}

fn scan_go_mod() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("go.mod").exists() { return tools; }
    let content = fs::read_to_string("go.mod").unwrap_or_default();

    let re = Regex::new(r"^go\s+(\d+\.\d+)").unwrap();
    let constraint = if let Some(caps) = re.captures(&content) {
        format!(">={}", &caps[1])
    } else {
        ">=1".to_string()
    };

    tools.push(DetectedTool { name: "go".to_string(), constraint });
    tools
}

fn scan_requirements() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("requirements.txt").exists() && !Path::new("pyproject.toml").exists() {
        return tools;
    }

    // try to extract python version from pyproject.toml
    let constraint = if Path::new("pyproject.toml").exists() {
        let content = fs::read_to_string("pyproject.toml").unwrap_or_default();
        let re = Regex::new(r#"python_requires\s*=\s*"([^"]+)""#).unwrap();
        if let Some(caps) = re.captures(&content) {
            caps[1].to_string()
        } else {
            ">=3.8".to_string()
        }
    } else {
        ">=3.8".to_string()
    };

    tools.push(DetectedTool { name: "python3".to_string(), constraint });
    tools.push(DetectedTool { name: "pip".to_string(), constraint: ">=1".to_string() });
    tools
}

fn scan_dockerfile() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("Dockerfile").exists() { return tools; }
    tools.push(DetectedTool { name: "docker".to_string(), constraint: ">=20".to_string() });
    tools
}

fn scan_docker_compose(ports: &mut Vec<u16>) -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    let path = if Path::new("docker-compose.yml").exists() {
        "docker-compose.yml"
    } else if Path::new("docker-compose.yaml").exists() {
        "docker-compose.yaml"
    } else {
        return tools;
    };

    let content = fs::read_to_string(path).unwrap_or_default();
    tools.push(DetectedTool { name: "docker".to_string(), constraint: ">=20".to_string() });

    // extract host ports from "- PORT:CONTAINER" mappings
    let port_re = Regex::new(r#"["']?(\d+):\d+["']?"#).unwrap();
    for caps in port_re.captures_iter(&content) {
        if let Ok(port) = caps[1].parse::<u16>() {
            if !ports.contains(&port) {
                ports.push(port);
            }
        }
    }

    tools
}

fn scan_pom_xml() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("pom.xml").exists() { return tools; }
    let content = fs::read_to_string("pom.xml").unwrap_or_default();

    // extract java version from maven.compiler.source or java.version property
    let re = Regex::new(r"<(?:maven\.compiler\.source|java\.version)>(\d+)</").unwrap();
    let constraint = if let Some(caps) = re.captures(&content) {
        format!(">={}", &caps[1])
    } else {
        ">=11".to_string()
    };

    tools.push(DetectedTool { name: "java".to_string(), constraint });
    tools.push(DetectedTool { name: "maven".to_string(), constraint: ">=3".to_string() });
    tools
}

fn scan_build_gradle() -> Vec<DetectedTool> {
    let mut tools = Vec::new();
    if !Path::new("build.gradle").exists() && !Path::new("build.gradle.kts").exists() {
        return tools;
    }

    let path = if Path::new("build.gradle").exists() { "build.gradle" } else { "build.gradle.kts" };
    let content = fs::read_to_string(path).unwrap_or_default();

    // extract java version from sourceCompatibility or JavaVersion
    let re = Regex::new(r#"sourceCompatibility\s*=\s*['"]?(\d+)['"]?"#).unwrap();
    let constraint = if let Some(caps) = re.captures(&content) {
        format!(">={}", &caps[1])
    } else {
        ">=11".to_string()
    };

    tools.push(DetectedTool { name: "java".to_string(), constraint });
    tools
}