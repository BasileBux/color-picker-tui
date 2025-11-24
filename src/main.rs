use palette::{RgbHue, SetHue};
use std::io::{self, Write, stdout};
use tui_color_picker::types::*;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::*,
    execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::*,
};
use std::time::Duration;
use tui_color_picker::clipboard::*;
use tui_color_picker::constants::*;
use tui_color_picker::utils::{hsv_from_rgb, rgb_from_hsv};
mod hue_picker;
mod inputs;
mod sv_picker;
mod value_display;

fn setup() -> io::Result<()> {
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
    Ok(())
}

struct Cleanup;
impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen, Show);
        let _ = stdout().flush();
        let _ = disable_raw_mode();
    }
}

fn compute_offset(term_width: u16, term_height: u16) -> Vec2 {
    let offset_x = (term_width as i16 - TOTAL_WIDTH as i16) / 2;
    let offset_y = (term_height as i16 - TOTAL_HEIGHT as i16) / 2;
    Vec2 {
        x: offset_x.max(0) as u32,
        y: offset_y.max(0) as u32,
    }
}

fn offset_all(
    sv_picker: &mut sv_picker::SVPicker,
    hue_picker: &mut hue_picker::HuePicker,
    inputs: &mut inputs::Inputs,
    offset: &Vec2,
) {
    inputs.pos = INPUTS_REL_POS + *offset;
    sv_picker.pos = SV_PICKER_REL_POS + *offset;
    hue_picker.pos = HUE_PICKER_REL_POS + *offset;
}

fn draw_all(
    sv_picker: &mut sv_picker::SVPicker,
    hue_picker: &mut hue_picker::HuePicker,
    inputs: &mut inputs::Inputs,
    offset: &Vec2,
) -> io::Result<()> {
    sv_picker.draw()?;
    hue_picker.draw()?;
    value_display::draw_value_display(
        &(VALUE_DISPLAY_REL_POS + *offset),
        &sv_picker.selected_color,
    )?;
    inputs.draw(&sv_picker.selected_color)?;
    Ok(())
}

fn check_terminal_size(width: u16, height: u16) -> bool {
    width < (TOTAL_WIDTH + 2) as u16 || height < (TOTAL_HEIGHT + 2) as u16
}

