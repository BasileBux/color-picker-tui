use palette::Hsv;
use std::io::{self, stdout};
use crate::constants::*;
use crate::utils::rgb_from_hsv;
use crate::types::Vec2;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::*,
};

pub fn draw_value_display(pos: &Vec2, color: &Hsv) -> io::Result<()> {
    let (r, g, b) = rgb_from_hsv(color);
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Rgb { r, g, b }),
        Print(format!("{}", FULL_CELL_BLOCK).repeat(8)),
        ResetColor,
        SetBackgroundColor(Color::Rgb {
            r: BACKGROUND_COLOR.r,
            g: BACKGROUND_COLOR.g,
            b: BACKGROUND_COLOR.b,
        }),
        Print(format!(
            "{}HEX: #{:02x}{:02x}{:02x}{}RGB: {:>3}, {:>3}, {:>3}{}HSL: {:>3.0}, {:>3.2}%, {:>3.2}%",
            SPACE,
            r,
            g,
            b,
            SPACE,
            r,
            g,
            b,
            SPACE,
            color.hue.into_positive_degrees(),
            color.saturation * 100.0,
            color.value * 100.0
        ))
    )?;
    Ok(())
}
