use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::constants::*;
use crate::drawing::DrawingMode;
use crate::utils::file_operations::{save_image, generate_default_filename};

mod buttons;

pub fn display_ui(state: &mut AppState) {
    // let screen_width = screen_width();
    // let toolbar_width = screen_width.min(TOOLBAR_WIDTH);
    let screen_height = screen_height();
    draw_rectangle(0.0, 0.0, TOOLBAR_WIDTH, screen_height, UI_BG_COLOR);

    let bottom_margin = 10.0;
    let button_height = 30.0;
    let button_spacing = 10.0;
    // let button_width = (toolbar_width - 40.0) / 3.0;
    let palette_start_y = 50.0;

    let bottom_button_y = screen_height - bottom_margin - button_height;
    let clear_button_y = bottom_button_y - button_height - button_spacing;
    let save_button_y = clear_button_y - button_height - button_spacing;

    let mode_button_width = (TOOLBAR_WIDTH - 30.0) / 2.0;
    let mode_button_y = palette_start_y + 220.0;

    if buttons::create_button(state, "Draw", 10.0, mode_button_y, mode_button_width, button_height, 
    matches!(state.drawing_mode, DrawingMode::Draw), "Switch to Draw Mode") {
        state.drawing_mode = DrawingMode::Draw;
    }
    if buttons::create_button(state, "Erase", 20.0 + mode_button_width, mode_button_y, mode_button_width, button_height, 
    matches!(state.drawing_mode, DrawingMode::Erase), "Switch to Erase Mode") {
        state.drawing_mode = DrawingMode::Erase;
    }

    for (i, color) in state.palette.iter().enumerate() {
        let x = 10.0 + (i as f32 % 4.0 * 55.0);
        let y = palette_start_y + (i as f32 / 4.0).floor() * 55.0;
        if buttons::create_color_button(x, y, 50.0, 50.0, *color, state.active_color == *color) {
            state.active_color = *color;
        }
    }

    let slider_y = palette_start_y + 120.0 + 30.0;
    let slider_width = TOOLBAR_WIDTH - 20.0;
    state.brush_size = buttons::create_slider(10.0, slider_y + 15.0, slider_width, state.brush_size, state.min_brush_size, state.max_brush_size);

    let size_text = format!("Brush Size: {:.1}", state.brush_size);
    draw_text(&size_text, 10.0, slider_y - 10.0, 20.0, TEXT_COLOR);

    if buttons::create_button(state, "Save Image", 10.0, save_button_y, TOOLBAR_WIDTH - 20.0, button_height, false, "Save Image (Ctrl+S)") {
        let default_name = generate_default_filename();
        save_image(state, &default_name);
    }

    if buttons::create_button(state, "Reset Canvas", 10.0, clear_button_y, TOOLBAR_WIDTH - 20.0, button_height, false, "Clear Canvas (Ctrl+N)") {
        state.clear_canvas();
    }

    if buttons::create_button(state, "Undo", 10.0, bottom_button_y, (TOOLBAR_WIDTH - 30.0) / 2.0, button_height, false, "Undo (Ctrl+Z)") {
        state.undo();
    }
    if buttons::create_button(state, "Redo", TOOLBAR_WIDTH / 2.0 + 5.0, bottom_button_y, (TOOLBAR_WIDTH - 30.0) / 2.0, button_height, false, "Redo (Ctrl+Y)") {
        state.redo();
    }

    display_tooltip(state);
}

fn display_tooltip(state: &AppState) {
    if state.tooltip.visible {
        let padding = 5.0;
        let font_size = 16.0;
        let text_size = measure_text(&state.tooltip.text, None, font_size as u16, 1.0);
        let rect_width = text_size.width + 2.0 * padding;
        let rect_height = text_size.height + 2.0 * padding;
        
        draw_rectangle(
            state.tooltip.position.x,
            state.tooltip.position.y,
            rect_width,
            rect_height,
            TOOLTIP_BG_COLOR,
        );
        draw_text(
            &state.tooltip.text,
            state.tooltip.position.x + padding,
            state.tooltip.position.y + padding + text_size.height,
            font_size,
            TOOLTIP_TEXT_COLOR,
        );
    }
}