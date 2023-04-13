use crate::io;

pub fn get_username() -> Result<String, String> {
    match io::read_file(&get_config_path()) {
        Ok(data) => {
            let data: Vec<String> = data.split("\n").map(|f| f.to_string()).collect();
            let username = &data[0]
                .split("=")
                .map(|f| f.to_string())
                .collect::<Vec<String>>()[1];
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
    let config = format!(
        "{}/rsmsg",
        std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap();
            format!("{}/.config", home)
        }),
    );
    std::fs::create_dir_all(&config).unwrap();
    let config = format!("{}/{}", config, config_name);
    config
}

#[cfg(target_os = "windows")]
fn get_config_path() -> String {
    use home::home_dir;
    use std::path::Path;
    let config_name = "rsmsg.conf";
    let home = &home_dir().expect("Couldn't get home directory");
    let config = Path::join(home, "AppData").join("Roaming").join("rsmsg");
    std::fs::create_dir_all(&config).unwrap();
    return Path::join(&config, config_name).to_str().unwrap().to_string();
}
