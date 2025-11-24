use std::io::{self, Write, stdout};
use crate::types::*;

use crossterm::{
    cursor::{Hide, Show},
    event::*,
    execute,
    style::{Color, SetBackgroundColor},
    terminal::*,
};
use crate::constants::*;
use crate::ui::*;

pub struct State {
    pub sv_picker: sv_picker::SVPicker,
    pub hue_picker: hue_picker::HuePicker,
    pub inputs: inputs::Inputs,
    pub offset: Vec2,
    pub term_too_small: bool,
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
        term_too_small: bool,
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
            term_too_small,
            offset: Vec2::zero(),
        })
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.sv_picker.draw()?;
        self.hue_picker.draw()?;
        value_display::draw_value_display(
            &(VALUE_DISPLAY_REL_POS + self.offset),
            &self.sv_picker.selected_color,
        )?;
        self.inputs.draw(&self.sv_picker.selected_color)?;
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

    pub fn draw_component(&mut self, component: Component) -> io::Result<()> {
        match component {
            Component::SVPicker => self.sv_picker.draw(),
            Component::HuePicker => self.hue_picker.draw(),
            Component::Inputs => self.inputs.draw(&self.sv_picker.selected_color),
            Component::ValueDisplay => value_display::draw_value_display(
                &(VALUE_DISPLAY_REL_POS + self.offset),
                &self.sv_picker.selected_color,
            ),
        }
    }
}
