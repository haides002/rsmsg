use std::io;
use std::io::Write;
use term_size;

pub fn display_messages(messages: &Vec<String>) {
    _ = clearscreen::clear();
    for message in messages {
        println!("{}\n", message);
    }
}
pub fn ask_ip() -> String {
    let ip_message = "Enter the ip of the server you want to use: ";
    // Get terminal size
    let (width, height) = term_size::dimensions().unwrap();
    // clear the screen
    _ = clearscreen::clear();
    // get the password
    let mut ip = String::new();
    for _ in 0..(height / 2) {
        println!("");
    }
    if !(width / 2 < ip_message.len()) {
        for _ in 0..(width / 2 - ip_message.len()) {
            print!(" ");
        }
    }
    print!("{}", ip_message);
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut ip).unwrap();
    _ = clearscreen::clear();
    return ip.trim().to_string();
}

pub fn ask_password() -> String {
    let password_message = "Enter your password: ";
    // Get terminal size
    let (width, height) = term_size::dimensions().unwrap();
    // clear the screen
    _ = clearscreen::clear();
    // get the password
    let mut password = String::new();
    for _ in 0..(height / 2) {
        println!("");
    }
    if !(width / 2 < password_message.len()) {
        for _ in 0..(width / 2 - password_message.len()) {
            print!(" ");
        }
    }
    print!("{}", password_message);
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut password).unwrap();
    _ = clearscreen::clear();
    return password.trim().to_string();
}

pub fn ask_message() -> String {
    let mut new_message = String::new();
    print!("rsmsg message: ");
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut new_message).unwrap();
    return new_message;
}

pub fn ask_username() -> String {
    let username_message = "Enter your username: ";
    let (width, height) = term_size::dimensions().unwrap();
    _ = clearscreen::clear();
    let mut username = String::new();
    for _ in 0..(height / 2) {
        println!("");
    }
    if !(width / 2 < username_message.len()) {
        for _ in 0..(width / 2 - username_message.len()) {
            print!(" ");
        }
    }
    print!("{}", username_message);
    _ = std::io::stdout().flush();
    io::stdin().read_line(&mut username).unwrap();
    _ = clearscreen::clear();
    return username.trim().to_string();
}
