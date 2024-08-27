use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::constants::*;

mod sketch;
pub use sketch::Sketch;

// Enum to represent the current drawing mode
#[derive(PartialEq, Clone, Copy)]
pub enum DrawingMode {
    Draw,
    Erase,
}

pub fn perform_drawing(state: &mut AppState, mouse_pos: &Vec2) {
    match state.drawing_mode {
        DrawingMode::Draw | DrawingMode::Erase => {
            // Start a new sketch when the mouse button is first pressed
            if is_mouse_button_pressed(MouseButton::Left) && is_within_canvas(*mouse_pos) {
                let (color, is_eraser) = get_drawing_properties(state);
                state.active_sketch = Some(Sketch::create(vec![*mouse_pos], state.brush_size, color, true, is_eraser));
            } 
            // Continue drawing while the mouse button is held down
            else if is_mouse_button_down(MouseButton::Left) && is_within_canvas(*mouse_pos) {
                let new_point = simplify_point(state, mouse_pos);

                if let Some(point) = new_point {
                    if let Some(sketch) = &mut state.active_sketch {
                        // Change sketch type from Dot to Line if more than one point
                        if sketch.sketch_type == sketch::SketchType::Dot {
                            sketch.sketch_type = sketch::SketchType::Line;
                        }
                        sketch.points.push(point);
                    } else {
                        // Create a new sketch if there isn't an active one
                        let (color, is_eraser) = get_drawing_properties(state);
                        state.active_sketch = Some(Sketch::create(vec![point], state.brush_size, color, false, is_eraser));
                    }
                }
            } 
            // Finish drawing when the mouse button is released
            else if is_mouse_button_released(MouseButton::Left) {
                stop_drawing(state);
            }
        },
    }
}

// Get the color and eraser status based on the current drawing mode
fn get_drawing_properties(state: &AppState) -> (Color, bool) {
    match state.drawing_mode {
        DrawingMode::Draw => (state.active_color, false),
        DrawingMode::Erase => (CANVAS_COLOR, true),
    }
}

// Simplify the mouse position to avoid too many points close together
fn simplify_point(state: &AppState, mouse_pos: &Vec2) -> Option<Vec2> {
    if let Some(sketch) = &state.active_sketch {
        if let Some(last_point) = sketch.points.last() {
            let distance = last_point.distance(*mouse_pos);
            // Only add point if it's at least 2 pixels away from the last point
            if distance >= 2.0 && distance <= 10.0 {
                Some(*mouse_pos)
            } 
            // If distance is too large, add a point in the direction of mouse movement
            else if distance > 10.0 {
                Some(*last_point + (*mouse_pos - *last_point).normalize() * 10.0)
            } else {
                None
            }
        } else {
            Some(*mouse_pos)
        }
    } else {
        Some(*mouse_pos)
    }
}

// Finalize the sketch and add it to the artwork
fn stop_drawing(state: &mut AppState) {
    if let Some(sketch) = state.active_sketch.take() {
        if !sketch.points.is_empty() {
            state.artwork.push(sketch);
            state.save_checkpoint();
        }
    }
}

// Check if the given point is within the canvas boundaries
fn is_within_canvas(point: Vec2) -> bool {
    point.x > TOOLBAR_WIDTH && 
    point.x < screen_width() - 10.0 && 
    point.y > 10.0 && 
    point.y < screen_height() - 10.0
}