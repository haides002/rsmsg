use std::io;
use std::io::Write;

pub fn display_messages(messages: &Vec<String>) {
    _ = clearscreen::clear();
    for message in messages {
        println!("{}\n", message);
    }
}

pub fn ask_password() -> String {
    _ = clearscreen::clear();
    let mut password = String::new();
    print!("Enter your password > ");
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut password).unwrap();
    _ = clearscreen::clear();
    return password.trim().to_string();
}

pub fn ask_message() -> String {
    let mut new_message = String::new();
    print!("msg > ");
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut new_message).unwrap();
    return new_message;
}
