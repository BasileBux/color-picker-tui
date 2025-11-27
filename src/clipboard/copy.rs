use arboard::Clipboard;
use std::{
    io::{self},
    process::Command,
};

/// Warning no input validation is done on args inside of this
/// function - be careful when using user input.
pub fn wl_copy(arg: &str) -> io::Result<std::process::Child> {
    Command::new("wl-copy").arg(arg).spawn()
}

// We have to use a special method to copy to clipboard on Wayland because arboard
// will not write to wl-copy so when the prgram is closed, the clipboard is lost.
pub fn clipboard_copy(str: &str) -> io::Result<()> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        wl_copy(str)?;
    } else {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(str).unwrap();
    }
    Ok(())
}
