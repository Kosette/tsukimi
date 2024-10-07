use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs::File, io::Write};
use winreg::{enums::*, RegKey};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert tsukimi.toml into .reg file, is default command
    Convert,
    /// Backup all keys in tsukimi registry
    Backup,
}

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
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Convert) {
        Commands::Convert => convert_toml_to_reg()?,
        Commands::Backup => backup_registry()?,
    }

    Ok(())
}

fn convert_toml_to_reg() -> Result<(), Box<dyn Error>> {
    let mut path = env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap());
    path.push("tsukimi");
    path.push("tsukimi.toml");

    let toml_content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&toml_content)?;
    let json = serde_json::to_string(&config.accounts)?;
    let reg_content = format!(
        "Windows Registry Editor Version 5.00\r\n\r\n\
        [HKEY_CURRENT_USER\\Software\\GSettings\\moe\\tsukimi]\r\n\
        \"accounts\"=\"{}\"\r\n",
        json.replace("\"", "\\\"")
    );

    let output_path = env::current_dir()?.join("tsukimi_accounts.reg");
    let mut file = File::create(output_path)?;

    file.write_all(&[0xFF, 0xFE])?;

    for utf16_unit in reg_content.encode_utf16() {
        file.write_all(&utf16_unit.to_le_bytes())?;
    }

    println!(
        "Successfully generated .reg file at: {:?}",
        file.metadata()?.len()
    );
    println!("You can now import this file by double-clicking it or using the 'regedit' command.");

    Ok(())
}

fn backup_registry() -> Result<(), Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\GSettings\moe\tsukimi";
    let key = hkcu.open_subkey(path)?;

    let mut reg_content = String::from("Windows Registry Editor Version 5.00\r\n\r\n");
    reg_content.push_str(&format!("[HKEY_CURRENT_USER\\{}]\r\n", path));

    for value in key.enum_values().map(|x| x.unwrap()) {
        let name = value.0;
        let data = value.1;
        let reg_type = data.vtype;
        let raw_data = data.bytes;

        let formatted_value = match reg_type {
            REG_DWORD => {
                let value = u32::from_le_bytes(raw_data[..4].try_into().unwrap());
                format!("dword:{:08x}", value)
            }
            REG_QWORD => {
                let value = u64::from_le_bytes(raw_data[..8].try_into().unwrap());
                format!("qword:{:016x}", value)
            }
            REG_SZ | REG_EXPAND_SZ => {
                let value = String::from_utf16_lossy(
                    &raw_data
                        .chunks_exact(2)
                        .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
                        .collect::<Vec<u16>>(),
                );
                format!("\"{}\"", value.trim_end_matches('\0').replace("\"", "\\\""))
            }
            REG_MULTI_SZ => {
                let values = String::from_utf16_lossy(
                    &raw_data
                        .chunks_exact(2)
                        .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
                        .collect::<Vec<u16>>(),
                );
                format!(
                    "hex(7):{}",
                    values
                        .split('\0')
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|s| s
                            .encode_utf16()
                            .map(|c| format!("{:04x}", c))
                            .collect::<Vec<String>>()
                            .join(","))
                        .collect::<Vec<String>>()
                        .join(",00,")
                )
            }
            _ => continue,
        };

        reg_content.push_str(&format!("\"{}\"={}\r\n", name, formatted_value));
    }

    let output_path = env::current_dir()?.join("tsukimi_backup.reg");
    let mut file = File::create(output_path)?;

    file.write_all(&[0xFF, 0xFE])?;

    for utf16_unit in reg_content.encode_utf16() {
        file.write_all(&utf16_unit.to_le_bytes())?;
    }

    println!(
        "Successfully backed up registry to: {:?}",
        file.metadata()?.len()
    );

    Ok(())
}
