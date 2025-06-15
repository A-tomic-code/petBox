//! # Tamagotchi Virtual Pet Simulator
//!
//! This is a virtual pet simulator based on the classic Tamagotchi game.
//! The application allows the user to care for a virtual pet that has states
//! of hunger and happiness that change over time.
//!
//! ## Architecture
//!
//! The program uses two threads:
//! - **Main Thread**: Handles user input and actions
//! - **Secondary Thread**: Updates the Tamagotchi state every second
//!
//! ## Modules
//!
//! - `constants`: Contains all game constants
//! - `models`: Contains the structure and logic of the Tamagotchi  
//! - `utils`: Utility functions for input/output

use std::sync::{Arc, Mutex};
use std::{thread, time};

mod constants;
mod models;
mod utils;

use models::tamagotchi::Tamagotchi;

/// Main entry point of the Tamagotchi application.
///
/// This function:
/// 1. Initializes the game and requests the pet's name
/// 2. Creates a Tamagotchi shared among threads using Arc<Mutex<T>>
/// 3. Spawns a secondary thread to update the state every second
/// 4. Executes the main loop to handle user actions
fn main() {
    thread::sleep(time::Duration::from_secs(2));
    utils::clear_screen();

    println!("Welcome to Tamagotchi!");

    let pet_name = utils::read_input("Enter your Tamagotchi's name:");
    let tamagotchi = Arc::new(Mutex::new(Tamagotchi::new(pet_name)));

    // Spawn a thread to handle the tick for the Tamagotchi
    thread::spawn({
        let tamagotchi = Arc::clone(&tamagotchi);
        move || {
            loop {
                // Clear the screen to avoid cluttering the terminal
                utils::clear_screen();

                // Perform the tick (update the state) for the Tamagotchi
                {
                    let mut tamagotchi = tamagotchi.lock().unwrap();
                    tamagotchi.tick();
                    tamagotchi.print_state();
                }

                // Print the menu (loop it every second)
                utils::print_menu(constants::MAIN_MENU_OPTIONS);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    });

    loop {
        let action = utils::read_from_user();

        if handle_user_action(&action, &tamagotchi) {
            break;
        }
    }
}

/// Handles user actions based on their input.
///
/// # Arguments
///
/// * `action` - The action selected by the user as a string
/// * `tamagotchi` - Reference to the Tamagotchi shared among threads
///
/// # Returns
///
/// * `bool` - `true` if the user wants to exit the game, `false` otherwise
///
/// # Available Actions
///
/// * "1" - Play with the Tamagotchi (increases happiness)
/// * "2" - Feed the Tamagotchi (reduces hunger)  
/// * "3" - Exit the game
/// * Any other input - Displays an error message
fn handle_user_action(action: &str, tamagotchi: &Arc<Mutex<Tamagotchi>>) -> bool {
    match action {
        "1" => {
            // Action to play with the Tamagotchi
            let mut tamagotchi = tamagotchi.lock().unwrap();
            tamagotchi.play();
            false
        }
        "2" => {
            // Action to feed the Tamagotchi
            let mut tamagotchi = tamagotchi.lock().unwrap();
            tamagotchi.feed();
            false
        }
        "3" => {
            println!("Goodbye!");
            true
        }
        _ => {
            println!("Invalid option. Please try again.");
            false
        }
    }
}
