mod chat_handler;
mod config_handler;
mod cryption;
mod io;
mod ui;
pub use crate::chat_handler::*;
pub use crate::config_handler::*;
pub use crate::cryption::*;
pub use crate::io::*;
pub use crate::ui::*;

pub const SEPARATOR: &str = "\\(seperator)\\";
pub const SERVER: &str = "127.0.0.1:8000";

// const KEY: &str = "uwu";

fn main() {
    let server_ip: String = match config_handler::get_server_ip() {
        Ok(ip) => ip,
        Err(_e) => {
            let ip = ui::ask_ip();
            ip
        }
    };
    // let user: String = ui::ask_username();
    let user: String = match config_handler::get_username() {
        Ok(username) => username,
        Err(_e) => {
            let username = ui::ask_username();
            username
        }
    };

    let key: String = ui::ask_password();

    loop {
        let chat = chat_handler::get_chat(&key, &server_ip);
        ui::display_messages(&chat);
        chat_handler::process_message(ui::ask_message(), &user, &key, &server_ip);
    }
}
