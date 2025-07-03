use k256::ecdsa::VerifyingKey;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use wasm_bindgen::prelude::*;

const PK_MIN: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
];

const PK_MAX: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RareKey {
    pub private_key: [u8; 32],
    pub address: [u8; 20],
    pub level: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateKeyAndAddress {
    pub private_key: [u8; 32],
    pub address: [u8; 20],
}

#[inline]
fn is_in_range(bytes: &[u8; 32]) -> bool {
    bytes >= &PK_MIN && bytes < &PK_MAX
}

fn pk_to_address(pk_bytes: &[u8; 32], addr_bytes: &mut [u8; 20]) {
    let private_key = k256::SecretKey::from_slice(pk_bytes).unwrap();
    let public_key = private_key.public_key();
    let verifying_key = VerifyingKey::from(public_key).to_encoded_point(false);
    let public_key_bytes = verifying_key.as_bytes();
    let hash = Keccak256::digest(&public_key_bytes[1..]);
    addr_bytes.copy_from_slice(&hash[12..]);
}

#[inline]
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
pub fn generate_rare_keys_batch(level_threshold: u8, batch_size: u32) -> JsValue {
    let mut found_keys = Vec::with_capacity(32);
    let mut pk_bytes = [0u8; 32];
    let mut addr_bytes = [0u8; 20];
    let mut i = 0;

    while i < batch_size {
        OsRng.fill_bytes(&mut pk_bytes);

        if !is_in_range(&pk_bytes) {
            continue;
        }

        pk_to_address(&pk_bytes, &mut addr_bytes);

        let level = calculate_level(&addr_bytes);

        if level >= level_threshold {
            found_keys.push(RareKey {
                private_key: pk_bytes,
                address: addr_bytes,
                level,
            });
        }

        i += 1;
    }

    serde_wasm_bindgen::to_value(&found_keys).unwrap()
}

#[wasm_bindgen]
pub fn find_address_with_mask(mask: &[u8]) -> JsValue {
    if mask.len() != 44 {
        return JsValue::NULL;
    }

    let value_mask = &mask[..20];
    let care_mask = &mask[20..40];
    let batch_size = u32::from_be_bytes(mask[40..44].try_into().unwrap());

    let mut pk_bytes = [0u8; 32];
    let mut addr_bytes = [0u8; 20];
    let mut i = 0;

    while i < batch_size {
        OsRng.fill_bytes(&mut pk_bytes);

        if !is_in_range(&pk_bytes) {
            continue;
        }

        pk_to_address(&pk_bytes, &mut addr_bytes);

        let mut j = 0;

        while j < 20 {
            if (addr_bytes[j] & care_mask[j]) != (value_mask[j] & care_mask[j]) {
                break;
            }

            j += 1;
        }

        if j == 20 {
            let found = PrivateKeyAndAddress {
                private_key: pk_bytes,
                address: addr_bytes,
            };

            return serde_wasm_bindgen::to_value(&found).unwrap();
        }

        i += 1;
    }

    JsValue::NULL
}
