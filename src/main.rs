use palette::{RgbHue, SetHue};
use std::io::{self, Write, stdout};
use tui_color_picker::types::*;

use crossterm::{
    cursor::{MoveTo},
    event::*,
    execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::*,
};
use std::time::Duration;
use tui_color_picker::clipboard::*;
use tui_color_picker::constants::*;
use tui_color_picker::utils::{hsv_from_rgb, rgb_from_hsv};
use tui_color_picker::ui::*;
use tui_color_picker::state::*;

fn check_terminal_size(width: u16, height: u16) -> bool {
    width < (TOTAL_WIDTH + 2) as u16 || height < (TOTAL_HEIGHT + 2) as u16
}

// enum PopupType {
//     ClipboardFormatSelection,
//     None,
// }

fn main() -> io::Result<()> {
    let (term_width, term_height) = crossterm::terminal::size()?;

    let mut app = State::new(
        sv_picker::SVPicker::new(SV_PICKER_REL_POS, SV_PICKER_WIDTH, SV_PICKER_HEIGHT),
        hue_picker::HuePicker::new(HUE_PICKER_REL_POS, HUE_PICKER_WIDTH, HUE_PICKER_HEIGHT),
        inputs::Inputs::new(INPUTS_REL_POS),
        check_terminal_size(term_width, term_height),
    )?;
    app.update_offset(term_width, term_height);
    app.offset_all();
    app.draw()?;

    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Mouse(event) if !app.term_too_small => {
                    if event.kind == MouseEventKind::Down(MouseButton::Left)
                        || event.kind == MouseEventKind::Drag(MouseButton::Left)
                    {
                        let x = event.column as i16 - app.sv_picker.pos.x as i16;
                        let y = event.row as i16 - app.sv_picker.pos.y as i16;
                        if let Ok(()) = app.sv_picker.set_selected_color(Vec2 {
                            x: x as u32,
                            y: y as u32,
                        }) && y >= 0
                            && x >= 0
                        {
                            app.draw_component(Component::ValueDisplay)?;
                            app.draw_component(Component::Inputs)?;
                        }

                        let x = event.column as i16 - app.hue_picker.pos.x as i16;
                        let y = event.row as i16 - app.hue_picker.pos.y as i16;

                        if let Ok(hue) = app.hue_picker.get(x as u32, y as u32)
                            && x >= 0
                            && y >= 0
                        {
                            app.sv_picker.set_hue(hue);
                            app.draw_component(Component::ValueDisplay)?;
                            app.draw_component(Component::Inputs)?;
                            app.draw_component(Component::SVPicker)?;
                        }

                        let x = event.column as i16 - app.inputs.pos.x as i16;
                        let y = event.row as i16 - app.inputs.pos.y as i16;
                        if let Ok(()) = app.inputs.mouse_click(x as u16, y as u16)
                            && x >= 0
                            && y >= 0
                        {
                            app.inputs.gain_focus(&app.sv_picker.selected_color)?;
                        } else {
                            let _ = app.inputs.lose_focus();
                            app.draw_component(Component::Inputs)?;
                        }
                    }
                }
                Event::Key(event) if !app.term_too_small => {
                    if event.code == KeyCode::Char('q')
                        || event.code == KeyCode::Char('c')
                        || event.code == KeyCode::Esc
                    {
                        break;
                    }
                    if event.code == KeyCode::Char('y') {
                        let (r, g, b) = rgb_from_hsv(&app.sv_picker.selected_color);
                        // TODO: Allow copying in different formats
                        clipboard_copy(&format!("#{:02X}{:02X}{:02X}", r, g, b))?;
                        continue;
                    }
                    if event.code == KeyCode::Char('p') {
                        // TODO: Allow pasting from different formats
                        let clipboard_content = clipboard_paste()?;
                        let clipboard_content = clipboard_content.trim().trim_start_matches('#');
                        let value = u32::from_str_radix(&clipboard_content, 16).unwrap_or(0);
                        let r = ((value >> 16) & 0xFF) as u8;
                        let g = ((value >> 8) & 0xFF) as u8;
                        let b = (value & 0xFF) as u8;
                        app.sv_picker.selected_color = hsv_from_rgb(r, g, b);
                        app.draw()?;
                    }

                    match app.inputs.value_input(event.code) {
                        Some((focus, value)) => {
                            match focus {
                                inputs::Focus::Hex => {
                                    let r = ((value >> 16) & 0xFF) as u8;
                                    let g = ((value >> 8) & 0xFF) as u8;
                                    let b = (value & 0xFF) as u8;
                                    app.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::R => {
                                    let (_, g, b) = rgb_from_hsv(&app.sv_picker.selected_color);
                                    let r = value.min(255) as u8;
                                    app.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::G => {
                                    let (r, _, b) = rgb_from_hsv(&app.sv_picker.selected_color);
                                    let g = value.min(255) as u8;
                                    app.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::B => {
                                    let (r, g, _) = rgb_from_hsv(&app.sv_picker.selected_color);
                                    let b = value.min(255) as u8;
                                    app.sv_picker.selected_color = hsv_from_rgb(r, g, b)
                                }
                                inputs::Focus::H => {
                                    app.sv_picker
                                        .selected_color
                                        .set_hue(RgbHue::from_degrees(value as f32));
                                }
                                inputs::Focus::S => {
                                    app.sv_picker.selected_color.saturation =
                                        (value.min(100) as f32) / 100.0;
                                }
                                inputs::Focus::V => {
                                    app.sv_picker.selected_color.value =
                                        (value.min(100) as f32) / 100.0;
                                }
                                _ => {}
                            }
                            app.draw()?;
                        }
                        None => {
                            if app.inputs.focus == inputs::Focus::NONE {
                                let _ = app.inputs.lose_focus();
                                app.draw_component(Component::Inputs)?;
                            }
                        }
                    }
                }
                Event::Resize(x, y) => {
                    if check_terminal_size(x, y) {
                        app.term_too_small = true;
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
                    app.term_too_small = false;
                    app.update_offset(x, y);
                    app.offset_all();
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
                }
                _ => {}
            }
        }
    }
    Ok(())
}
