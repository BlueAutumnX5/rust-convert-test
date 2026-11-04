use std::io::{self, Write};

/// Helper function to print a prompt and read user input from stdin.
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let first_value_str = prompt("First Value: ");
    let end_value_str = prompt("End Value: ");

    // Parse input types (replicates Python's try/except block)
    let (first_value, end_value) = match (
        first_value_str.parse::<i64>(),
                                          end_value_str.parse::<i64>(),
    ) {
        (Ok(f), Ok(e)) => (f, e),
        _ => {
            println!("Error: First Value and End Value must be filled a number");
            prompt("Press ENTER to exit");
            return;
        }
    };

    if first_value >= 2 && first_value < end_value {
        let mut count = 0;
        let mut counting = first_value;

        print!("Prime Numbers: ");
        io::stdout().flush().expect("Failed to flush stdout");

        while counting <= end_value {
            let a = counting / 2;
            let b = (counting as f64).sqrt();

            if counting <= 3 {
                print!("{} ", counting);
                io::stdout().flush().expect("Failed to flush stdout");
                counting += 1;
                count += 1;
            } else if counting % 2 == 0 {
                counting += 1;
            } else if b.fract() == 0.0 {
                // Equivalent to `b == int(b)` (checking for perfect squares)
                counting += 2;
            } else if a >= 2 {
                let mut is_prime = true;

                // range(3, a + 1, 2)
                let mut i = 3;
                while i <= a {
                    if counting % i == 0 {
                        is_prime = false;
                        break;
                    }
                    i += 2;
                }

                if is_prime {
                    print!("{} ", counting);
                    io::stdout().flush().expect("Failed to flush stdout");
                    count += 1;
                }
                counting += 2;
            }
        }

        println!();
        let total_numbers = (end_value - first_value + 1) as f64;
        let percentage = (count as f64 / total_numbers) * 100.0;

        println!("Total prime numbers found: {}", count);
        println!("Percentage of prime numbers: {:.2}%", percentage);
        println!();
        println!("Done");
    } else {
        println!("Error: Sorry, the Program cannot be run because First Value is less than 2 or higher than End Value");
    }

    prompt("Press ENTER to exit");
}
