// Rbxsig/Rbxsig2 Generator
// Written by: Walnut (@worships / @aircanister)
// Description: Keypair generation for rbxsig, and rbxsig2

use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::traits::PublicKeyParts;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;
use base64::{engine::general_purpose, Engine};

fn export_csp_public_blob(public_key: &RsaPublicKey, mod_len: usize) -> Vec<u8> {
    let modulus = public_key.n().to_bytes_le();
    let mut mod_bytes = vec![0u8; mod_len];
    mod_bytes[..modulus.len()].copy_from_slice(&modulus);

    let exp_bytes_vec = public_key.e().to_bytes_le();
    let mut exp_bytes = [0u8; 4];
    for (i, b) in exp_bytes_vec.iter().enumerate().take(4) {
        exp_bytes[i] = *b;
    }
    let exp_3bytes = &exp_bytes[..3];

    let mut blob = vec![];

    blob.extend_from_slice(&[
        0x06,
        0x02,
        0x00, 0x00,
        0x00, 0x24, 0x00, 0x00,
    ]);

    blob.extend_from_slice(&[b'R', b'S', b'A', b'1']);
    blob.extend_from_slice(&(mod_len as u32 * 8).to_le_bytes());

    let mut pubexp = [0u8; 4];
    pubexp[..3].copy_from_slice(exp_3bytes);
    blob.extend_from_slice(&pubexp);
    blob.extend_from_slice(&mod_bytes);

    blob
}

pub fn generate_keypair(key_size: usize, priv_key_name: &str, pub_key_name: &str, blob_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)?;

    let priv_pem = private_key.to_pkcs1_pem(LineEnding::LF)?;
    File::create(priv_key_name)?.write_all(priv_pem.as_bytes())?;

    let public_key = RsaPublicKey::from(&private_key);
    let pub_pem = public_key.to_pkcs1_pem(LineEnding::LF)?;
    File::create(pub_key_name)?.write_all(pub_pem.as_bytes())?;

    let blob = export_csp_public_blob(&public_key, key_size / 8);
    let b64_blob = general_purpose::STANDARD.encode(&blob);
    File::create(blob_name)?.write_all(b64_blob.as_bytes())?;

    Ok(())
}