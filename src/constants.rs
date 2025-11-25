use crate::types::Vec2;

// This file contains all compile-time constants used in the app

pub const LOWER_HALF_BLOCK: char = '\u{2584}';
pub const FULL_CELL_BLOCK: char = '\u{2588}';
pub const SPACE: &str = "   ";

pub struct CustomRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
pub const BACKGROUND_COLOR: CustomRgb = CustomRgb {
    r: 0x1d,
    g: 0x1d,
    b: 0x1d,
};

pub const TEXT_COLOR: CustomRgb = CustomRgb {
    r: 0xff,
    g: 0xff,
    b: 0xff,
};

pub const FADE_VALUE_FACTOR: f32 = 0.4;

pub const FADED_TEXT_COLOR: CustomRgb = CustomRgb {
    r: 0x80,
    g: 0x80,
    b: 0x80,
};

pub const VALUE_DISPLAY_REL_POS: Vec2 = Vec2 { x: 0, y: 0 };

pub const SV_PICKER_REL_POS: Vec2 = Vec2 { x: 0, y: 2 };
pub const SV_PICKER_HEIGHT: u32 = 30;
pub const SV_PICKER_WIDTH: u32 = 60;

pub const HUE_PICKER_REL_POS: Vec2 = Vec2 {
    x: SV_PICKER_WIDTH + 3,
    y: 2,
};
pub const HUE_PICKER_HEIGHT: u32 = SV_PICKER_HEIGHT;
pub const HUE_PICKER_WIDTH: u32 = 5;

pub const INPUTS_REL_POS: Vec2 = Vec2 {
    x: HUE_PICKER_REL_POS.x + HUE_PICKER_WIDTH + 3,
    y: 2,
};
pub const INPUTS_CB_HEIGHT: u32 = 4;
pub const INPUTS_CB_WIDTH: u16 = 16;

pub const TOTAL_WIDTH: u32 = INPUTS_REL_POS.x + INPUTS_CB_WIDTH as u32;
pub const TOTAL_HEIGHT: u32 = SV_PICKER_REL_POS.y + SV_PICKER_HEIGHT;
