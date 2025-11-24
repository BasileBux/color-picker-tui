use std::{io::{self, ErrorKind}, process::Command};

// TODO: Add support for X11, windows, darwin clipboards.
// TODO: Add visual feedback for copy/paste operations.

/// Warning no input validation is done on args inside of this
/// function - be careful when using user input.
pub fn wl_copy(arg: &str) -> io::Result<std::process::Child> {
    std::process::Command::new("wl-copy").arg(arg).spawn()
}

/// Warning no output validation is done on the result of this
/// function - be careful when using the result.
pub fn wl_paste() -> Result<String, io::Error> {
    let out = Command::new("wl-paste").output()?;

    if !out.status.success() {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!("wl-paste exited with {}", out.status),
        ));
    }

    String::from_utf8(out.stdout)
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "clipboard contained invalid UTF-8"))
}
