use std::process::Command;
use colored::Colorize;
use regex::Regex;

use crate::config::Config;
use crate::types::CheckResult;
use crate::tools::{TOOLS, FIX_COMMANDS};

lazy_static::lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(r"(?:go|v)?(\d+\.\d+\.\d+)").unwrap(); // credit goes to chatgpt for the regex
}

pub fn parse_and_tell(tool: &str, output: &str, required: &str) -> CheckResult {
    let version = VERSION_REGEX.captures(output)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");
    
    // missing tool
    if version.is_empty() {
        return CheckResult::Missing {
            tool: tool.to_string(),
            ver_required: required.to_string()
        };
    }
    let req = match semver::VersionReq::parse(required) {
        Ok(r) => r,
        Err(_) => return CheckResult::InvalidSpec {
            tool: tool.to_string(),
            reason: format!("invalid constraint: {}", required)
        },
    };
    let ver = match semver::Version::parse(version) {
        Ok(v) => v,
        Err(_) => return CheckResult::InvalidSpec {
            tool: tool.to_string(),
            reason: format!("couldn't parse version: {}", version)
        },
    };
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

//suggest fixes instead of fixing coz im lazy. If youre seeing this and want to make it happen, pls help
fn try_fix(tool: &str) {
    if let Some(cmd) = FIX_COMMANDS.get(tool) {
        println!("         fix: {}", cmd);
    } else {
        println!("         fix: no fix command available for '{}'", tool);
    }
}

// checks if some tool/dependancy/lang is [OK], [MISSING] or [OUTDATED]
pub fn check(config: Config, fix: bool) {
    let mut results: Vec<CheckResult> = Vec::new();
    // checks each tool against the configured version
    for (tool, version) in config.tools.unwrap_or_default() {
        if let Some(command_parts) = TOOLS.get(tool.as_str()) { // command_parts is &'static [&'static str]
            // "splitting command into program and args and running it, then capturing the version from the output" - chatgpt, 2026
            if let Some(program) = command_parts.get(0) {
                let args = &command_parts[1..];
                match Command::new(program).args(args).output() { // Pass program (as &str) and args (as &[&str])
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        // some tools print to stderr instead of stdout idk why they'd do that but yeah
                        let output_str = if stdout.trim().is_empty() {
                            String::from_utf8_lossy(&output.stderr).to_string()
                        } else {
                            stdout.to_string()
                        };
                        
                        // println!("DEBUG: '{}'", output_str.trim());
                        
                        let result = parse_and_tell(&tool, output_str.trim(), &version);
                        results.push(result);
                    }
                    Err(_) => {
                        results.push(CheckResult::Missing {tool: tool.clone(), ver_required: version.clone() });
                        continue;
                    }
                }
            } else {
                // tool not in TOOLS map
                println!("Error: Command definition for '{}' is empty in TOOLS map.", tool);
                results.push(CheckResult::Missing {tool: tool.clone(), ver_required: version.clone() });
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
        CheckResult::InvalidSpec {tool: _, reason: _} => 3,
    });

    println!("------------- [TOOLS]-------------");
    
    for result in &results {
        match result {
            CheckResult::Valid { tool, ver } => {
                println!("{:<12} {:<12} {}", "[OK]".green(), tool, ver);
            }
            CheckResult::Outdated { tool, ver, ver_required } => {
                println!("{:<12} {:<12} {:<10} (required {})", "[OUTDATED]".yellow(), tool, ver, ver_required);
                if fix { try_fix(tool); }
            }
            CheckResult::Missing { tool, ver_required } => {
                println!("{:<12} {:<12} (required {})", "[MISSING]".red().bold(), tool, ver_required);
                if fix { try_fix(tool); }
            }
            CheckResult::InvalidSpec { tool, reason } => {
                println!("{:<12} {:<12} {}", "[ERROR]".red().bold(), tool, reason);
            }
        }
    }

    //SUMMARY
    let ok = results.iter().filter(|r| matches!(r, CheckResult::Valid {..})).count();
    let outdated = results.iter().filter(|r| matches!(r, CheckResult::Outdated {..})).count();
    let missing = results.iter().filter(|r| matches!(r, CheckResult::Missing {..})).count();
    let invalid = results.iter().filter(|r| matches!(r, CheckResult::InvalidSpec {..})).count();
    
    println!("\n{}, {}, {}{}", 
        format!("{} ok", ok).green(),
        format!("{} outdated", outdated).yellow(),
        format!("{} missing", missing).red(),
        if invalid > 0 { format!(", {} invalid", invalid).red().to_string() } else { String::new() }
    );
    
    // exit code 1 if any checks fail
    let has_issues = results.iter().any(|r| !matches!(r, CheckResult::Valid {..}));
    if has_issues {
        std::process::exit(1);
    }
}