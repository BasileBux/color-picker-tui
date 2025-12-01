use crate::clipboard::{paste::*, ui::*};
use crate::types::*;
use crate::ui::hue_picker::HuePicker;
use crate::ui::inputs::{Focus, Inputs};
use crate::ui::saturation_value_picker::SaturationValuePicker;
use crate::ui::value_display::draw_value_display;
use crate::utils::*;
use std::io::{self, Write, stdout};

use crate::constants::*;
use crate::crossterm_commands::*;
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
    pub sv_picker: SaturationValuePicker,
    pub hue_picker: HuePicker,
    pub inputs: Inputs,
    pub offset: Vec2,
    pub term_too_small: bool,
    pub flags: u8,
}

pub enum Component {
    SVPicker,
    HuePicker,
    Inputs,
    ValueDisplay,
}

pub const EXIT_FLAG: u8 = 1 << 0;
pub const COPY_FLAG: u8 = 1 << 1;
pub const COPY_CONFIRMED_FLAG: u8 = 1 << 2;
pub const PASTE_CONFIRMED_FLAG: u8 = 1 << 3;

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
        sv_picker: SaturationValuePicker,
        hue_picker: HuePicker,
        inputs: Inputs,
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
            flags: 0,
            offset: Vec2::zero(),
        })
    }

    pub fn draw(&mut self, fade: bool) -> io::Result<()> {
        self.sv_picker.draw(fade)?;
        self.hue_picker.draw(fade)?;
        draw_value_display(
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
            Component::ValueDisplay => draw_value_display(
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
                ResetDefaultColors(false),
                Clear(ClearType::All),
                MoveTo((x / 2) - (warning_text.len() as u16 / 2), y / 2),
                Print(warning_text),
            )?;
            return Ok(());
        }
        self.term_too_small = false;
        self.update_offset(x, y);
        self.offset_all();
        execute!(stdout(), ResetDefaultColors(false), Clear(ClearType::All))?;
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
                && event.kind != MouseEventKind::Drag(MouseButton::Left)
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
        if self.flags & COPY_FLAG != 0 {
            handle_copy_input_format_selection_input(event, self.sv_picker.selected_color)?;
            self.flags &= !COPY_FLAG;
            clear_clipboard_format_selector(COPY_FORMAT_SELECTOR_RES_POS + self.offset)?;
            draw_copied_confirmation(COPY_FORMAT_SELECTOR_RES_POS + self.offset, false)?;
            self.flags |= COPY_CONFIRMED_FLAG;
            return Ok(());
        }

        if event.code == KeyCode::Char('q')
            || (event.code == KeyCode::Char('c') && event.modifiers.contains(KeyModifiers::CONTROL))
            || event.code == KeyCode::Esc
        {
            self.flags |= EXIT_FLAG;
            return Ok(());
        }
        if event.code == KeyCode::Char('y') {
            draw_clipboard_format_selector(
                COPY_FORMAT_SELECTOR_RES_POS + self.offset,
                self.sv_picker.selected_color,
                false,
            )?;
            self.flags |= COPY_FLAG;
            return Ok(());
        }
        if event.code == KeyCode::Char('p') {
            if let Some(clipboard_content) = clipboard_paste() {
                self.sv_picker.selected_color = clipboard_content;
                self.draw(false)?;
                draw_pasted_confirmation(COPY_FORMAT_SELECTOR_RES_POS + self.offset, false)?;
                self.flags |= PASTE_CONFIRMED_FLAG;
            }
        }

        match self.inputs.value_input(event.code) {
            Some((focus, value)) => {
                match focus {
                    Focus::Hex => {
                        let r = ((value >> 16) & 0xFF) as u8;
                        let g = ((value >> 8) & 0xFF) as u8;
                        let b = (value & 0xFF) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    Focus::R => {
                        let (_, g, b) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let r = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    Focus::G => {
                        let (r, _, b) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let g = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    Focus::B => {
                        let (r, g, _) = rgb_from_hsv(&self.sv_picker.selected_color);
                        let b = value.min(255) as u8;
                        self.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                    }
                    Focus::H => {
                        self.sv_picker
                            .selected_color
                            .set_hue(RgbHue::from_degrees(value as f32));
                    }
                    Focus::S => {
                        self.sv_picker.selected_color.saturation = (value.min(100) as f32) / 100.0;
                    }
                    Focus::V => {
                        self.sv_picker.selected_color.value = (value.min(100) as f32) / 100.0;
                    }
                    _ => {}
                }
                self.draw(false)?;
            }
            None => {
                if self.inputs.focus == Focus::NONE {
                    let _ = self.inputs.lose_focus();
                    self.draw_component(Component::Inputs, false)?;
                }
            }
        }
        Ok(())
    }
}
