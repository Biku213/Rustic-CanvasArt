use macroquad::prelude::*;
mod app_state;
mod ui;
mod drawing;
mod utils;
mod constants;

use app_state::AppState;
use constants::*;
use drawing::perform_drawing;
use ui::display_ui;
use utils::performance_monitor::PerformanceMonitor;
use utils::file_operations::save_image;

#[macroquad::main("Rustic Canvas Art")]
async fn main() {
    execute().await;
}

pub async fn execute() {
    let mut state = AppState::create();
    let mut performance_monitor = PerformanceMonitor::create();

    loop {
        let dt = get_frame_time();
        performance_monitor.update(dt);

        clear_background(BG_COLOR);
        
        let canvas_x = TOOLBAR_WIDTH.min(screen_width()) + 10.0;
        let canvas_y = 10.0;
        let canvas_width = screen_width() - canvas_x - 10.0;
        let canvas_height = screen_height() - 20.0;
        draw_rectangle(canvas_x, canvas_y, canvas_width, canvas_height, CANVAS_COLOR);

        display_ui(&mut state);

        handle_keyboard_input(&mut state);

        let mouse_pos = Vec2::from(mouse_position());
        perform_drawing(&mut state, &mouse_pos);
        
        display_artwork(&state);

        if is_within_canvas(mouse_pos) {
            display_cursor(&state, mouse_pos);
        }

        performance_monitor.display();

        state.hide_tooltip();

        next_frame().await
    }
}

fn handle_keyboard_input(state: &mut AppState) {
    if is_key_pressed(KeyCode::LeftBracket) {
        state.brush_size = (state.brush_size - 1.0).max(state.min_brush_size);
    }
    if is_key_pressed(KeyCode::RightBracket) {
        state.brush_size = (state.brush_size + 1.0).min(state.max_brush_size);
    }
    if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
        state.undo();
    }
    if is_key_pressed(KeyCode::Y) && is_key_down(KeyCode::LeftControl) {
        state.redo();
    }
    if is_key_pressed(KeyCode::S) && is_key_down(KeyCode::LeftControl) {
        let default_name = utils::file_operations::generate_default_filename();
        save_image(state, &default_name);
    }
    if is_key_pressed(KeyCode::N) && is_key_down(KeyCode::LeftControl) {
        state.clear_canvas();
    }
}

// Display all artwork on the canvas
fn display_artwork(state: &AppState) {
    for sketch in &state.artwork {
        state.render_sketch(sketch);
    }

    if let Some(active_sketch) = &state.active_sketch {
        state.render_sketch(active_sketch);
    }
}

// Display the cursor based on the current drawing mode
fn display_cursor(state: &AppState, mouse_pos: Vec2) {
    match state.drawing_mode {
        drawing::DrawingMode::Draw => {
            draw_circle(mouse_pos.x, mouse_pos.y, state.brush_size / 2.0, state.active_color);
        },
        drawing::DrawingMode::Erase => {
            let color = Color::new(0.5, 0.5, 0.5, 0.5);
            draw_circle(mouse_pos.x, mouse_pos.y, state.brush_size / 2.0, color);
            draw_circle_lines(mouse_pos.x, mouse_pos.y, state.brush_size / 2.0, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
        },
    }
}

// Check if a point is within the canvas boundaries
fn is_within_canvas(point: Vec2) -> bool {
    point.x > TOOLBAR_WIDTH && 
    point.x < screen_width() - 10.0 && 
    point.y > 10.0 && 
    point.y < screen_height() - 10.0
}