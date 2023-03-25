mod cryption;
mod io;
pub use crate::cryption::*;
pub use crate::io::*;

fn main() {
    const SEPARATOR: &str = "\\([seperator])\\";
    let messages = get_messages_from_file("chat.txt");
    for msg in messages {
        //println!("'{}'", msg);
        println!("{}\n", decrypt("uwu", &msg));
    }

    // let message = "Zxmon\nHello World!";
    // let encrypted_message = cryption::encrypt("uwu", message);
    // print!("{}", cryption::decrypt("uwu", &encrypted_message));
    // save_file("chat.txt", &format!("{}\n{}\n", &encrypted_message, SEPARATOR));


    // what do we need?
    // 1. skill
    // 2. dummy chat (encrypted) chat should be a list of aes encrypted messages
    // 3. decryption functions
    // 4. encryption function
}
