mod cryption;
mod io;
mod chat_handler;
mod ui;
pub use crate::cryption::*;
pub use crate::io::*;
pub use crate::chat_handler::*;
pub use crate::ui::*;

pub const SEPARATOR: &str = "\\([seperator])\\";

fn main() {
    const KEY: &str = "uwu";
    

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
