use palette::{FromColor, Hsv, RgbHue, SetHue, Srgb};
use std::io::{self, Write, stdout};
use tui_color_picker::types::*;

use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub struct HuePicker {
    pub pos: Vec2,
    pub width: u32,
    pub height: u32,
    pub hue_step: f32,
    pub buf: Vec<u8>,
}

impl HuePicker {
    pub fn new(pos: Vec2, width: u32, height: u32) -> Self {
        HuePicker {
            pos: pos,
            width,
            height,
            hue_step: 360.0 / (height * 2) as f32,
            buf: Vec::with_capacity(height as usize * width as usize * 8),
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.buf.clear();

        let mut pixel = Hsv::new(0.0, 1.0, 1.0);
        self.buf
            .queue(MoveTo(self.pos.x as u16, self.pos.y as u16))?;

        for _ in 0..self.height {
            let (r, g, b) = Srgb::from_color(pixel)
                .into_format::<u8>()
                .into_components();
            self.buf
                .queue(SetBackgroundColor(Color::Rgb { r: r, g: g, b: b }))?;
            let lower = Hsv::new(
                (pixel.hue.into_positive_degrees() + self.hue_step) % 360.0,
                1.0,
                1.0,
            );
            let (lower_r, lower_g, lower_b) = Srgb::from_color(lower)
                .into_format::<u8>()
                .into_components();
            self.buf.queue(SetForegroundColor(Color::Rgb {
                r: lower_r,
                g: lower_g,
                b: lower_b,
            }))?;
            self.buf.queue(Print(
                format!("{}", LOWER_HALF_BLOCK).repeat(self.width as usize),
            ))?;
            pixel.set_hue(RgbHue::from_degrees(
                (pixel.hue.into_positive_degrees() + self.hue_step * 2.0) % 360.0,
            ));
            self.buf.queue(MoveLeft(self.width as u16))?;
            self.buf.queue(MoveDown(1))?;
        }
        self.buf.queue(ResetColor)?;
        stdout().write_all(&self.buf)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn get(&self, x: u32, y: u32) -> Result<f32, ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }
        Ok(self.hue_step * (y * 2) as f32)
    }
}
