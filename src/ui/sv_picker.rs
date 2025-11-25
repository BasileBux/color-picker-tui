use crate::constants::*;
use crate::types::Vec2;
use palette::{Hsv, RgbHue, SetHue};
use std::io::{self, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveTo},
    style::Print,
};

use crate::crossterm_commands::*;

pub struct SVPicker {
    pub buf: Vec<u8>,
    pub pos: Vec2,
    pub selected_color: Hsv,
    pub saturation_step: f64,
    pub value_step: f64,
    pub width: u32,
    pub height: u32,
}

impl SVPicker {
    pub fn new(pos: Vec2, width: u32, height: u32) -> Self {
        SVPicker {
            width,
            height,
            pos,
            saturation_step: 1.0 / width as f64,
            value_step: 1.0 / (height * 2) as f64,
            selected_color: Hsv::new(RgbHue::from_degrees(0.0), 1.0, 1.0),
            buf: Vec::with_capacity(height as usize * width as usize * 8),
        }
    }

    pub fn draw(&mut self, fade: bool) -> io::Result<()> {
        let mut pixel = Hsv::new(self.selected_color.hue.into_positive_degrees(), 0.0, 1.0);
        self.buf.clear();
        self.buf
            .queue(MoveTo(self.pos.x as u16, self.pos.y as u16))?;
        for _ in 0..self.height {
            for _ in 0..self.width {
                let mut lower = pixel.clone();
                lower.value = (lower.value - self.value_step as f32).max(0.0);
                self.buf.queue(SetCellPixelsColor(&pixel, &lower, fade))?;
                self.buf.queue(Print(LOWER_HALF_BLOCK))?;
                pixel.saturation += self.saturation_step as f32;
            }
            pixel = Hsv::new(
                pixel.hue.into_positive_degrees(),
                0.0,
                pixel.value - (self.value_step * 2.0) as f32,
            );
            self.buf.queue(MoveLeft(self.width as u16))?;
            self.buf.queue(MoveDown(1))?;
        }
        self.buf.queue(ResetDefaultColors(fade))?;
        stdout().write_all(&self.buf)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn get(&self, x: u32, y: u32) -> Result<Hsv, ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }
        Ok(Hsv::new(
            RgbHue::from_degrees(self.selected_color.hue.into_positive_degrees()),
            x as f32 * self.saturation_step as f32,
            1.0 - (y as f32 * self.value_step as f32 * 2.0),
        ))
    }

    pub fn set_hue(&mut self, hue_degrees: f32) {
        self.selected_color
            .set_hue(RgbHue::from_degrees(hue_degrees));
    }

    pub fn change_color(&mut self, x: u32, y: u32) -> Result<(), ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }
        self.selected_color = self.get(x, y)?;
        Ok(())
    }
}
