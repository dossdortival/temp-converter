# temp-converter CLI

A simple command-line tool to convert temperatures between Celsius and Fahrenheit.

## Overview

This CLI tool allows you to quickly convert temperatures from Celsius to Fahrenheit and vice versa. It's written in Rust and uses the `clap` crate for command-line argument parsing.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.

### Steps

1. Clone this repository:

   ```bash
   git clone https://github.com/dossdortival/temp-converter.git
   cd temp-converter
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Install the binary:

   ```bash
   cargo install --path .
   ```

   This will install the `temp-converter` binary on your system.

## Usage

### Basic Usage

Convert a temperature from Fahrenheit to Celsius:

```bash
temp-converter 100 ftoc
```

Convert a temperature from Celsius to Fahrenheit:

```bash
temp-converter 100 ctof
```

### Help

To see the help message and available options, run:

```bash
temp-converter --help
```

### Examples

1. Convert 32°F to Celsius:

   ```bash
   temp-converter 32 ftoc
   ```

2. Convert 0°C to Fahrenheit:

   ```bash
   temp-converter 0 ctof
   ```

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them.
4. Push your changes to your fork.
5. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

Thanks for using `temp-converter`