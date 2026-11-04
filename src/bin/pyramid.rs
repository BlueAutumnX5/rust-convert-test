use std::io::{self, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use turtle::Turtle;

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let amount_input = prompt("Amount: ");
    let size_input = prompt("Size: ");

    // Parse input types matching Python's try/except block
    let (amount, size) = match (amount_input.parse::<i64>(), size_input.parse::<f64>()) {
        (Ok(a), Ok(s)) => (a, s),
        _ => {
            println!("Error: Please enter a valid number");
            sleep(Duration::from_secs(1));
            sleep(Duration::from_secs(2));
            return;
        }
    };

    // Validation
    if amount <= 1 || size <= 0.0 {
        println!("Error: Please enter valid values");
        sleep(Duration::from_secs(1));
        exit(0);
    }

    let total = (amount as f64) * size;

    // Initialize turtle window
    let mut turtle = Turtle::new();

    // Python turtle starts facing East (0°), Rust's starts facing North (90°).
    // Set heading to 0° ensures identical orientation to Python.
    turtle.set_heading(0.0);

    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(total / 2.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.forward(total - size / 2.0);
    turtle.left(90.0);

    for _ in 0..amount {
        turtle.forward(size);
        turtle.left(90.0);
        turtle.forward(size);
        turtle.right(90.0);
    }

    turtle.left(180.0);

    for _ in 0..(amount - 1) {
        turtle.forward(size);
        turtle.right(90.0);
        turtle.forward(size);
        turtle.left(90.0);
    }

    turtle.forward(size);
    turtle.left(90.0);
    turtle.forward(total - size / 2.0 + size);
    turtle.backward(total - size / 2.0);

    for i in 0..(amount - 1) {
        let step = size * ((i + 1) as f64);
        turtle.left(90.0);
        turtle.forward(step);
        turtle.backward(step);
        turtle.right(90.0);
        turtle.forward(size);
    }

    for i in 0..(amount - 1) {
        let step = total - size * ((i + 1) as f64);
        turtle.left(90.0);
        turtle.forward(step);
        turtle.backward(step);
        turtle.right(90.0);
        turtle.forward(size);
    }

    turtle.left(90.0);
    turtle.set_pen_size(2.5);

    for i in 0..amount {
        let len = total * 2.0 - size * 2.0 * ((i + 1) as f64);
        turtle.forward(size);
        turtle.left(90.0);
        turtle.forward(size);
        turtle.forward(len);
        turtle.backward(len);
        turtle.right(90.0);
    }

    turtle.left(180.0);

    for _ in 0..(amount - 1) {
        turtle.forward(size);
        turtle.right(90.0);
        turtle.forward(size);
        turtle.left(90.0);
    }

    turtle.forward(size);
    turtle.left(90.0);
    turtle.forward((total - size / 2.0) * 2.0);

    // Keep window alive before exiting (matches sleep(2))
    sleep(Duration::from_secs(2));

    turtle.wait_for_click();
}