use crate::constants::*;
use crate::utils::{fade_color, rgb_from_hsv};
use crossterm::Command;
use palette::Hsv;

// All functions under this comment support fading colors for a "disabled" effect

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
