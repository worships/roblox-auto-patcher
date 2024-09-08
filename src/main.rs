// Roblox Auto Patcher
// Written by: Walnut (@worships / @aircanister / @source-value)
// Description: Automatically patch older roblox clients (2019 and under)

mod utilities; // Import the utilities module
mod constants; // Import the constants module

use clap::{Arg, Command};
use hex;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::time::{Duration, Instant};
use std::thread;

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

    if rbxsig2 {
        println!("Patching >2017 clients is not possible at this time, please stick to 2017 and under!\n--rbxsig2 patching is currently not available, exiting!");
        return Ok(());
    }

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

    // Use the replace_hex function from the utilities module
    utilities::replace_hex(&mut file_data, &roblox_hex, &url_hex, verbose)?;

    if !Path::new("PublicKeyBlob.txt").exists() {
        println!("Generating certificate...");
        run_key_generator()?;
        println!("Certificate generated! You can find it in the root directory of the project.");
    }

    let public_key_hex = read_public_key()?;

    // Use the CERT_HEX constant from the constants module
    utilities::replace_hex(&mut file_data, constants::CERT_HEX, &public_key_hex, verbose)?;

    let output_path = if overwrite {
        file_path.clone()
    } else {
        format!("{}_PATCHED.exe", Path::new(&file_path).file_stem().unwrap().to_str().unwrap())
    };

    let mut output_file = File::create(output_path)?;
    output_file.write_all(&file_data)?;

    let duration = start_time.elapsed();
    println!("Patched successfully in {}ms!", duration.as_millis());

    Ok(())
}

fn run_key_generator() -> Result<(), Box<dyn Error>> {
    let output = ProcessCommand::new("KeyGenerator/Roblox.KeyGenerator.exe")
        .output()?;

    if !output.status.success() {
        return Err(Box::from("Failed to run key generator executable"));
    }

    thread::sleep(Duration::from_secs(5));
    Ok(())
}

fn read_public_key() -> Result<String, Box<dyn Error>> {
    let public_key_base64 = fs::read_to_string("PublicKeyBlob.txt")?;
    let public_key_hex = hex::encode(public_key_base64.trim());
    Ok(public_key_hex)
}