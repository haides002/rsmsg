mod chat_handler;
mod cryption;
mod io;
mod ui;
pub use crate::chat_handler::*;
pub use crate::cryption::*;
pub use crate::io::*;
pub use crate::ui::*;

pub const SEPARATOR: &str = "\\(seperator)\\";
pub const SERVER: &str = "127.0.0.1:8000";

// const KEY: &str = "uwu";

fn main() {
    let user: String = ui::ask_username();
    let key: String = ui::ask_password();

    loop {
        let chat = chat_handler::get_chat(&key);
        ui::display_messages(&chat);
        chat_handler::process_message(ui::ask_message(), &user, &key);
    }
}
