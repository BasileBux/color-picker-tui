use std::io::{self};

use crossterm::event::*;
use std::time::Duration;
use tui_color_picker::clipboard::ui::clear_clipboard_format_selector;
use tui_color_picker::constants::*;
use tui_color_picker::state::*;
use tui_color_picker::ui::hue_picker::HuePicker;
use tui_color_picker::ui::inputs::Inputs;
use tui_color_picker::ui::saturation_value_picker::SaturationValuePicker;

fn main() -> io::Result<()> {
    let (term_width, term_height) = crossterm::terminal::size()?;

    let mut app = State::new(
        SaturationValuePicker::new(SV_PICKER_REL_POS, SV_PICKER_WIDTH, SV_PICKER_HEIGHT),
        HuePicker::new(HUE_PICKER_REL_POS, HUE_PICKER_WIDTH, HUE_PICKER_HEIGHT),
        Inputs::new(INPUTS_REL_POS),
        term_width,
        term_height,
    )?;
    app.update_offset(term_width, term_height);
    app.offset_all();
    app.draw(false)?;

    loop {
        if poll(Duration::from_millis(100))? {
            // clear confirmation message after next event
            if app.flags & COPY_CONFIRMED_FLAG != 0 || app.flags & PASTE_CONFIRMED_FLAG != 0 {
                clear_clipboard_format_selector(COPY_FORMAT_SELECTOR_RES_POS + app.offset)?;
                app.flags &= !COPY_CONFIRMED_FLAG;
                app.flags &= !PASTE_CONFIRMED_FLAG;
            }

            let event = read()?;
            match event {
                Event::Mouse(event) if !app.term_too_small => {
                    app.handle_mouse_event(event)?;
                }
                Event::Key(event) if !app.term_too_small => {
                    app.handle_key_event(event)?;
                    if app.flags & EXIT_FLAG != 0 {
                        break;
                    }
                }
                Event::Resize(x, y) => {
                    app.handle_resize(x, y)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
