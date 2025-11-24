use crate::constants::*;
use crate::types::Vec2;
use crate::utils::rgb_from_hsv;
use palette::{Hsv, RgbHue, SetHue};
use std::io::{self, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub struct HuePicker {
    pub buf: Vec<u8>,
    pub pos: Vec2,
    pub width: u32,
    pub height: u32,
    pub hue_step: f32,
}

impl HuePicker {
    pub fn new(pos: Vec2, width: u32, height: u32) -> Self {
        HuePicker {
            width,
            height,
            pos,
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
            let (r, g, b) = rgb_from_hsv(&pixel);
            self.buf
                .queue(SetBackgroundColor(Color::Rgb { r: r, g: g, b: b }))?;
            let lower = Hsv::new(
                (pixel.hue.into_positive_degrees() + self.hue_step) % 360.0,
                1.0,
                1.0,
            );
            let (lower_r, lower_g, lower_b) = rgb_from_hsv(&lower);
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
        self.buf.queue(SetBackgroundColor(Color::Rgb {
            r: BACKGROUND_COLOR.r,
            g: BACKGROUND_COLOR.g,
            b: BACKGROUND_COLOR.b,
        }))?;
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
