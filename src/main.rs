mod chat_handler;
mod cryption;
mod io;
mod ui;
pub use crate::chat_handler::*;
pub use crate::cryption::*;
pub use crate::io::*;
pub use crate::ui::*;

pub const SEPARATOR: &str = "\\([seperator])\\";

// const KEY: &str = "uwu";

fn main() {
    let key = ask_password();
    let chat = get_chat(&key);
    display_messages(&chat);
}
