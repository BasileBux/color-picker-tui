use crate::constants::*;
use crate::types::Vec2;
use crate::utils::rgb_from_hsv;
use palette::Hsv;
use std::io::{self, stdout};

use crate::crossterm_commands::*;
use crossterm::{cursor::MoveTo, execute, style::Print, terminal::*};

pub fn draw_value_display(pos: &Vec2, color: &Hsv, fade: bool) -> io::Result<()> {
    let (r, g, b) = rgb_from_hsv(color);
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        Clear(ClearType::CurrentLine),
        SetForegroundColorWithFade(color, fade),
        Print(format!("{}", FULL_CELL_BLOCK).repeat(8)),
        ResetDefaultColors(fade),
        Print(SPACE),
        PrintBold("HEX: "),
        Print(format!("#{:02X}{:02X}{:02X}", r, g, b)),
        Print(SPACE),
        PrintBold("RGB: "),
        Print(format!("{:>3}, {:>3}, {:>3}", r, g, b)),
        Print(SPACE),
        PrintBold("HSV: "),
        Print(format!(
            "{:>3.0}, {:>3.2}%, {:>3.2}%",
            color.hue.into_positive_degrees(),
            color.saturation * 100.0,
            color.value * 100.0
        )),
    )?;
    Ok(())
}
