use std::process::Command;

use crate::config::Config;
use crate::types::CheckResult;
use crate::tools::TOOLS;


pub fn parse_and_tell(tool: &str, output: &str, required: &str) -> CheckResult {
    let version = output
        .split_whitespace()
        .find(|s| s.trim_start_matches('v').chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .unwrap_or("")
        .trim_start_matches('v');

    // missing tool
    if version.is_empty() {
        return CheckResult::Missing {
            tool: tool.to_string(),
            ver_required: required.to_string(),
        };
    }

    let req = semver::VersionReq::parse(required).expect("invalid version constraint");
    let ver = semver::Version::parse(version).expect("invalid version string");

    //valid or outdated tool
    if req.matches(&ver) {
        CheckResult::Valid {
            tool: tool.to_string(),
            ver: version.to_string(),
        }
    } else {
        CheckResult::Outdated {
            tool: tool.to_string(),
            ver: version.to_string(),
            ver_required: required.to_string(),
        }
    }
}

// pretty much the entire logic
pub fn check(config: Config) {
    // checks each tool against the configured version
    for (tool, version) in config.tools {
        if let Some(command) = TOOLS.get(tool.as_str()) {
            // splitting command into program and args and running it, then capturing the version from the output
            let mut parts = command.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();

                match Command::new(program).args(&args).output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        // some tools print to stderr instead of stdout
                        let output_str = if stdout.trim().is_empty() {
                            String::from_utf8_lossy(&output.stderr).to_string()
                        } else {
                            stdout.to_string()
                        };

                        // println!("DEBUG: '{}'", output_str.trim());
                        
                        let result = parse_and_tell(&tool, output_str.trim(), &version);
                        match result {
                            CheckResult::Valid { tool, ver } => {
                                println!("{:<12} {:<12} {}", "[OK]", tool, ver)
                            }
                            CheckResult::Outdated {
                                tool,
                                ver,
                                ver_required,
                            } => println!(
                                "{:<12} {:<12} {:<10} (required {})",
                                "[OUTDATED]", tool, ver, ver_required
                            ),
                            CheckResult::Missing { tool, ver_required } => println!(
                                "{:<12} {:<12} (required {})",
                                "[MISSING]", tool, ver_required
                            ),
                        }
                    }
                    Err(_e) => {
                        println!("{:<12} {:<12} (required {})", "[MISSING]", tool, version);
                    }
                }
            }
        } else {
            println!("No command found for '{}'", tool);
        }
    }
}
