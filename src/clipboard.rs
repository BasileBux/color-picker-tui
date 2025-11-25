use std::{
    io::{self, ErrorKind, stdout},
    process::Command,
};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo},
    event::{KeyCode, KeyEvent},
    execute,
    style::Print,
};
use palette::Hsv;

use crate::{
    constants::*,
    crossterm_commands::{FillRect, PrintBold, PrintBoldColored, ResetDefaultColors},
    types::Vec2,
    utils::rgb_from_hsv,
};

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

pub fn clipboard_copy(str: &str) -> io::Result<()> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        wl_copy(str)?;
    } else {
        todo!("Add support for other clipboard systems");
    }
    Ok(())
}

pub fn clipboard_paste() -> io::Result<String> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        wl_paste()
    } else {
        todo!("Add support for other clipboard systems");
    }
}

pub enum ColorFormat {
    Hex,
    Rgb,
    Hsv,
}

impl ColorFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "Hex",
            ColorFormat::Rgb => "RGB",
            ColorFormat::Hsv => "HSV",
        }
    }
    pub fn title(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "He[x]",
            ColorFormat::Rgb => "[R]GB",
            ColorFormat::Hsv => "[H]SV",
        }
    }
    pub fn as_char(&self) -> char {
        match self {
            ColorFormat::Hex => 'x',
            ColorFormat::Rgb => 'r',
            ColorFormat::Hsv => 'h',
        }
    }
}

pub fn clear_clipboard_format_selector(pos: Vec2) -> io::Result<()> {
    execute!(
        stdout(),
        ResetDefaultColors(false),
        FillRect(
            &pos,
            COPY_FORMAT_SELECTOR_WIDTH,
            COPY_FORMAT_SELECTOR_HEIGHT
        ),
    )
}

pub fn draw_clipboard_format_selector(pos: Vec2, mut color: Hsv, fade: bool) -> io::Result<()> {
    const TITLE: &str = "Select Copy Format:";

    let (r, g, b) = rgb_from_hsv(&color);
    let hex_str = format!("#{:02X}{:02X}{:02X}", r, g, b);
    let rgb_str = format!("rgb({}, {}, {})", r, g, b);
    let hsv_str = format!(
        "hsv({}, {}, {})",
        color.hue.into_positive_degrees() as u8,
        (color.saturation * 100.0) as u8,
        (color.value * 100.0) as u8
    );

    // Change color for display purposes
    color.saturation = 0.5;
    color.value = 0.95;
    let (r, g, b) = rgb_from_hsv(&color);

    execute!(
        stdout(),
        ResetDefaultColors(fade),
        MoveTo(pos.x as u16, pos.y as u16),
        PrintBold(TITLE),
        MoveLeft(TITLE.len() as u16),
        MoveDown(1),
        Print("He"),
        PrintBoldColored("x", r, g, b),
        Print(":"),
        MoveRight(COPY_FORMAT_SELECTOR_SPACING - 4),
        Print(&hex_str),
        MoveLeft(COPY_FORMAT_SELECTOR_SPACING + hex_str.len() as u16),
        MoveDown(1),
        PrintBoldColored("R", r, g, b),
        Print("GB:"),
        MoveRight(COPY_FORMAT_SELECTOR_SPACING - 4),
        Print(&rgb_str),
        MoveLeft(COPY_FORMAT_SELECTOR_SPACING + rgb_str.len() as u16),
        MoveDown(1),
        PrintBoldColored("H", r, g, b),
        Print("SV:"),
        MoveRight(COPY_FORMAT_SELECTOR_SPACING - 4),
        Print(&hsv_str),
    )?;
    Ok(())
}

pub fn handle_copy_input_format_selection_input(event: KeyEvent, color: Hsv) -> io::Result<bool> {
    match event.code {
        KeyCode::Char('x') => {
            let (r, g, b) = rgb_from_hsv(&color);
            let hex_str = format!("#{:02X}{:02X}{:02X}", r, g, b);
            clipboard_copy(&hex_str)?;
        }
        KeyCode::Char('r') => {
            let (r, g, b) = rgb_from_hsv(&color);
            let rgb_str = format!("rgb({}, {}, {})", r, g, b);
            clipboard_copy(&rgb_str)?;
        }
        KeyCode::Char('h') => {
            let hsv_str = format!(
                "hsv({}, {}, {})",
                color.hue.into_positive_degrees() as u8,
                (color.saturation * 100.0) as u8,
                (color.value * 100.0) as u8
            );
            clipboard_copy(&hsv_str)?;
        }
        _ => {
            return Ok(false);
        }
    }
    Ok(true)
}
