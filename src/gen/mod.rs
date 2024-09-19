// Rbxsig/Rbxsig2 Generator
// Written by: Walnut (@worships / @aircanister)
// Description: Keypair generation for rbxsig, and rbxsig2

use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;

pub fn generate_keypair(key_size: usize, priv_key_name: &str, pub_key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)?;

    let priv_pem = private_key.to_pkcs1_pem(LineEnding::LF)?;
    let mut priv_file = File::create(priv_key_name)?;
    priv_file.write_all(priv_pem.as_bytes())?;

    let public_key = RsaPublicKey::from(&private_key);
    let pub_pem = public_key.to_pkcs1_pem(LineEnding::LF)?;
    let mut pub_file = File::create(pub_key_name)?;
    pub_file.write_all(pub_pem.as_bytes())?;

    Ok(())
}