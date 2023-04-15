use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;
use std::{fs, path::Path};

pub fn read_file(name: &str) -> Result<String, String> {
    match fs::read_to_string(name) {
        Ok(data) => Ok(data),
        Err(_e) => Err(format!("Could not read file: {}", name)),
    }
}

pub fn save_file(file_name: &str, data: &str) {
    let mut file = if Path::exists(Path::new(&file_name)) {
        fs::OpenOptions::new().append(true).open(file_name).unwrap()
    } else {
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_name)
            .unwrap()
    };
    //let _ = writeln!(file, "{}", data);
    _ = file.write(data.as_bytes());
    _ = file.flush();
}

pub fn write_file(name: &str, data: &str) {
    let mut file = if Path::exists(Path::new(name)) {
        fs::OpenOptions::new()
            .write(true)
            .open(name)
            .expect(format!("Failed to open file {}", name).as_str())
    } else {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(name)
            .unwrap()
    };

    _ = file.write(data.as_bytes());
    _ = file.flush();
}

pub fn get_messages(server_ip: &str) -> String {
    let mut stream = TcpStream::connect(server_ip)
        .expect("Could not connect to server, is the server running?");
    let request_string = "GET / HTTP/1.1\r\n";
    let request = request_string.as_bytes();
    stream.write_all(request).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let response: Vec<String> = response.split("\n\r\n").map(|f| f.to_string()).collect();
    let response = &response[1];
    return response.to_owned();
}

pub fn send_message(message: String, server_ip: &str) -> String {
    let mut stream = TcpStream::connect(server_ip).unwrap();
    let request_string = format!("POST / HTTP/1.1\r\n\r\n{}", &message);
    let request = request_string.as_bytes();
    stream.write_all(request).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    return response;
}
