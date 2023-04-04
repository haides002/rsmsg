use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn decrypt(key: &str, to_decrypt: &str) -> String {
    let crypt_algorithm = new_magic_crypt!(key, 256);
    let decryption_result = crypt_algorithm.decrypt_base64_to_string(to_decrypt);
    return match decryption_result {
        Ok(dec) => dec,
        Err(_error) => "Could not decrypt. (perhaps bogus data or wrong key)".to_string(),
    };
}

pub fn encrypt(key: &str, to_encrypt: &str) -> String {
    let crypt_algorithm = new_magic_crypt!(key, 256);
    let base64 = crypt_algorithm.encrypt_bytes_to_base64(to_encrypt);
    return base64;
}
