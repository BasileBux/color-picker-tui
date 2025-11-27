use arboard::Clipboard;
use std::io::{self};

pub fn clipboard_copy(str: &str) -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(str).unwrap();
    Ok(())
}
