use palette::Hsv;
use std::io::{self, Write, stdout};
use tui_color_picker::types::*;
use tui_color_picker::constants::*;

use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveDown, MoveLeft, MoveTo, Show},
    event::KeyCode,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use tui_color_picker::utils::rgb_from_hsv;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Hex,
    R,
    G,
    B,
    H,
    S,
    V,
    NONE,
}

impl Focus {
    pub fn value(&self) -> u16 {
        match self {
            Focus::Hex => INPUTS_CB_HEIGHT,
            Focus::R => INPUTS_CB_HEIGHT + 2,
            Focus::G => INPUTS_CB_HEIGHT + 3,
            Focus::B => INPUTS_CB_HEIGHT + 4,
            Focus::H => INPUTS_CB_HEIGHT + 6,
            Focus::S => INPUTS_CB_HEIGHT + 7,
            Focus::V => INPUTS_CB_HEIGHT + 8,
            Focus::NONE => 0,
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            Focus::Hex => "#",
            Focus::R => "R ",
            Focus::G => "G ",
            Focus::B => "B ",
            Focus::H => "H ",
            Focus::S => "S ",
            Focus::V => "V ",
            Focus::NONE => "",
        }
    }

    pub fn input_max_len(&self) -> usize {
        match self {
            Focus::Hex => 6,
            Focus::R => 3,
            Focus::G => 3,
            Focus::B => 3,
            Focus::H => 3,
            Focus::S => 3,
            Focus::V => 3,
            Focus::NONE => 0,
        }
    }
}

pub struct Inputs {
    pub buf: Vec<u8>,
    pub pos: Vec2,
    input_str: String,
    pub focus: Focus,
    pub modified: bool,
}

impl Inputs {
    pub fn new(pos: Vec2) -> Inputs {
        Inputs {
            pos,
            focus: Focus::NONE,
            buf: Vec::with_capacity(128 * 8),
            input_str: String::with_capacity(8),
            modified: false,
        }
    }

    pub fn draw(&mut self, color: &Hsv) -> io::Result<()> {
        self.buf.clear();
        let (r, g, b) = rgb_from_hsv(&color);
        self.buf
            .queue(MoveTo(self.pos.x as u16, self.pos.y as u16))?;
        self.buf
            .queue(SetForegroundColor(Color::Rgb { r: r, g: g, b: b }))?;

        // Draw color block
        for _ in 0..INPUTS_CB_HEIGHT {
            self.buf.queue(Print(
                format!("{}", FULL_CELL_BLOCK).repeat(INPUTS_CB_WIDTH as usize),
            ))?;
            self.buf.queue(MoveDown(1))?;
            self.buf.queue(MoveLeft(INPUTS_CB_WIDTH))?;
        }

        self.buf.queue(ResetColor)?;
        self.buf.queue(SetBackgroundColor(Color::Rgb {
            r: BACKGROUND_COLOR.r,
            g: BACKGROUND_COLOR.g,
            b: BACKGROUND_COLOR.b,
        }))?;

        // Draw values
        self.buf
            .queue(Print(format!("#{:02x}{:02x}{:02x}", r, g, b)))?;
        self.buf.queue(MoveLeft(7))?;
        self.buf.queue(MoveDown(2))?;

        self.buf.queue(Print(format!("R {:>3}", r)))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(1))?;

