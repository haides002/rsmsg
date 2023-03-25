use std::{fs, env};

pub fn read_file(name: &str) -> String {
    fs::read_to_string(name).expect(&format!("Couldn't read {}", name))
}

pub fn get_messages_from_file(file_name: &str) -> String {
    let messages: String = read_file(file_name);
    const SEPARATOR: &str = "\\([seperator])\\";
    
    return "Hi".to_owned();
}