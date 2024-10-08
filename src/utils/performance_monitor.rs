use macroquad::prelude::*;
use crate::constants::FPS_TEXT_COLOR;

// Struct to monitor and display performance metrics
pub struct PerformanceMonitor {
    fps: i32,
    frame_time: f32,
}

impl PerformanceMonitor {
    // Constructor for PerformanceMonitor
    pub fn create() -> Self {
        PerformanceMonitor {
            fps: 0,
            frame_time: 0.0,
        }
    }

    // Update performance metrics
    pub fn update(&mut self, dt: f32) {
        self.frame_time += dt;
        // Update FPS count every second
        if self.frame_time >= 1.0 {
            self.fps = (1.0 / dt) as i32;
            self.frame_time = 0.0;
        }
    }

    // Display FPS on screen
    pub fn display(&self) {
        let fps_text = format!("FPS: {}", self.fps);
        draw_text(&fps_text, screen_width() - 100.0, screen_height() - 30.0, 20.0, FPS_TEXT_COLOR);
    }
}