//! # Utilities Module
//!
//! This module contains utility functions for input/output
//! and console interface manipulation.

use std::io::{self, Write};
use std::process::Command;
pub fn clear_screen() {
    // Command to clear the screen in Unix/Linux
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error clearing the screen");
    } else {
        print!("\x1B[2J\x1B[1;1H"); // Command for Unix systems
    }
    io::stdout().flush().unwrap();
}

/// Reads a line of input from the user showing a prompt.
///
/// # Arguments
///
/// * `prompt` - The message shown to the user before waiting for input
///
/// # Returns
///
/// * `String` - The user's input trimmed of leading/trailing whitespace
///
/// # Example
///
/// ```
/// let name = read_input("What is your name?");
/// println!("Hello, {}!", name);
/// ```
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input.trim().to_string()
}
pub fn print_menu(items: [&str; 3]) {
    println!("\n--------------------------------\n");
    println!("Select an option:\n");
    println!("--------------------------------\n");
    for (index, item) in items.iter().enumerate() {
        println!("{}. {}\n", index + 1, item);
    }
    println!("--------------------------------\n")
}
pub fn choose_option(items: [&str; 3]) -> u32 {
    let mut input = String::new();
    print_menu(items);
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input.trim().parse().expect("Please enter a valid number")
}

// Reads user input without a specific prompt given.
pub fn read_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input.trim().to_string()
}
