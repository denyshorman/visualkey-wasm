use k256::ecdsa::VerifyingKey;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use wasm_bindgen::prelude::*;

const PK_MIN: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
];

const PK_MAX: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x40,
];

#[derive(Serialize)]
pub struct RareKeyResult {
    pub private_key: String,
    pub address: String,
    pub level: u8,
}

fn generate_random_bytes() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

fn is_in_range(bytes: &[u8; 32]) -> bool {
    bytes > &PK_MIN && bytes < &PK_MAX
}

fn bytes_to_pk(bytes: &[u8; 32]) -> k256::SecretKey {
    k256::SecretKey::from_slice(bytes).unwrap()
}

fn pk_to_address(pk: &k256::SecretKey) -> [u8; 20] {
    let public_key = pk.public_key();
    let verifying_key = VerifyingKey::from(public_key).to_encoded_point(false);
    let public_key_bytes = verifying_key.as_bytes();
    let hash = Keccak256::digest(&public_key_bytes[1..]);
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

fn calculate_level(address: &[u8; 20]) -> u8 {
    let mut level = 0;
    
    for &byte in address {
        if byte == 0 {
            level += 8;
        } else {
            level += byte.leading_zeros() as u8;
            break;
        }
    }
    
    level
}

#[wasm_bindgen]
pub fn find_rare_key_batch(level_threshold: u8, batch_size: u32) -> JsValue {
    let mut found_keys = Vec::new();
    
    for _ in 0..batch_size {
        let mut pk_bytes = generate_random_bytes();
        
        while !is_in_range(&pk_bytes) {
            pk_bytes = generate_random_bytes();
        }

        let pk = bytes_to_pk(&pk_bytes);
        let address = pk_to_address(&pk);
        let level = calculate_level(&address);

        if level >= level_threshold {
            found_keys.push(RareKeyResult {
                private_key: hex::encode(pk_bytes),
                address: hex::encode(address),
                level,
            });
        }
    }
    
    serde_wasm_bindgen::to_value(&found_keys).unwrap()
}
