use image::{ImageBuffer, Rgba};
use rfd::FileDialog;
use std::path::PathBuf;
use chrono::Local;
use crate::app_state::AppState;
use crate::constants::*;
use macroquad::prelude::*;


pub fn save_image(state: &AppState, default_name: &str) {
    let file_dialog = FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .set_directory("/")
        .set_file_name(default_name)
        .save_file();

    if let Some(file_path) = file_dialog {
        let path = ensure_png_extension(file_path);
        
        let canvas_width = (screen_width() - TOOLBAR_WIDTH - 20.0) as u32;
        let canvas_height = (screen_height() - 20.0) as u32;

        let bounding_box = get_artwork_bounding_box(state, canvas_width, canvas_height);
        let img_buffer = create_image_buffer(state, &bounding_box);

        match img_buffer.save(&path) {
            Ok(_) => println!("Image saved as: {:?}", path),
            Err(e) => eprintln!("Failed to save image: {}", e),
        }
    } else {
        println!("Save cancelled");
    }
}

fn get_artwork_bounding_box(state: &AppState, canvas_width: u32, canvas_height: u32) -> (u32, u32, u32, u32) {
    let mut min_x = canvas_width;
    let mut min_y = canvas_height;
    let mut max_x = 0;
    let mut max_y = 0;

    for sketch in &state.artwork {
        for point in &sketch.points {
            let x = (point.x - TOOLBAR_WIDTH - 10.0) as u32;
            let y = (point.y - 10.0) as u32;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    if let Some(active_sketch) = &state.active_sketch {
        for point in &active_sketch.points {
            let x = (point.x - TOOLBAR_WIDTH - 10.0) as u32;
            let y = (point.y - 10.0) as u32;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    let padding = state.brush_size.ceil() as u32;
    min_x = min_x.saturating_sub(padding);
    min_y = min_y.saturating_sub(padding);
    max_x = (max_x + padding).min(canvas_width - 1);
    max_y = (max_y + padding).min(canvas_height - 1);

    (min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
}

fn create_image_buffer(state: &AppState, bounding_box: &(u32, u32, u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (min_x, min_y, width, height) = *bounding_box;
    
    let mut img_buffer = ImageBuffer::from_fn(width, height, |_, _| {
        Rgba([255, 255, 255, 255])
    });

    for sketch in &state.artwork {
        draw_sketch_on_image(&mut img_buffer, sketch, min_x, min_y);
    }

    if let Some(active_sketch) = &state.active_sketch {
        draw_sketch_on_image(&mut img_buffer, active_sketch, min_x, min_y);
    }

    img_buffer
}

fn draw_sketch_on_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, sketch: &crate::drawing::Sketch, min_x: u32, min_y: u32) {
    if sketch.points.len() < 2 {
        return;
    }

    let mut iter = sketch.points.windows(2);
    while let Some(&[start, end]) = iter.next() {
        let start_x = (start.x - TOOLBAR_WIDTH - 10.0 - min_x as f32) as i32;
        let start_y = (start.y - 10.0 - min_y as f32) as i32;
        let end_x = (end.x - TOOLBAR_WIDTH - 10.0 - min_x as f32) as i32;
        let end_y = (end.y - 10.0 - min_y as f32) as i32;

        draw_line_on_image(img, start_x, start_y, end_x, end_y, sketch.thickness, sketch.color);
    }
}

fn draw_line_on_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x0: i32, y0: i32, x1: i32, y1: i32, thickness: f32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    // let (width, height) = img.dimensions();
    let radius = (thickness / 2.0) as f32;

    while x != x1 || y != y1 {
        draw_circle_on_image(img, x, y, radius, color);

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    draw_circle_on_image(img, x1, y1, radius, color);
}

fn draw_circle_on_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, center_x: i32, center_y: i32, radius: f32, color: Color) {
    let (width, height) = img.dimensions();
    let r = radius as i32;

    for y in -r..=r {
        for x in -r..=r {
            if x*x + y*y <= r*r {
                let px = center_x + x;
                let py = center_y + y;
                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    let pixel = img.get_pixel_mut(px as u32, py as u32);
                    *pixel = Rgba([
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        (color.a * 255.0) as u8,
                    ]);
                }
            }
        }
    }
}

fn ensure_png_extension(mut path: PathBuf) -> String {
    if path.extension().map_or(true, |ext| ext != "png") {
        path.set_extension("png");
    }
    path.to_str().unwrap_or_default().to_string()
}

pub fn generate_default_filename() -> String {
    let now = Local::now();
    format!("artwork_{}.png", now.format("%Y%m%d_%H%M%S"))
}