use std::io::{self, Write, stdout};
use tui_color_picker::types::*;

use crossterm::{
    QueueableCommand,
    cursor::{MoveDown, MoveLeft, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use hsv::hsv_to_rgb;

pub struct SVPicker {
    pub pos: Vec2,
    pub width: u32,
    pub height: u32,
    pub saturation_step: f64,
    pub value_step: f64,
    pub hue: f64,
    pub selected_pos: Vec2,
    pub buf: Vec<u8>,
}

impl SVPicker {
    pub fn new(pos: Vec2, width: u32, height: u32) -> Self {
        SVPicker {
            pos: pos,
            width,
            height,
            saturation_step: 1.0 / width as f64,
            value_step: 1.0 / (height * 2) as f64,
            hue: 0.0,
            selected_pos: Vec2 { x: width - 1, y: 0 },
            buf: Vec::with_capacity(height as usize * width as usize * 8),
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.buf.clear();
        let mut pixel = HSV {
            h: self.hue,
            s: 0.0,
            v: 1.0,
        };
        self.buf
            .queue(MoveTo(self.pos.x as u16, self.pos.y as u16))?;
        for _ in 0..self.height {
            for _ in 0..self.width {
                let (r, g, b) = hsv_to_rgb(pixel.h, pixel.s, pixel.v);
                self.buf
                    .queue(SetBackgroundColor(Color::Rgb { r: r, g: g, b: b }))?;
                let lower = HSV {
                    h: pixel.h,
                    s: pixel.s,
                    v: pixel.v - self.value_step,
                };
                let (lower_r, lower_g, lower_b) = hsv_to_rgb(lower.h, lower.s, lower.v);
                self.buf.queue(SetForegroundColor(Color::Rgb {
                    r: lower_r,
                    g: lower_g,
                    b: lower_b,
                }))?;
                self.buf.queue(Print(LOWER_HALF_BLOCK))?;
                pixel.s += self.saturation_step;
            }
            pixel = HSV {
                h: pixel.h,
                s: 0.0,
                v: pixel.v - self.value_step * 2.0,
            };
            self.buf.queue(MoveLeft(self.width as u16))?;
            self.buf.queue(MoveDown(1))?;
        }
        self.buf.queue(ResetColor)?;
        stdout().write_all(&self.buf)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn get(&self, x: u32, y: u32) -> Result<HSV, ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }
        Ok(HSV {
            h: self.hue,
            s: x as f64 * self.saturation_step,
            v: 1.0 - (y as f64 * self.value_step * 2.0),
        })
    }

    pub fn get_current(&self) -> HSV {
        // It's safe to unwrap here because selected_pos is always within bounds
        self.get(self.selected_pos.x, self.selected_pos.y).unwrap()
    }
}
