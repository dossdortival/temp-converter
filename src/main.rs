use clap::{Parser, ValueEnum};
use colored::Colorize;

/// Temperature Converter CLI
#[derive(Parser)]
#[command(name = "temp-converter")]
#[command(version = "1.0")]
#[command(about = "Convert temperatures between Celsius and Fahrenheit", long_about = None)]
struct Args {
    /// Temperature value to convert
    value: f64,

    /// Conversion type
    #[arg(value_enum)]
    conversion: ConversionType,
}

#[derive(ValueEnum, Clone)]
enum ConversionType {
    /// Convert Fahrenheit to Celsius
    #[clap(name = "ftoc")]
    FtoC,

    /// Convert Celsius to Fahrenheit
    #[clap(name = "ctof")]
    CtoF, 
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Perform conversion based on user choice
    match args.conversion {
        ConversionType::FtoC => {
            let converted = fahrenheit_to_celsius(args.value);
            println!("{}", format!("{}°F is {:.2}°C", args.value, converted).yellow().bold());
        }
        ConversionType::CtoF => {
            let converted = celsius_to_fahrenheit(args.value);
            println!("{}", format!("{}°C is {}°F", args.value, converted).yellow().bold());
        }
    } 

    // Print a message to the user
    println!("{}", "Thanks for using the temp-converter!".bold());

    // Exit the program
    std::process::exit(0);
}