use std::io;
use std::ptr;
use std::result::Result;

fn main() {
    display_menu();
    let chosen_option: u8 = read_and_validate_menu_option();

    if chosen_option == 1 {
        println!("Converting F=>C");
        let degrees: f32 = match read_degrees() {
            Ok(num) => num,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };
        match validate_fahrenheit_degrees(degrees) {
            Ok(_) => {
                let celsius_degrees = fahrenheit_to_celsius(degrees);
                println!("{}F is roughly {}C", degrees, round_2(celsius_degrees));
            }
            Err(msg) => println!("{}", msg),
        };
    } else {
        println!("Converting C=>F");
        let degrees: f32 = match read_degrees() {
            Ok(num) => num,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };
        match validate_celsius_degrees(degrees) {
            Ok(_) => {
                let fahrenheit_degrees = celsius_to_fahrenheit(degrees);
                println!("{}C is roughly {}F", degrees, round_2(fahrenheit_degrees));
            }
            Err(msg) => println!("{}", msg),
        };
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

        let chosen_option: u8 = match chosen_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Really? Try again...");
                continue;
            }
        };
        if chosen_option == 0 || chosen_option > 2 {
            println!("Really? Try again...");
            continue;
        } else {
            break chosen_option;
        }
    };
}

fn read_degrees() -> Result<f32, String> {
    println!("Place your number for conversion:");
    let mut degrees = String::new();
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    return match degrees.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Failed to parse number".to_string()),
    };
}

fn validate_fahrenheit_degrees(degrees: f32) -> Result<*const f32, String> {
    if degrees >= -459.67 {
        return Ok(ptr::null());
    } else {
        return Err("Given temperature is below absolute 0.".to_string());
    }
}

fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    return (degrees - 32.0) / 1.8;
}

fn validate_celsius_degrees(degrees: f32) -> Result<*const f32, String> {
    if degrees >= -273.15 {
        return Ok(ptr::null());
    } else {
        return Err("Given temperature is below absolute 0.".to_string());
    }
}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {
    return (degrees * 1.8) + 32.0;
}

fn round_2(number: f32) -> f32 {
    return (number * 100.0).round() / 100.0;
}
