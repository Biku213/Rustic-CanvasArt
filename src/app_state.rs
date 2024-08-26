use macroquad::prelude::*;
use crate::drawing::{Sketch, DrawingMode};

pub struct Tooltip {
    pub text: String,
    pub position: Vec2,
    pub visible: bool,
}

pub struct AppState {
    pub previous_mouse_pos: Option<Vec2>,
    pub artwork: Vec<Sketch>,
    pub undo_stack: Vec<Vec<Sketch>>,
    pub redo_stack: Vec<Vec<Sketch>>,
    pub active_sketch: Option<Sketch>,
    pub brush_size: f32,
    pub active_color: Color,
    pub palette: Vec<Color>,
    pub min_brush_size: f32,
    pub max_brush_size: f32,
    pub tooltip: Tooltip,
    pub drawing_mode: DrawingMode,
}

impl AppState {
    pub fn create() -> Self {
        let mut state = AppState {
            drawing_mode: DrawingMode::Draw,
            previous_mouse_pos: None,
            artwork: Vec::new(),
            active_sketch: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            brush_size: 10.0,
            active_color: BLACK,
            palette: vec![RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE, BLACK, WHITE],
            min_brush_size: 1.0,
            max_brush_size: 50.0,
            tooltip: Tooltip {
                text: String::new(),
                position: Vec2::new(0.0, 0.0),
                visible: false,
            },
        };
        state.save_checkpoint();
        state
    }

    pub fn show_tooltip(&mut self, text: &str, position: Vec2) {
        self.tooltip.text = text.to_string();
        self.tooltip.position = position;
        self.tooltip.visible = true;
    }

    pub fn hide_tooltip(&mut self) {
        self.tooltip.visible = false;
    }

    pub fn undo(&mut self) {
        if self.undo_stack.len() > 1 {
            if let Some(current_state) = self.undo_stack.pop() {
                self.redo_stack.push(current_state);
                self.artwork = self.undo_stack.last().unwrap().clone();
            }
        }
    }
    
    pub fn redo(&mut self) {
        if let Some(next_state) = self.redo_stack.pop() {
            self.undo_stack.push(self.artwork.clone());
            self.artwork = next_state;
        }
    }
    
    pub fn save_checkpoint(&mut self) {
        let current_state = self.artwork.clone();
        if self.undo_stack.last() != Some(&current_state) {
            self.undo_stack.push(current_state);
            self.redo_stack.clear();
            
            if self.undo_stack.len() > 50 {
                self.undo_stack.remove(0);
            }
        }
    }

    pub fn clear_canvas(&mut self) {
        self.save_checkpoint();
        self.artwork.clear();
        self.active_sketch = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.previous_mouse_pos = None;
        self.drawing_mode = DrawingMode::Draw;
        self.save_checkpoint();
    }

    pub fn render_sketch(&self, sketch: &Sketch) {
        sketch.render();
    }
}