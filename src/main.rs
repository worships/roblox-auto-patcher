// Roblox Auto Patcher
// Written by: Walnut (@worships / @aircanister / @source-value)
// Description: Automatically patch older roblox clients (2019 and under)

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

    // rbxsig2 patching isnt available yet, will allow for >2017 patching
    // still gotta write a key generator for it
    if rbxsig2 {
        println!("Patching >2017 clients is not possible at this time, please stick to 2017 and under!\n--rbxsig2 patching is currently not available, exiting!");
        return Ok(());
    }

    // safety,  
    // todo: add safe byte adding for longer domains, maybe shorter as well
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

    replace_hex(&mut file_data, &roblox_hex, &url_hex, verbose)?;

    if !Path::new("PublicKeyBlob.txt").exists() {
        println!("Generating certificate...");
        run_key_generator()?;
        println!("Certificate generated! You can find it in the root directory of the project.");
    }

    let public_key_hex = read_public_key()?;

    let cert_hex = "426749414141436B414142535530457841415141414145414151436A62557978394F585442635745416F6E5A4F66416F543759684D532B4C32315777415A6C73456A767A48585170756C7061734E4668433155367442583663385165793266695242584870716268377641433775326E695436644D4C4C715939557A4949306A79784B442F45554F44635148544B70624D313846526F62714C63764B30444E6449614877797072374E526E53576B344E5868744D3076343057372F6D7233355078624A3872513D3D";

    replace_hex(&mut file_data, cert_hex, &public_key_hex, verbose)?;

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

fn replace_hex(data: &mut Vec<u8>, search_hex: &str, replace_hex: &str, verbose: bool) -> Result<(), Box<dyn Error>> {
    let search_bytes = hex::decode(search_hex)?;
    let replace_bytes = hex::decode(replace_hex)?;

    if replace_bytes.len() != search_bytes.len() {
        if verbose {
            println!("Error: Replacement hex string must be the same length as the search hex string");
            println!("Search hex string: {}", search_hex);
            println!("Replacement hex string: {}", replace_hex);
            println!("Search hex length: {}", search_bytes.len());
            println!("Replacement hex length: {}", replace_bytes.len());
        }
        return Err(Box::from("Replacement hex string must be the same length as the search hex string"));
    }

    for i in 0..=(data.len() - search_bytes.len()) {
        if &data[i..i + search_bytes.len()] == &search_bytes[..] {
            data[i..i + replace_bytes.len()].copy_from_slice(&replace_bytes);
            if verbose {
                println!("Replaced at offset 0x{:X}: {} -> {}", i, search_hex, replace_hex);
            }
        }
    }
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