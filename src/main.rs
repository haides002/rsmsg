mod cryption;
pub use crate::cryption::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};

fn main() {
    println!("Hello, world!");
<<<<<<< HEAD
    cryption::crypting::decrypt();
    cryption::crypting::encrypt();
=======
    cryption::Crypting::decrypt();
    cryption::Crypting::encrypt();

    // https://docs.rs/chacha20poly1305/latest/chacha20poly1305/
    
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, b"Hello World!".as_ref()).unwrap();
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"Hello World!");
    println!("key: {:?}\ncipher: {:?}\nplain: {}",
        key,
        ciphertext,
        String::from_utf8_lossy(&plaintext)
    );
>>>>>>> 29c4788d403af837b8ddb2aeebacc49d0b3dfeeb
    // what do we need?
    // 1. skill
    // 2. dummy chat (encrypted) chat should be a list of aes encrypted messages
    // 3. decryption functions
    // 4. encryption function
}
