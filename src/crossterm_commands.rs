use crate::constants::*;
use crate::types::Vec2;
use crate::utils::{fade_color, rgb_from_hsv};
use crossterm::Command;
use crossterm::cursor::{MoveDown, MoveLeft, MoveTo};
use palette::Hsv;

pub struct SetForegroundColorWithFade<'a>(pub &'a Hsv, pub bool);
impl<'a> Command for SetForegroundColorWithFade<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let mut color = *self.0;
        if self.1 {
            color = fade_color(color)
        }
        let (r, g, b) = rgb_from_hsv(&color);
        write!(f, "\x1b[38;2;{};{};{}m", r, g, b)
    }
}

pub struct ResetDefaultColors(pub bool);
impl Command for ResetDefaultColors {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let (r, g, b) = if self.0 {
            (FADED_TEXT_COLOR.r, FADED_TEXT_COLOR.g, FADED_TEXT_COLOR.b)
        } else {
            (TEXT_COLOR.r, TEXT_COLOR.g, TEXT_COLOR.b)
        };
        write!(
            f,
            "\x1b[38;2;{};{};{};48;2;{};{};{}m",
            r, g, b, BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b
        )
    }
}

pub struct SetCellPixelsColor<'a>(pub &'a Hsv, pub &'a Hsv, pub bool);
impl<'a> Command for SetCellPixelsColor<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let mut top = *self.0;
        let mut bottom = *self.1;
        if self.2 {
            top = fade_color(top);
            bottom = fade_color(bottom);
        }
        let (top_r, top_g, top_b) = rgb_from_hsv(&top);
        let (bottom_r, bottom_g, bottom_b) = rgb_from_hsv(&bottom);
        write!(
            f,
            "\x1b[48;2;{};{};{};38;2;{};{};{}m",
            top_r, top_g, top_b, bottom_r, bottom_g, bottom_b
        )
    }
}

pub struct FillRect<'a>(pub &'a Vec2, pub u16, pub u16);
impl<'a> Command for FillRect<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "{}", MoveTo(self.0.x as u16, self.0.y as u16))?;
        for _ in 0..(self.2) {
            write!(
                f,
                "{}{}{}",
                " ".repeat((self.1) as usize),
                MoveDown(1),
                MoveLeft(self.1)
            )?;
        }
        Ok(())
    }
}

/// Pos, width, height, rounded
pub struct DrawBoxBorder<'a>(pub &'a Vec2, pub u16, pub u16, pub bool);
impl<'a> Command for DrawBoxBorder<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let (top_left, top_right, bottom_left, bottom_right) = if self.3 {
            (
                TOP_LEFT_ROUNDED,
                TOP_RIGHT_ROUNDED,
                BOTTOM_LEFT_ROUNDED,
                BOTTOM_RIGHT_ROUNDED,
            )
        } else {
            (
                TOP_LEFT_SHARP,
                TOP_RIGHT_SHARP,
                BOTTOM_LEFT_SHARP,
                BOTTOM_RIGHT_SHARP,
            )
        };
        // Draw box
        write!(f, "{}", MoveTo(self.0.x as u16, self.0.y as u16))?;
        write!(f, "{}", top_left)?;
        for _ in 0..(self.1 - 2) {
            write!(f, "{}", HORIZONTAL_LINE)?;
        }
        write!(f, "{}", top_right)?;
        for _ in 1..(self.2 - 1) {
            write!(f, "{}{}{}", MoveDown(1), MoveLeft(1), VERTICAL_LINE)?;
        }
        write!(f, "{}{}{}", MoveDown(1), MoveLeft(1), bottom_right)?;
        write!(f, "{}", MoveTo(self.0.x as u16, self.0.y as u16))?;
        for _ in 0..(self.2 - 1) {
            write!(f, "{}{}{}", MoveDown(1), VERTICAL_LINE, MoveLeft(1))?;
        }
        write!(f, "{}", bottom_left)?;
        for _ in 0..(self.1 - 2) {
            write!(f, "{}", HORIZONTAL_LINE)?;
        }

        // Clear inside of box
        write!(f, "{}", MoveTo(self.0.x as u16 + 1, self.0.y as u16 + 1))?;
        for _ in 0..(self.2 - 2) {
            write!(
                f,
                "{}{}{}",
                " ".repeat((self.1 - 2) as usize),
                MoveDown(1),
                MoveLeft(self.1 - 2)
            )?;
        }
        Ok(())
    }
}

pub struct PrintBold<'a>(pub &'a str);
impl<'a> Command for PrintBold<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, "\x1b[1m{}\x1b[22m", self.0)
    }
}

pub struct PrintBoldColored<'a>(pub &'a str, pub u8, pub u8, pub u8);
impl<'a> Command for PrintBoldColored<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(
            f,
            "\x1b[1;38;2;{};{};{}m{}\x1b[22;39m",
            self.1, self.2, self.3, self.0
        )
    }
}
