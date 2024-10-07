use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct Config {
    accounts: Vec<Account>,
}

#[derive(Deserialize, Serialize)]
struct Account {
    servername: String,
    server: String,
    username: String,
    password: String,
    port: String,
    user_id: String,
    access_token: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap());
    path.push("tsukimi");
    path.push("tsukimi.toml");

    let toml_content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&toml_content)?;
    let json = serde_json::to_string(&config.accounts)?;

    let reg_content = format!(
        r#"Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\Software\GSettings\moe\tsukimi]
"accounts"="{}"
"#,
        json.replace("\"", "\\\"")
    );

    let output_path = env::current_dir()?.join("tsukimi_accounts.reg");
    fs::write(&output_path, reg_content)?;

    println!("Successfully generated .reg file at: {:?}", output_path);
    println!("You can now import this file by double-clicking it or using the 'regedit' command.");

    Ok(())
}