fn main() -> io::Result<()> {
    setup()?;
    let _clean = Cleanup;

    let (term_width, term_height) = crossterm::terminal::size()?;
    let mut term_too_small = check_terminal_size(term_width, term_height);
    let mut offset = compute_offset(term_width, term_height);

    let mut sv_picker = sv_picker::SVPicker::new(
        SV_PICKER_REL_POS + offset,
        SV_PICKER_WIDTH,
        SV_PICKER_HEIGHT,
    );
    let mut hue_picker = hue_picker::HuePicker::new(
        HUE_PICKER_REL_POS + offset,
        HUE_PICKER_WIDTH,
        HUE_PICKER_HEIGHT,
    );
    let mut inputs = inputs::Inputs::new(INPUTS_REL_POS + offset);

    draw_all(&mut sv_picker, &mut hue_picker, &mut inputs, &offset)?;

    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Mouse(event) if !term_too_small => {
                    if event.kind == MouseEventKind::Down(MouseButton::Left)
                        || event.kind == MouseEventKind::Drag(MouseButton::Left)
                    {
                        let x = event.column as i16 - sv_picker.pos.x as i16;
                        let y = event.row as i16 - sv_picker.pos.y as i16;
                        if let Ok(()) = sv_picker.set_selected_color(Vec2 {
                            x: x as u32,
                            y: y as u32,
                        }) && y >= 0
                            && x >= 0
                        {
                            value_display::draw_value_display(
                                &(VALUE_DISPLAY_REL_POS + offset),
                                &sv_picker.selected_color,
                            )?;
                            inputs.draw(&sv_picker.selected_color)?;
                        }

                        let x = event.column as i16 - hue_picker.pos.x as i16;
                        let y = event.row as i16 - hue_picker.pos.y as i16;

                        if let Ok(hue) = hue_picker.get(x as u32, y as u32)
                            && x >= 0
                            && y >= 0
                        {
                            sv_picker.set_hue(hue);
                            value_display::draw_value_display(
                                &(VALUE_DISPLAY_REL_POS + offset),
                                &sv_picker.selected_color,
                            )?;
                            inputs.draw(&sv_picker.selected_color)?;
                            sv_picker.draw()?;
                        }

                        let x = event.column as i16 - inputs.pos.x as i16;
                        let y = event.row as i16 - inputs.pos.y as i16;
                        if let Ok(()) = inputs.mouse_click(x as u16, y as u16)
                            && x >= 0
                            && y >= 0
                        {
                            inputs.gain_focus(&sv_picker.selected_color)?;
                        } else {
                            let _ = inputs.lose_focus();
                            inputs.draw(&sv_picker.selected_color)?;
                        }
                    }
                }
                Event::Key(event) if !term_too_small => {
                    if event.code == KeyCode::Char('q')
                        || event.code == KeyCode::Char('c')
                        || event.code == KeyCode::Esc
                    {
                        break;
                    }
                    if event.code == KeyCode::Char('y') {
                        let (r, g, b) = rgb_from_hsv(&sv_picker.selected_color);
                        // TODO: Use abstraction for wayland / X11, Windows, osx clipboard handling
                        // TODO: Allow copying in different formats
                        wl_copy(&format!("#{:02X}{:02X}{:02X}", r, g, b))?;
                        continue;
                    }
                    if event.code == KeyCode::Char('p') {
                        // TODO: Use abstraction for wayland / X11, Windows, osx clipboard handling
                        // TODO: Allow pasting from different formats
                        let clipboard_content = wl_paste()?;
                        let clipboard_content = clipboard_content.trim().trim_start_matches('#');
                        let value = u32::from_str_radix(&clipboard_content, 16).unwrap_or(0);
                        let r = ((value >> 16) & 0xFF) as u8;
                        let g = ((value >> 8) & 0xFF) as u8;
                        let b = (value & 0xFF) as u8;
                        sv_picker.selected_color = hsv_from_rgb(r, g, b);
                        draw_all(&mut sv_picker, &mut hue_picker, &mut inputs, &offset)?;
                    }

                    match inputs.value_input(event.code) {
                        Some((focus, value)) => {
                            match focus {
                                inputs::Focus::Hex => {
                                    let r = ((value >> 16) & 0xFF) as u8;
                                    let g = ((value >> 8) & 0xFF) as u8;
                                    let b = (value & 0xFF) as u8;
                                    sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::R => {
                                    let (_, g, b) = rgb_from_hsv(&sv_picker.selected_color);
                                    let r = value.min(255) as u8;
                                    sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::G => {
                                    let (r, _, b) = rgb_from_hsv(&sv_picker.selected_color);
                                    let g = value.min(255) as u8;
                                    sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::B => {
                                    let (r, g, _) = rgb_from_hsv(&sv_picker.selected_color);
                                    let b = value.min(255) as u8;
                                    sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::H => {
                                    sv_picker
                                        .selected_color
                                        .set_hue(RgbHue::from_degrees(value as f32));
                                }
                                inputs::Focus::S => {
                                    sv_picker.selected_color.saturation =
                                        (value.min(100) as f32) / 100.0;
                                }
                                inputs::Focus::V => {
                                    sv_picker.selected_color.value =
                                        (value.min(100) as f32) / 100.0;
                                }
                                _ => {}
                            }
                            // hue_picker.draw()?;
                            // sv_picker.draw()?;
                            // value_display::draw_value_display(
                            //     &(VALUE_DISPLAY_REL_POS + offset),
                            //     &sv_picker.selected_color,
                            // )?;
                            // inputs.draw(&sv_picker.selected_color)?;
                            draw_all(&mut sv_picker, &mut hue_picker, &mut inputs, &offset)?;
                        }
                        None => {
                            if inputs.focus == inputs::Focus::NONE {
                                let _ = inputs.lose_focus();
                                inputs.draw(&sv_picker.selected_color)?;
                            }
                        }
                    }
                }
                Event::Resize(x, y) => {
                    if check_terminal_size(x, y) {
                        term_too_small = true;
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
                        continue;
                    }
                    term_too_small = false;
                    offset = compute_offset(x, y);
                    offset_all(&mut sv_picker, &mut hue_picker, &mut inputs, &offset);
                    execute!(
                        stdout(),
                        SetBackgroundColor(Color::Rgb {
                            r: BACKGROUND_COLOR.r,
                            g: BACKGROUND_COLOR.g,
                            b: BACKGROUND_COLOR.b
                        }),
                        Clear(ClearType::All)
                    )?;
                    stdout().flush()?;
                    draw_all(&mut sv_picker, &mut hue_picker, &mut inputs, &offset)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
