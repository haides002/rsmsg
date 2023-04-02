use std::io::Write;
use std::{fs, path::Path};

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

pub fn get_messages(file_name: &str) -> String {
    let messages: String = read_file(file_name);
    return messages;
}
