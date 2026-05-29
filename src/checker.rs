use std::process::Command;
use colored::Colorize;

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
            ver_required: required.to_string()
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

    let mut results: Vec<CheckResult> = Vec::new();

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
                            CheckResult::Valid {tool, ver } => {
                                results.push(CheckResult::Valid { tool, ver });
                            }
                            CheckResult::Outdated { tool, ver, ver_required } => {
                                results.push(CheckResult::Outdated {tool, ver, ver_required });
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        results.push(CheckResult::Missing {tool: tool.clone(), ver_required: version.clone() });
                    }
                }
            }
        } else {
            println!("No command found for '{}'", tool);
        }
    }

    //sorting and printing
    results.sort_by_key(|result| match result {
        CheckResult::Valid {tool: _, ver: _} => 0,
        CheckResult::Outdated {tool: _, ver: _, ver_required: _} => 1,
        CheckResult::Missing {tool: _, ver_required: _} => 2,
    });

    for result in results {
        match result {
            CheckResult::Valid { tool, ver } => {
                println!("{:<12} {:<12} {}", "[OK]".green(), tool, ver);
            }
            CheckResult::Outdated { tool, ver, ver_required } => {
                println!("{:<12} {:<12} {:<10} (required {})", "[OUTDATED]".yellow(), tool, ver, ver_required);
            }
            CheckResult::Missing { tool, ver_required } => {
                println!("{:<12} {:<12} (required {})", "[MISSING]".red().bold(), tool, ver_required);
            }
        }
    }
}
