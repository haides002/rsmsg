use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn decrypt(key: &str,to_decrypt: &str) -> String {
    let crypt_algorithm = new_magic_crypt!(key, 256);
    return crypt_algorithm.decrypt_base64_to_string(to_decrypt).unwrap();
}

pub fn encrypt(key: &str,to_encrypt: &str) -> String {
    let crypt_algorithm = new_magic_crypt!(key, 256);
    let base64 = crypt_algorithm.encrypt_bytes_to_base64(to_encrypt);
    return base64;
}
