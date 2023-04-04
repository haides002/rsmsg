mod chat_handler;
mod cryption;
mod io;
mod ui;
pub use crate::chat_handler::*;
pub use crate::cryption::*;
pub use crate::io::*;
pub use crate::ui::*;

pub const SEPARATOR: &str = "\\(seperator)\\";

// const KEY: &str = "uwu";

fn main() {
    let user = ui::ask_username();
    let key = ui::ask_password();
    let chat = chat_handler::get_chat(&key);
    ui::display_messages(&chat);
    process_message(ui::ask_message(), user, key);
}
