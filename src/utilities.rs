// Utilities
// Written by: Walnut (@worships / @aircanister / @source-value)
// Description: Functions & tools used by the program

use hex;
use std::error::Error;

// replace hex data in the roblox binary file
pub fn replace_hex(data: &mut Vec<u8>, search_hex: &str, replace_hex: &str, verbose: bool) -> Result<(), Box<dyn Error>> {
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
