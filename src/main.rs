use std::io;

const ERROR_MESSAGE_PROMPT: &str = "Really? Try again...";

fn main() {
    display_menu();
    let chosen_option = read_and_validate_menu_option();
    let degrees = read_degrees();

    if chosen_option == 1 {
        let celsius_degrees = fahrenheit_to_celsius(degrees);
        println!("{}F is roughly {}C", degrees, round_2(celsius_degrees));
    } else {
        let fahrenheit_degrees = celsius_to_fahrenheit(degrees);
        println!("{}C is roughly {}F", degrees, round_2(fahrenheit_degrees));
    };
}

fn display_menu() {
    println!("Fahrenheit <=> Celsius converter");
    println!("What would you like to convert?(pick a number)");
    println!("1. Fahrenheit => Celsius");
    println!("2. Celsius => Fahrenheit");
}

fn read_and_validate_menu_option() -> u8 {
    return loop {
        let mut chosen_option = String::new();
        io::stdin()
            .read_line(&mut chosen_option)
            .expect("Failed to read line");

        let maybe_valid_option = chosen_option
            .trim()
            .parse()
            .ok()
            .and_then(|option| validate_menu_option(option));

        match maybe_valid_option {
            Some(num) => break num,
            None => log_error(),
        }
    };
}

fn validate_menu_option(option: u8) -> Option<u8> {
    if (1..=2).contains(&option) {
        Some(option)
    } else {
        None
    }
}

fn read_degrees() -> f32 {
    println!("Place your number for conversion:");

    return loop {
        let mut degrees = String::new();
        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");

        match degrees.trim().parse() {
            Ok(num) => break num,
            Err(_) => log_error(),
        };
    };
}

fn log_error() {
    println!("{}", ERROR_MESSAGE_PROMPT);
}

fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    return (degrees - 32.0) / 1.8;
}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {
    return (degrees * 1.8) + 32.0;
}

fn round_2(number: f32) -> f32 {
    return (number * 100.0).round() / 100.0;
}