        self.buf.queue(Print(format!("G {:>3}", g)))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(1))?;

        self.buf.queue(Print(format!("B {:>3}", b)))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(2))?;

        self.buf.queue(Print(format!(
            "H {:>3.0}",
            color.hue.into_positive_degrees()
        )))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(1))?;

        self.buf
            .queue(Print(format!("S {:>3.0}", color.saturation * 100.0)))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(1))?;

        self.buf
            .queue(Print(format!("V {:>3.0}", color.value * 100.0)))?;
        self.buf.queue(MoveLeft(5))?;
        self.buf.queue(MoveDown(1))?;

        stdout().write_all(&self.buf)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn mouse_click(&mut self, x: u16, y: u16) -> Result<(), ()> {
        if x >= 7 || y < INPUTS_CB_HEIGHT || y >= Focus::V.value() + 1 {
            // 7 is the length of "#RRGGBB"
            let _ = self.lose_focus();
            return Err(());
        }
        self.focus = match y {
            y if y == Focus::Hex.value() => Focus::Hex,
            y if y == Focus::R.value() => Focus::R,
            y if y == Focus::G.value() => Focus::G,
            y if y == Focus::B.value() => Focus::B,
            y if y == Focus::H.value() => Focus::H,
            y if y == Focus::S.value() => Focus::S,
            y if y == Focus::V.value() => Focus::V,
            _ => {
                return Err(());
            }
        };
        Ok(())
    }

    pub fn value_input(&mut self, input: KeyCode) -> Option<(Focus, u32)> {
        if self.focus == Focus::NONE {
            return None;
        }
        if input.is_enter() {
            let focus = self.focus;
            self.modified = false;
            let _ = self.lose_focus();
            let value = match focus {
                Focus::Hex => u32::from_str_radix(&self.input_str, 16).unwrap_or(0),
                _ => self.input_str.parse::<u32>().unwrap_or(0),
            };
            return Some((focus, value));
        }

        if input.is_esc() {
            self.focus = Focus::NONE;
            return None;
        }

        if input.is_backspace() {
            self.modified = true;
            if self.input_str.len() > 0 {
                self.input_str.pop();
            }
        }

        if self.focus == Focus::Hex {
            let c = input.as_char();
            c.map(|c| {
                if !self.modified {
                    self.input_str.clear();
                    self.modified = true;
                }
                if self.input_str.len() < self.focus.input_max_len() && c.is_digit(16) {
                    self.input_str.push(c.to_ascii_lowercase());
                }
            });
        } else {
            let c = input.as_char();
            c.map(|c| {
                if !self.modified {
                    self.input_str.clear();
                    self.modified = true;
                }
                if self.input_str.len() < self.focus.input_max_len() && c.is_digit(10) {
                    self.input_str.push(c);
                }
            });
        }

        let _ = execute!(
            stdout(),
            MoveTo(
                self.pos.x as u16 + self.focus.prefix().len() as u16,
                self.pos.y as u16 + self.focus.value()
            ),
            Show,
            Print(format!("{}", " ".repeat(7))), //Equivalent of clearing
            MoveLeft(7),
            Print(&self.input_str),
        );
        let _ = stdout().flush();
        return None;
    }

    pub fn gain_focus(&mut self, color: &Hsv) -> io::Result<()> {
        let (r, g, b) = rgb_from_hsv(&color);
        match self.focus {
            Focus::Hex => {
                self.input_str = format!("{:02x}{:02x}{:02x}", r, g, b);
            }
            Focus::R => {
                self.input_str = format!("{:>3}", r);
            }
            Focus::G => {
                self.input_str = format!("{:>3}", g);
            }
            Focus::B => {
                self.input_str = format!("{:>3}", b);
            }
            Focus::H => {
                self.input_str = format!("{:>3.0}", color.hue.into_positive_degrees());
            }
            Focus::S => {
                self.input_str = format!("{:>3.0}", color.saturation * 100.0);
            }
            Focus::V => {
                self.input_str = format!("{:>3.0}", color.value * 100.0);
            }
            Focus::NONE => self.input_str.clear(),
        };
        self.modified = false;
        execute!(
            stdout(),
            MoveTo(
                self.pos.x as u16 + self.focus.prefix().len() as u16,
                self.pos.y as u16 + self.focus.value()
            ),
            Show,
            Print(format!("{}", " ".repeat(7))), //Equivalent of clearing
            MoveLeft(7),
            Print(&self.input_str),
            MoveLeft(self.focus.input_max_len() as u16 - self.input_str.len() as u16),
        )?;
        stdout().flush()?;
        Ok(())
    }

    // Returns true if value was applied, false if lost focus without applying
    pub fn lose_focus(&mut self) -> bool {
        self.focus = Focus::NONE;
        let _ = execute!(stdout(), Hide);
        let _ = stdout().flush();
        return !self.modified;
    }
}
