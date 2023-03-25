use crate::cryption::*;
use crate::io::*;

pub fn decrypt_messages(encrypted_messages:Vec<String>, key:&str) -> Vec<String>{
    let mut messages:Vec<String> = vec![];
    for msg_ptr in encrypted_messages {
        messages.push(decrypt(key, &msg_ptr));
    }
    return messages;
}