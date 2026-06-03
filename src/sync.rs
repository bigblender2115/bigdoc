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
    let body = reqwest::blocking::get(url)?.text()?;
    fs::write(".devspec.toml", body).expect("failed to write .devspec.toml");
    println!("synced .devspec.toml from {}", url);
    Ok(())
}
