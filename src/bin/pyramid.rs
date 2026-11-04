//! Pyramid Turtle Generator — translated from pyramid.py
//! Uses macroquad for graphics with a custom Turtle struct
//! (the `turtle` crate is incompatible with modern Linux/Wayland)

use macroquad::prelude::*;
use std::io::{self, Write};
use std::process::exit;

// ── Input helper ────────────────────────────────────────────────────────────

/// Prints a prompt and returns the user's input as a String.
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// ── Mini Turtle ──────────────────────────────────────────────────────────────
// Since `turtle` crate is broken on modern Linux, we implement our own.
// It mimics Python's turtle API: forward/backward/left/right/pen_up/pen_down.

struct Turtle {
    x: f64,
    y: f64,
    angle: f64,     // degrees, 0° = East, counter-clockwise positive
    is_pen_down: bool,
    pen_size: f32,
    lines: Vec<(f32, f32, f32, f32, f32)>, // x1, y1, x2, y2, thickness
}

impl Turtle {
    fn new(start_x: f64, start_y: f64) -> Self {
        Self {
            x: start_x,
            y: start_y,
            angle: 0.0,
            is_pen_down: true,
            pen_size: 1.0,
            lines: Vec::new(),
        }
    }

    fn set_heading(&mut self, angle: f64) {
        self.angle = angle;
    }

    fn forward(&mut self, dist: f64) {
        let rad = self.angle.to_radians();
        let nx = self.x + dist * rad.cos();
        let ny = self.y - dist * rad.sin(); // screen Y is flipped vs math Y
        if self.is_pen_down && dist != 0.0 {
            self.lines.push((
                self.x as f32, self.y as f32,
                nx as f32,     ny as f32,
                self.pen_size,
            ));
        }
        self.x = nx;
        self.y = ny;
    }

    fn backward(&mut self, dist: f64) {
        self.forward(-dist);
    }

    fn left(&mut self, angle: f64) {
        self.angle = (self.angle + angle) % 360.0;
    }

    fn right(&mut self, angle: f64) {
        self.angle = (self.angle - angle + 360.0) % 360.0;
    }

    fn pen_up(&mut self)   { self.is_pen_down = false; }
    fn pen_down(&mut self) { self.is_pen_down = true;  }

    fn set_pen_size(&mut self, size: f32) {
        self.pen_size = size;
    }
}

// ── Window config ────────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: "Pyramid".to_string(),
        window_width: 700,
        window_height: 700,
        ..Default::default()
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    let amount_input = prompt("Amount: ");
    let size_input   = prompt("Size: ");

    // Parse inputs — replicates Python's try/except block
    let (amount, size) = match (amount_input.parse::<i64>(), size_input.parse::<f64>()) {
        (Ok(a), Ok(s)) => (a, s),
        _ => {
            println!("Error: Please enter a valid number");
            return;
        }
    };

    // Validation
    if amount <= 1 || size <= 0.0 {
        println!("Error: Please enter valid values");
        exit(0);
    }

    let total = (amount as f64) * size;

    // Auto-scale so the pyramid fits nicely in the window
    let scale = (700.0_f64 * 0.6) / (total * 2.0).max(1.0);

    // Start turtle in the center of the window
    let mut turtle = Turtle::new(350.0, 350.0);

    // ── Drawing logic (identical structure to pyramid.py) ──────────────────

    turtle.set_heading(0.0);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward((total / 2.0) * scale);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.forward((total - size / 2.0) * scale);
    turtle.left(90.0);

    for _ in 0..amount {
        turtle.forward(size * scale);
        turtle.left(90.0);
        turtle.forward(size * scale);
        turtle.right(90.0);
    }

    turtle.left(180.0);

    for _ in 0..(amount - 1) {
        turtle.forward(size * scale);
        turtle.right(90.0);
        turtle.forward(size * scale);
        turtle.left(90.0);
    }

    turtle.forward(size * scale);
    turtle.left(90.0);
    turtle.forward((total - size / 2.0 + size) * scale);
    turtle.backward((total - size / 2.0) * scale);

    for i in 0..(amount - 1) {
        let step = size * ((i + 1) as f64) * scale;
        turtle.left(90.0);
        turtle.forward(step);
        turtle.backward(step);
        turtle.right(90.0);
        turtle.forward(size * scale);
    }

    for i in 0..(amount - 1) {
        let step = (total - size * ((i + 1) as f64)) * scale;
        turtle.left(90.0);
        turtle.forward(step);
        turtle.backward(step);
        turtle.right(90.0);
        turtle.forward(size * scale);
    }

    turtle.left(90.0);
    turtle.set_pen_size(2.5);

    for i in 0..amount {
        let len = (total * 2.0 - size * 2.0 * ((i + 1) as f64)) * scale;
        turtle.forward(size * scale);
        turtle.left(90.0);
        turtle.forward(size * scale);
        turtle.forward(len);
        turtle.backward(len);
        turtle.right(90.0);
    }

    turtle.left(180.0);

    for _ in 0..(amount - 1) {
        turtle.forward(size * scale);
        turtle.right(90.0);
        turtle.forward(size * scale);
        turtle.left(90.0);
    }

    turtle.forward(size * scale);
    turtle.left(90.0);
    turtle.forward((total - size / 2.0) * 2.0 * scale);

    // ── Render loop ────────────────────────────────────────────────────────
    // All lines are pre-computed above; we just redraw them every frame.

    loop {
        clear_background(WHITE);

        for &(x1, y1, x2, y2, thickness) in &turtle.lines {
            draw_line(x1, y1, x2, y2, thickness, BLACK);
        }

        draw_text("Press ESC to exit", 10.0, 20.0, 18.0, DARKGRAY);
        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}
