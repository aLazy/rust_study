use std::{collections::HashMap, io::Read};

use crate::cli::TextsigFormat;
use anyhow::Result;
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

pub fn process_encrypt_func(key: &[u8], nonce: &[u8], plan_text: &mut dyn Read) -> Result<Vec<u8>> {
    let generate_key = GenericArray::from_slice(key);
    let cipher = ChaCha20Poly1305::new(generate_key);
    let generate_nonce = GenericArray::from_slice(nonce);
    let mut input = String::new();
    plan_text.read_to_string(&mut input).unwrap();
    let cipher_text = cipher.encrypt(generate_nonce, input.as_bytes()).unwrap();
    Ok(cipher_text)
}
pub fn process_decrypt_fun(key: &[u8], nonce: &[u8], cipher_text: &[u8]) -> Result<Vec<u8>> {
    let generate_key = GenericArray::from_slice(key);
    let generate_nonce = GenericArray::from_slice(nonce);
    let cipher = ChaCha20Poly1305::new(generate_key);
    let plain_text = cipher.decrypt(generate_nonce, cipher_text).unwrap();
    Ok(plain_text)
}

pub fn process_key_generate(format: TextsigFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextsigFormat::Chacha20Poly1305 => ChaCha20::generate(),
    }
}

struct ChaCha20 {}
impl ChaCha20 {
    pub fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut map = HashMap::new();
        map.insert("key.key", key.to_vec());
        map.insert("nonce.key", nonce.to_vec());
        Ok(map)
    }
}
