mod cryption;
pub use crate::cryption::*;

fn main() {
    println!("Hello, world!");
    cryption::crypting::decrypt();
    cryption::crypting::encrypt();
    // what do we need?
    // 1. skill
    // 2. dummy chat (encrypted) chat should be a list of aes encrypted messages
    // 3. decryption functions
    // 4. encryption function
}
