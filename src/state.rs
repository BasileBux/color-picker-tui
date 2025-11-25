use crate::types::*;
use crate::utils::*;
use std::io::{self, Write, stdout};

use crate::clipboard::*;
use crate::constants::*;
use crate::ui::*;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::{
    cursor::{Hide, Show},
    event::*,
    execute,
    style::{Color, SetBackgroundColor},
    terminal::*,
};
use palette::RgbHue;
use palette::SetHue;

pub struct State {
    pub sv_picker: sv_picker::SVPicker,
    pub hue_picker: hue_picker::HuePicker,
    pub inputs: inputs::Inputs,
    pub offset: Vec2,
    pub term_too_small: bool,
    pub exit_signal: bool,
}

pub enum Component {
    SVPicker,
    HuePicker,
    Inputs,
    ValueDisplay,
}

impl Drop for State {
    /// Cleans up the terminal state when the application exits.
    fn drop(&mut self) {
        let _ = execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen, Show);
        let _ = stdout().flush();
        let _ = disable_raw_mode();
    }
}

impl State {
    /// Only use this method to create a new State instance.
    pub fn new(
        sv_picker: sv_picker::SVPicker,
        hue_picker: hue_picker::HuePicker,
        inputs: inputs::Inputs,
        terminal_width: u16,
        terminal_height: u16,
    ) -> io::Result<Self> {
        execute!(
            stdout(),
            Hide,
            EnterAlternateScreen,
            EnableMouseCapture,
            SetBackgroundColor(Color::Rgb {
                r: BACKGROUND_COLOR.r,
                g: BACKGROUND_COLOR.g,
                b: BACKGROUND_COLOR.b
            }),
            Clear(ClearType::All)
        )?;
        stdout().flush()?;
        enable_raw_mode()?;
        Ok(Self {
            sv_picker,
            hue_picker,
            inputs,
            term_too_small: check_terminal_size(terminal_width, terminal_height),
            exit_signal: false,
            offset: Vec2::zero(),
        })
    }

    pub fn draw(&mut self, fade: bool) -> io::Result<()> {
        self.sv_picker.draw(fade)?;
        self.hue_picker.draw(fade)?;
        value_display::draw_value_display(
            &(VALUE_DISPLAY_REL_POS + self.offset),
            &self.sv_picker.selected_color,
            fade,
        )?;
        self.inputs.draw(&self.sv_picker.selected_color, fade)?;
        Ok(())
    }

    pub fn update_offset(&mut self, term_width: u16, term_height: u16) {
        let offset_x = (term_width as i16 - TOTAL_WIDTH as i16) / 2;
        let offset_y = (term_height as i16 - TOTAL_HEIGHT as i16) / 2;
        self.offset = Vec2 {
            x: offset_x.max(0) as u32,
            y: offset_y.max(0) as u32,
        };
    }

    pub fn offset_all(&mut self) {
        self.inputs.pos = INPUTS_REL_POS + self.offset;
        self.sv_picker.pos = SV_PICKER_REL_POS + self.offset;
        self.hue_picker.pos = HUE_PICKER_REL_POS + self.offset;
    }

    pub fn draw_component(&mut self, component: Component, fade: bool) -> io::Result<()> {
        match component {
            Component::SVPicker => self.sv_picker.draw(fade),
            Component::HuePicker => self.hue_picker.draw(fade),
            Component::Inputs => self.inputs.draw(&self.sv_picker.selected_color, fade),
            Component::ValueDisplay => value_display::draw_value_display(
                &(VALUE_DISPLAY_REL_POS + self.offset),
                &self.sv_picker.selected_color,
                fade,
            ),
        }
    }

