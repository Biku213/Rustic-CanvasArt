
# Rustic Canvas Art

Rustic Canvas Art is a lightweight, cross-platform drawing application built with Rust using the Macroquad framework. It provides a simple yet powerful interface for digital sketching, offering a smooth experience even for complex artwork. This tool is perfect for users seeking an easy-to-use digital art creation application.

## Features

- **Intuitive Interface**: Simple and responsive, with adjustable brush sizes.
- **Color Palette**: Choose from a wide range of colors.
- **Eraser Tool**: Easily switch between drawing and erasing.
- **Undo/Redo**: Correct mistakes with ease.
- **Save as PNG**: Export artwork as PNG images.
- **Optimized Performance**: Ensures smooth performance even for intricate designs.

## How It Works

Rustic Canvas Art provides a canvas for users to create sketches and paintings using their mouse or trackpad. The application includes tools like a brush, eraser, and color selector, allowing for easy and creative art production. Behind the scenes, the app uses a state-based architecture to manage drawing, user interaction, and performance for a responsive drawing experience.

### Key Components:

1. **Drawing Engine**: Handles canvas rendering and drawing inputs.
2. **User Interface**: Offers a toolbar for color selection, brush adjustment, and file management.
3. **File Handling**: Facilitates exporting artwork in PNG format.

Rustic Canvas Art leverages Rust's efficiency, ensuring fast performance while maintaining simplicity.

## Why Choose Rustic Canvas Art?

- **User-Friendly**: Designed for quick and easy use without a steep learning curve.
- **Fast Performance**: Built with Rust for reliable, efficient performance.
- **Customizable**: Adjustable brush sizes and colors for creative flexibility.
- **Undo/Redo**: Allows users to fix mistakes and improve their work.
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Video Demo

Watch this video demo for a walkthrough of Rustic Canvas Art’s features.
https://github.com/user-attachments/assets/28bcae13-1783-4000-b42b-d7327271870a

## Installation & Setup

### Prerequisites

- Latest stable version of Rust.
- Cargo package manager.

### Building and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rustic-canvas-art.git
   cd rustic-canvas-art
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage Instructions

- **Left-click and drag** on the canvas to draw.
- Use the **toolbar** to:
  - Select colors.
  - Adjust brush size.
  - Switch between drawing and erasing.
  - Undo/Redo actions.
  - Clear the canvas.
  - Save your artwork.

### Keyboard Shortcuts

- `Ctrl + Z`: Undo.
- `Ctrl + Y`: Redo.
- `Ctrl + S`: Save as PNG.
- `Ctrl + N`: Clear canvas.
- `[`: Decrease brush size.
- `]`: Increase brush size.

