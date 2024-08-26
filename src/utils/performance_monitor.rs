use macroquad::prelude::*;
use crate::constants::TEXT_COLOR;

pub struct PerformanceMonitor {
    fps: i32,
    frame_time: f32,
}

impl PerformanceMonitor {
    pub fn create() -> Self {
        PerformanceMonitor {
            fps: 0,
            frame_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.frame_time += dt;
        if self.frame_time >= 1.0 {
            self.fps = (1.0 / dt) as i32;
            self.frame_time = 0.0;
        }
    }

    pub fn display(&self) {
        let fps_text = format!("FPS: {}", self.fps);
        draw_text(&fps_text, screen_width() - 100.0, screen_height() - 30.0, 20.0, TEXT_COLOR);
    }
}