    pub fn handle_resize(&mut self, x: u16, y: u16) -> io::Result<()> {
        if check_terminal_size(x, y) {
            self.term_too_small = true;
            let warning_text = "Terminal too small!";
            execute!(
                stdout(),
                SetBackgroundColor(Color::Rgb {
                    r: BACKGROUND_COLOR.r,
                    g: BACKGROUND_COLOR.g,
                    b: BACKGROUND_COLOR.b
                }),
                Clear(ClearType::All),
                MoveTo((x / 2) - (warning_text.len() as u16 / 2), y / 2),
                Print(warning_text),
            )?;
            return Ok(());
        }
        self.term_too_small = false;
        self.update_offset(x, y);
        self.offset_all();
        execute!(
            stdout(),
            SetBackgroundColor(Color::Rgb {
                r: BACKGROUND_COLOR.r,
                g: BACKGROUND_COLOR.g,
                b: BACKGROUND_COLOR.b
            }),
            Clear(ClearType::All)
        )?;
        self.draw(false)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn handle_mouse_event(&mut self, event: MouseEvent) -> io::Result<()> {
        if event.kind == MouseEventKind::Down(MouseButton::Left)
            || event.kind == MouseEventKind::Drag(MouseButton::Left)
        {
            if let Some(pos) = normalize_pos(event, &self.sv_picker.pos)
                && let Ok(()) = self.sv_picker.change_color(pos.x, pos.y)
            {
                self.draw_component(Component::ValueDisplay, false)?;
                self.draw_component(Component::Inputs, false)?;
            }

            if let Some(pos) = normalize_pos(event, &self.hue_picker.pos)
                && let Ok(hue) = self.hue_picker.get(pos.x, pos.y)
            {
                self.sv_picker.set_hue(hue);
                self.draw_component(Component::ValueDisplay, false)?;
                self.draw_component(Component::Inputs, false)?;
                self.draw_component(Component::SVPicker, false)?;
            }

            if let Some(pos) = normalize_pos(event, &self.inputs.pos)
                && let Ok(()) = self.inputs.mouse_click(pos.x, pos.y)
            {
                self.inputs.gain_focus(&self.sv_picker.selected_color)?;
            } else {
                let _ = self.inputs.lose_focus();
                self.draw_component(Component::Inputs, false)?;
            }
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> io::Result<()> {
        if event.code == KeyCode::Char('q')
            || (event.code == KeyCode::Char('c') && event.modifiers.contains(KeyModifiers::CONTROL))
            || event.code == KeyCode::Esc
        {
            self.exit_signal = true;
            return Ok(());
        }
        if event.code == KeyCode::Char('y') {
            let (r, g, b) = rgb_from_hsv(&self.sv_picker.selected_color);
            // TODO: Allow copying in different formats
            clipboard_copy(&format!("#{:02X}{:02X}{:02X}", r, g, b))?;
            return Ok(());
        }
        if event.code == KeyCode::Char('p') {
            // TODO: Allow pasting from different formats
            let clipboard_content = clipboard_paste()?;
            let clipboard_content = clipboard_content.trim().trim_start_matches('#');
            let value = u32::from_str_radix(&clipboard_content, 16).unwrap_or(0);
            let r = ((value >> 16) & 0xFF) as u8;
            let g = ((value >> 8) & 0xFF) as u8;
            let b = (value & 0xFF) as u8;
            self.sv_picker.selected_color = hsv_from_rgb(r, g, b);
            self.draw(false)?;
        }

        match self.inputs.value_input(event.code) {
            Some((focus, value)) => {
                match focus {
                    inputs::Focus::Hex => {
                        let r = ((value >> 16) & 0xFF) as u8;
                        let g = ((value >> 8) & 0xFF) as u8;
                        let b = (value & 0xFF) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    inputs::Focus::R => {
                        let (_, g, b) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let r = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    inputs::Focus::G => {
                        let (r, _, b) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let g = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    inputs::Focus::B => {
                        let (r, g, _) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let b = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    inputs::Focus::H => {
                        self.sv_picker
                            .selected_color
                            .set_hue(RgbHue::from_degrees(value as f32));
                    }
                    inputs::Focus::S => {
                        self.sv_picker.selected_color.saturation = (value.min(100) as f32) / 100.0;
                    }
                    inputs::Focus::V => {
                        self.sv_picker.selected_color.value = (value.min(100) as f32) / 100.0;
                    }
                    _ => {}
                }
                self.draw(false)?;
            }
            None => {
                if self.inputs.focus == inputs::Focus::NONE {
                    let _ = self.inputs.lose_focus();
                    self.draw_component(Component::Inputs, false)?;
                }
            }
        }
        Ok(())
    }
}
