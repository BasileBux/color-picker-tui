use hsv::hsv_to_rgb;
use std::io::{self, Write, stdout};
use tui_color_picker::types::*;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::*,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::*,
};
use std::time::Duration;
mod hue_picker;
mod inputs;
mod sv_picker;

const SPACE: &str = "   ";

const SV_PICKER_HEIGHT: u32 = 25;
const SV_PICKER_WIDTH: u32 = 50;
const HUE_PICKER_HEIGHT: u32 = SV_PICKER_HEIGHT;
const HUE_PICKER_WIDTH: u32 = 5;

fn setup() -> io::Result<()> {
    let _ = execute!(stdout(), Hide).unwrap();
    let _ = execute!(stdout(), EnterAlternateScreen).unwrap();
    let _ = execute!(stdout(), EnableMouseCapture).unwrap();
    enable_raw_mode()?;
    Ok(())
}

struct Cleanup;
impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = execute!(stdout(), DisableMouseCapture);
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = execute!(stdout(), Show);
        let _ = disable_raw_mode();
    }
}

fn draw_value_display(pos: Vec2, color: &HSV) -> io::Result<()> {
    let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
    execute!(
        stdout(),
        MoveTo(pos.x as u16, pos.y as u16),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Rgb { r, g, b }),
        Print(format!("{}", FULL_CELL_BLOCK).repeat(8)),
        ResetColor,
        Print(format!(
            "{}HEX: #{:02x}{:02x}{:02x}{}RGB: {:>3}, {:>3}, {:>3}{}HSL: {:>3.0}, {:>3.2}%, {:>3.2}%",
            SPACE,
            r,
            g,
            b,
            SPACE,
            r,
            g,
            b,
            SPACE,
            color.h,
            color.s * 100.0,
            color.v * 100.0
        ))
    )?;
    Ok(())
}

fn main() -> io::Result<()> {
    setup()?;
    let _clean = Cleanup;

    let mut sv_picker =
        sv_picker::SVPicker::new(Vec2 { x: 0, y: 6 }, SV_PICKER_WIDTH, SV_PICKER_HEIGHT);
    let mut hue_picker =
        hue_picker::HuePicker::new(Vec2 { x: 53, y: 6 }, HUE_PICKER_WIDTH, HUE_PICKER_HEIGHT);
    let mut inputs = inputs::Inputs::new(Vec2 { x: 60, y: 6 });
    sv_picker.draw()?;
    hue_picker.draw()?;

    if let Ok(color) = sv_picker.get(sv_picker.selected_pos.x, sv_picker.selected_pos.y) {
        draw_value_display(Vec2 { x: 0, y: 4 }, &color)?;
        inputs.draw(&color)?;
    }

    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Mouse(event) => {
                    if event.kind == MouseEventKind::Down(MouseButton::Left)
                        || event.kind == MouseEventKind::Drag(MouseButton::Left)
                    {
                        let x = event.column as i16 - sv_picker.pos.x as i16;
                        let y = event.row as i16 - sv_picker.pos.y as i16;
                        if let Ok(color) = sv_picker.get(x as u32, y as u32)
                            && x >= 0
                            && y >= 0
                        {
                            sv_picker.selected_pos = Vec2 {
                                x: x as u32,
                                y: y as u32,
                            };
                            draw_value_display(Vec2 { x: 0, y: 4 }, &color)?;
                            inputs.draw(&color)?;
                        }

                        let x = event.column as i16 - hue_picker.pos.x as i16;
                        let y = event.row as i16 - hue_picker.pos.y as i16;
                        if let Ok(hue) = hue_picker.get(x as u32, y as u32)
                            && x >= 0
                            && y >= 0
                        {
                            sv_picker.hue = hue;
                            let color = sv_picker.get_current();
                            draw_value_display(Vec2 { x: 0, y: 4 }, &color)?;
                            inputs.draw(&color)?;
                            sv_picker.draw()?;
                        }

                        let x = event.column as i16 - inputs.pos.x as i16;
                        let y = event.row as i16 - inputs.pos.y as i16;
                        if x <= 0 || y <= 0 {
                            if !inputs.lose_focus() {
                                // BUG: This doesn't work and doesn't redraw the inputs
                                let color = sv_picker.get_current();
                                inputs.draw(&color)?;
                            }
                            continue;
                        }
                        if let Ok(()) = inputs.mouse_click(x as u16, y as u16) {
                            // Call input management function
                            if let Ok(color) =
                                sv_picker.get(sv_picker.selected_pos.x, sv_picker.selected_pos.y)
                            {
                                inputs.gain_focus(&color)?;
                            }
                        }
                    }
                }
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                    match inputs.value_input(event.code) {
                        Some((focus, value)) => {
                            let mut color = sv_picker.get_current();
                            match focus {
                                inputs::Focus::Hex => {
                                    let r = ((value >> 16) & 0xFF) as u8;
                                    let g = ((value >> 8) & 0xFF) as u8;
                                    let b = (value & 0xFF) as u8;
                                    // color = hsv::rgb_to_hsv(r, g, b);
                                }
                                inputs::Focus::R => {
                                    let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
                                    let new_r = value.min(255) as u8;
                                    // color = hsv::rgb_to_hsv(new_r, g, b);
                                }
                                inputs::Focus::G => {
                                    let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
                                    let new_g = value.min(255) as u8;
                                    // color = hsv::rgb_to_hsv(r, new_g, b);
                                }
                                inputs::Focus::B => {
                                    let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
                                    let new_b = value.min(255) as u8;
                                    // color = hsv::rgb_to_hsv(r, g, new_b);
                                }
                                inputs::Focus::H => {
                                    color.h = (value % 360) as f64;
                                }
                                inputs::Focus::S => {
                                    color.s = (value.min(100) as f64) / 100.0;
                                }
                                inputs::Focus::V => {
                                    color.v = (value.min(100) as f64) / 100.0;
                                }
                                _ => {}
                            }
                            sv_picker.hue = color.h;
                            hue_picker.draw()?;
                            sv_picker.draw()?;
                            draw_value_display(Vec2 { x: 0, y: 4 }, &color)?;
                            inputs.draw(&color)?;
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
