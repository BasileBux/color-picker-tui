use crate::constants::*;
use crate::types::Vec2;
use crate::utils::rgb_from_hsv;
use palette::Hsv;
use std::io::{self, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print},
    terminal::*,
};
use crate::crossterm_commands::*;

pub fn draw_value_display(pos: &Vec2, color: &Hsv, fade: bool) -> io::Result<()> {
    let (r, g, b) = rgb_from_hsv(color);
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        Clear(ClearType::CurrentLine),
        SetForegroundColorWithFade(color, fade),
        Print(format!("{}", FULL_CELL_BLOCK).repeat(8)),
        ResetDefaultColors(fade),
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
