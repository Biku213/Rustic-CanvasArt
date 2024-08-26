# Rustic Canvas Art

Rustic Canvas Art is a lightweight, intuitive drawing application built with Rust using the macroquad framework. It offers a simple yet powerful interface for digital sketching and painting.

## Features

- Intuitive drawing interface with adjustable brush sizes
- Color palette selection
- Eraser tool
- Undo and Redo functionality
- Save artwork as PNG images
- Performance monitoring

## How It Works

Rustic Canvas Art provides a digital canvas where users can draw freely using their mouse or trackpad. The application uses a state-based architecture to manage the drawing process, user interface, and various tools.

Key components:

1. Drawing Engine: Handles sketch creation and rendering
2. User Interface: Manages toolbar buttons, color selection, and brush size adjustment
3. File Handling: Allows saving artwork as PNG images

The application utilizes efficient algorithms for smooth drawing and performance optimization, ensuring a responsive experience even with complex artwork.

## Why Choose Rustic Canvas Art?

1. **Simplicity**: Clean, intuitive interface for effortless digital sketching
2. **Performance**: Built with Rust for optimal speed and efficiency
3. **Customization**: Adjustable brush sizes and color options
4. **Error Recovery**: Undo/Redo functionality for easy mistake correction
5. **Cross-platform**: Works on various operating systems

## Video Demo

[Insert link to video demo here]

## Building and Running

### Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rustic-canvas-art.git
   cd rustic-canvas-art
   ```

2. Build the application:

   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

## Usage

- Left-click and drag to draw
- Use the toolbar to:
  - Select colors
  - Adjust brush size
  - Switch between drawing and eraser modes
  - Undo/Redo actions
  - Clear the canvas
  - Save your artwork

## Keyboard Shortcuts

- `Ctrl+Z`: Undo
- `Ctrl+Y`: Redo
- `Ctrl+S`: Save artwork
- `Ctrl+N`: Clear canvas
- `[`: Decrease brush size
- `]`: Increase brush size
