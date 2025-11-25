use crate::constants::*;
use crate::types::Vec2;
use crossterm::event::MouseEvent;
use palette::{FromColor, Hsv, Srgb};

pub fn rgb_from_hsv(hsv: &Hsv) -> (u8, u8, u8) {
    Srgb::from_color(*hsv).into_format::<u8>().into_components()
}

pub fn hsv_from_rgb(r: u8, g: u8, b: u8) -> Hsv {
    Hsv::from_color(Srgb::new(r, g, b).into_format::<f32>())
}

pub fn fade_color(mut color: Hsv) -> Hsv {
    color.value -= FADE_VALUE_FACTOR;
    color
}

pub fn normalize_pos(event: MouseEvent, pos: &Vec2) -> Option<Vec2> {
    let x = event.column as i32 - pos.x as i32;
    let y = event.row as i32 - pos.y as i32;
    if x < 0 || y < 0 {
        return None;
    }
    Some(Vec2 {
        x: x as u32,
        y: y as u32,
    })
}

pub fn check_terminal_size(width: u16, height: u16) -> bool {
    width < (TOTAL_WIDTH + 2) as u16 || height < (TOTAL_HEIGHT + 2) as u16
}
