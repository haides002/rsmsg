mod cryption;
pub use crate::cryption::*;

fn main() {
    //cryption::crypting::decrypt();
    //cryption::crypting::encrypt();
    // 
    // let crypt_algorithm = new_magic_crypt!("UwU", 256);
    // let base64 = crypt_algorithm.encrypt_bytes_to_base64("Rawr");
    // println!("Encrypted: {}", base64);
    // assert_eq!("NQ1QnCJtwBLjcDBVdto8xQ==", base64);
    // assert_eq!("Rawr", crypt_algorithm.decrypt_base64_to_string(&base64).unwrap());
    
    let to_encrypt = "Rawr";
    let to_decrypt = cryption::encrypt("uwu", to_encrypt);
    print!("{}", cryption::decrypt("uwu", &to_decrypt))

    // what do we need?
    // 1. skill
    // 2. dummy chat (encrypted) chat should be a list of aes encrypted messages
    // 3. decryption functions
    // 4. encryption function
}
