//! # Utilities Module
//!
//! This module contains utility functions for input/output
//! and console interface manipulation.

use super::constants;
use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use std::io::{self, stdout, Write};

pub fn print_line(text: &str) {
    execute!(std::io::stdout(), cursor::MoveToNextLine(0)).unwrap();
    println!("{}", text);
    execute!(std::io::stdout(), cursor::MoveToNextLine(0)).unwrap();

    stdout().flush().unwrap();
}

pub fn clear_screen() {
    execute!(
        stdout(),
        Clear(ClearType::FromCursorDown),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
    stdout().flush().unwrap();
}

pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input.trim().to_string()
}
pub fn print_menu(items: [&str; 3]) {
    print_line("--------------------------------");
    print_line("Select an option:");
    print_line("--------------------------------");

    for (index, item) in items.iter().enumerate() {
        let option = format!("{}. {}", index + 1, item);
        print_line(option.as_str());
    }

    print_line("--------------------------------")
}

pub fn change_text_color(color: Color) {
    execute!(stdout(), SetForegroundColor(color)).unwrap();
}

pub fn print_warning(text: &str) {
    change_text_color(constants::WARNING_COLOR);
    print_line(text);
    change_text_color(constants::NORMAL_COLOR);
}
