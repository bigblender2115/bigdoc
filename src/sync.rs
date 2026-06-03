use std::fs;
use std::path::Path;

pub fn sync_spec(url: &str) -> Result<(), reqwest::Error> {
    if Path::new(".devspec.toml").exists() {
        print!(".devspec.toml already exists, overwrite? (y/n): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("aborted.");
            return Ok(());
        }
    }

    // to make sure its a toml to avoid uhh .devspec.toml to be filled with random stuff
    let response = reqwest::blocking::get(url)?;
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    if !content_type.contains("text/plain") {
        eprintln!("error: URL did not return plain text. Make sure you're using the raw content URL.");
        std::process::exit(1);
    }
    
    let body = response.text()?;
    
    if toml::from_str::<toml::Value>(&body).is_err() {
        eprintln!("error: content is not valid TOML.");
        std::process::exit(1);
    }
    fs::write(".devspec.toml", body).expect("failed to write .devspec.toml");
    println!("synced .devspec.toml from {}", url);
    Ok(())
}
