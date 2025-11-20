use palette::{FromColor, Hsv, Srgb};

pub fn rgb_from_hsv(hsv: &Hsv) -> (u8, u8, u8) {
    Srgb::from_color(*hsv).into_format::<u8>().into_components()
}

pub fn hsv_from_rgb(r: u8, g: u8, b: u8) -> Hsv {
    Hsv::from_color(Srgb::new(r, g, b).into_format::<f32>())
}
