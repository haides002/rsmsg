use crate::io;
use std::fs;

pub fn get_username() -> Result<String, String> {
    match io::read_file(&get_config_path()) {
        Ok(data) => {
            let data: Vec<String> = data.split("\n").map(|f| f.to_string()).collect();
            let username = &data[0].split("=").map(|f| f.to_string()).collect::<Vec<String>>()[1];
            Ok(username.to_owned())
        }
        Err(e) => Err(e),
    }
}

pub fn write_username(username: &str) {
    io::write_file(&get_config_path(), &format!("username={}", username));
}

#[cfg(target_os = "linux")]
fn get_config_path() -> String {
    let config_name = "rsmsg.conf";
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.config/rsmsg/{}", home, config_name);
    fs::create_dir_all(format!("{}/.config/rsmsg", home)).unwrap();
    path
}
