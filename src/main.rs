use std::io;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn main() {
    println!("Temperature Converter");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    // Get user choice
    let choice: u8 = loop {
        println!("Enter your choice (1 or 2):");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(1) | Ok(2) => break input.trim().parse().unwrap(),
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    };

    // Get temperature input
    let temperature: f64 = loop {
        println!("Enter the temperature:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(temp) => break temp,
            _ => println!("Invalid input. Please enter a number."),
        }
    };

    // Perform conversion based on user choice
    match choice {
        1 => {
            let converted = fahrenheit_to_celsius(temperature);
            println!("{}°F is {}°C", temperature, converted);
        }
        2 => {
            let converted = celsius_to_fahrenheit(temperature);
            println!("{}°C is {}°F", temperature, converted);
        }
        _ => unreachable!(), // This will never happen because of the choice validation
    }
}