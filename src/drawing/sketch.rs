use macroquad::prelude::*;


// Enum to represent the type of sketch
#[derive(Clone, PartialEq)]
pub enum SketchType {
    Line,
    Dot,
}

// Struct to represent a sketch
#[derive(Clone, PartialEq)]
pub struct Sketch {
    pub points: Vec<Vec2>,
    pub thickness: f32,
    pub color: Color,
    pub sketch_type: SketchType,
    pub is_eraser: bool,
}

impl Sketch {
    // Constructor method
    pub fn create(points: Vec<Vec2>, thickness: f32, color: Color, is_dot: bool, is_eraser: bool) -> Self {
        Sketch {
            points,
            thickness,
            color,
            sketch_type: if is_dot { SketchType::Dot } else { SketchType::Line },
            is_eraser,
        }
    }

    // Method to render the sketch
    pub fn render(&self) {
        match self.sketch_type {
            SketchType::Dot => {
                if let Some(point) = self.points.first() {
                    draw_circle(point.x, point.y, self.thickness / 2.0, self.color);
                }
            },
            SketchType::Line => {
                if self.points.len() < 2 {
                    return;
                }
            
                let mut previous_point: Option<Vec2> = None;
                for (i, point) in self.points.iter().enumerate() {
                    if let Some(prev) = previous_point {
                        // Draw line segments and circles for smoother appearance
                        if self.is_eraser {
                            draw_line(prev.x, prev.y, point.x, point.y, self.thickness, WHITE);
                        } else {
                            draw_line(prev.x, prev.y, point.x, point.y, self.thickness, self.color);
                        }
                        if i == 1 {
                            draw_circle(prev.x, prev.y, self.thickness / 2.0, self.color);
                        }
                        draw_circle(point.x, point.y, self.thickness / 2.0, self.color);
                    }
                    previous_point = Some(*point);
                }
            },
        }
    }
}