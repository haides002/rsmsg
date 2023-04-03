use crate::*;
use chrono;

pub fn get_chat(key: &str) -> Vec<String> {
    return decrypt_messages(seperate_messages(io::get_messages("chat.txt")), key);
}

pub fn decrypt_messages(encrypted_messages: Vec<String>, key: &str) -> Vec<String> {
    let mut messages: Vec<String> = vec![];
    for msg_ptr in encrypted_messages {
        messages.push(decrypt(key, &msg_ptr));
    }
    return messages;
}

pub fn seperate_messages(messages: String) -> Vec<String> {
    let mut split_messages: Vec<String> = messages
        .split(crate::SEPARATOR)
        .map(|str| str.trim().to_owned())
        .collect();
    _ = split_messages.pop();
    return split_messages;
}

pub fn process_message(mut message: String, user: String, key: String) {
    if false {
        todo!();
    } else {
        let current_time = chrono::Local::now();
        message = format!("---\n{}\n{}\n---\n{}", user, current_time, message);
        let return_code = send_message(encrypt(&key, &message));
        print!("{}", return_code);
    }
}
