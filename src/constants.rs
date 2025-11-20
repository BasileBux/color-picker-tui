use crate::types::Vec2;

pub const LOWER_HALF_BLOCK: char = '\u{2584}';
pub const FULL_CELL_BLOCK: char = '\u{2588}';
pub const SPACE: &str = "   ";

pub struct CustomRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
pub const BACKGROUND_COLOR: CustomRgb = CustomRgb { r: 0x1d, g: 0x1d, b: 0x1d };

pub const VALUE_DISPLAY_REL_POS: Vec2 = Vec2 { x: 0, y: 0 };

pub const SV_PICKER_REL_POS: Vec2 = Vec2 { x: 0, y: 2 };
pub const SV_PICKER_HEIGHT: u32 = 25;
pub const SV_PICKER_WIDTH: u32 = 50;

pub const HUE_PICKER_REL_POS: Vec2 = Vec2 { x: SV_PICKER_WIDTH + 3, y: 2 };
pub const HUE_PICKER_HEIGHT: u32 = SV_PICKER_HEIGHT;
pub const HUE_PICKER_WIDTH: u32 = 5;

pub const INPUTS_REL_POS: Vec2 = Vec2 { x: HUE_PICKER_REL_POS.x + HUE_PICKER_WIDTH + 3, y: 2 };
pub const INPUTS_CB_HEIGHT: u16 = 4;
pub const INPUTS_CB_WIDTH: u16 = 16;

pub const TOTAL_WIDTH: u32 = INPUTS_REL_POS.x + INPUTS_CB_WIDTH as u32;
pub const TOTAL_HEIGHT: u32 = SV_PICKER_REL_POS.y + SV_PICKER_HEIGHT;
