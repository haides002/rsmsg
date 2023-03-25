mod cryption;
pub use crate::cryption::*;

fn main() {
    let to_encrypt = "Rawr";
    let to_decrypt = cryption::encrypt("uwu", to_encrypt);
    print!("{}", cryption::decrypt("uwu", &to_decrypt))

    // what do we need?
    // 1. skill
    // 2. dummy chat (encrypted) chat should be a list of aes encrypted messages
    // 3. decryption functions
    // 4. encryption function
}
