use std::io::Write;
use std::{fs, path::Path};
use std::io::prelude::*;
use std::net::TcpStream;

pub fn read_file(name: &str) -> String {
    fs::read_to_string(name).expect(&format!("Couldn't read {}", name))
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

pub fn get_messages() -> String {
    let mut stream = TcpStream::connect(crate::SERVER).unwrap();
    let request_string = "GET / HTTP/1.1\r\n";
    let request = request_string.as_bytes();
    stream.write_all(request).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let response: Vec<String> = response.split("\n\r\n").map(|f| f.to_string() ).collect();
    let response = &response[1];
    return response.to_owned();
}

pub fn send_message(message: String) -> String {
    let mut stream = TcpStream::connect(crate::SERVER).unwrap();
    let request_string = format!("POST / HTTP/1.1\r\n\r\n{}", &message);
    let request = request_string.as_bytes();
    stream.write_all(request).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    return response;
}
