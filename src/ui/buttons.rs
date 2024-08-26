use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::constants::*;

pub fn create_button(state: &mut AppState, text: &str, x: f32, y: f32, width: f32, height: f32, is_active: bool, tooltip: &str) -> bool {
    let rect = Rect::new(x, y, width, height);
    let mouse_pos = Vec2::from(mouse_position());
    let is_hovered = rect.contains(mouse_pos);
    
    let color = if is_active {
        HIGHLIGHT_COLOR
    } else if is_hovered {
        Color::new(0.25, 0.25, 0.3, 0.8)
    } else {
        UI_BG_COLOR
    };
    
    draw_rounded_rectangle(x, y, width, height, 5.0, color);
    
    let text_size = measure_text(text, None, 20, 1.0);
    let text_x = x + (width - text_size.width) / 2.0;
    let text_y = y + (height + text_size.height) / 2.0;
    draw_text(text, text_x, text_y, 20.0, TEXT_COLOR);
    
    if is_hovered {
        state.show_tooltip(tooltip, Vec2::new(mouse_pos.x + 10.0, mouse_pos.y + 10.0));
    }
    
    is_mouse_button_pressed(MouseButton::Left) && is_hovered
}

pub fn create_color_button(x: f32, y: f32, width: f32, height: f32, color: Color, is_selected: bool) -> bool {
    let rect = Rect::new(x, y, width, height);
    
    draw_rounded_rectangle(x, y, width, height, 5.0, color);
    
    if is_selected {
        draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 2.0, HIGHLIGHT_COLOR);
    }
    
    is_mouse_button_pressed(MouseButton::Left) && rect.contains(Vec2::from(mouse_position()))
}

pub fn create_slider(x: f32, y: f32, width: f32, value: f32, min: f32, max: f32) -> f32 {
    let height = 20.0;
    let slider_pos = (value - min) / (max - min) * width;
    
    draw_rounded_rectangle(x, y, width, height, 5.0, UI_BG_COLOR);
    draw_rounded_rectangle(x, y, slider_pos, height, 5.0, HIGHLIGHT_COLOR);
    
    let mouse_pos = mouse_position();
    if is_mouse_button_down(MouseButton::Left) 
        && mouse_pos.0 >= x && mouse_pos.0 <= x + width
        && mouse_pos.1 >= y && mouse_pos.1 <= y + height {
        ((mouse_pos.0 - x) / width) * (max - min) + min
    } else {
        value
    }
}

fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
}