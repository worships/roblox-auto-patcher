// Roblox Auto Patcher
// Written by: Walnut (@worships / @aircanister)
// Description: Automatically patch older roblox clients (2019 and under)

mod constants;
mod gen;
mod utilities;

use clap::{Arg, Command};
use hex;
use std::error::Error;
use std::fs::{self};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("roblox-auto-patcher")
        .version("0.1.0")
        .author("Walnut (@worships / @aircanister)")
        .about("automatically patch older roblox clients!")
        .arg(
            Arg::new("filePath")
                .short('f')
                .long("filePath")
                .value_name("FILE")
                .help("Path to the input executable file (Player, RCC, and Studio)")
                .required(true),
        )
        .arg(
            Arg::new("rbxsig2")
                .short('r')
                .long("rbxsig2")
                .action(clap::ArgAction::SetTrue)
                .help("Generate & replace --rbxsig2% signatures"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("The URL to replace roblox.com with")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose output"),
        )
        .arg(
            Arg::new("overwrite")
                .long("overwrite")
                .short('o')
                .action(clap::ArgAction::SetTrue)
                .help("Overwrite the original executable"),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("filePath").unwrap();
    let url = matches.get_one::<String>("url").unwrap();
    let verbose = matches.get_flag("verbose");
    let overwrite = matches.get_flag("overwrite");
    let rbxsig2 = matches.get_flag("rbxsig2");

    if url.len() != 10 {
        println!("Error: URL must be exactly 10 characters long.");
        return Ok(());
    }
    if !url.chars().all(|c| c.is_ascii_alphanumeric() || c == '.') {
        println!("Error: URL contains invalid characters. Only ASCII alphanumeric characters and periods are allowed.");
        return Ok(());
    }

    let url_hex = hex::encode(url.as_bytes());
    let roblox_hex = hex::encode("roblox.com");
    let start_time = Instant::now();
    let mut file_data = fs::read(file_path)?;

    utilities::replace_hex(&mut file_data, &roblox_hex, &url_hex, verbose)?;

    if !Path::new("rbxsig_public.pub").exists() {
        println!("Generating certificate...");
        gen::generate_keypair(1096, "rbxsig_private.pem", "rbxsig_public.pub")?;
        println!("Certificate generated! You can find it located in the current directory.\n");
    }

    let public_key_hex = read_public_key("rbxsig_public.pub")?;
    utilities::replace_hex(
        &mut file_data,
        constants::RBXSIG_HEX,
        &public_key_hex,
        verbose,
    )?;

    if rbxsig2 {
        if !Path::new("rbxsig2_public.pub").exists() {
            println!("Generating rbxsig2 certificate...");
            gen::generate_keypair(2084, "rbxsig2_private.pem", "rbxsig2_public.pub")?;
            println!(
                "rbxsig2 certificate generated! You can find it located in the current directory.\n"
            );
        }

        let public_key_hex2 = read_public_key("rbxsig2_public.pub")?;
        utilities::replace_hex(
            &mut file_data,
            constants::RBXSIG2_HEX,
            &public_key_hex2,
            verbose,
        )?;
    }

    let output_path = if overwrite {
        file_path.clone()
    } else {
        let mut path = Path::new(file_path).to_path_buf();
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        path.set_file_name(format!("{}_patched.{}", file_stem, extension));
        path.to_str().unwrap().to_string()
    };

    fs::write(&output_path, file_data)?;
    println!("Completed in {:.2?}!", start_time.elapsed());

    Ok(())
}

fn read_public_key(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let public_key_base64 = std::fs::read_to_string(file_name)?;
    let public_key_base64 = public_key_base64
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<&str>>()
        .join("");
    let public_key_hex = hex::encode(public_key_base64.trim());
    Ok(public_key_hex)
}