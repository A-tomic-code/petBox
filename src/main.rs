use std::sync::{Arc, Mutex};
use std::{thread, time};

mod constants;
mod models;
mod utils;

use models::tamagotchi::Tamagotchi;

fn main() {
    println!("Bienvenido a Tamagotchi!");

    let pet_name = utils::read_input("Introduce el nombre de tu Tamagotchi:");
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
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    });

    // Main loop to interact with the user
    loop {
        // Ask for user input without blocking the secondary thread
        let action = utils::read_from_user();

        match action.as_str() {
            "1" => {
                // Action to play with the Tamagotchi
                let mut tamagotchi = tamagotchi.lock().unwrap();
                tamagotchi.play();
            }
            "2" => {
                // Action to feed the Tamagotchi
                let mut tamagotchi = tamagotchi.lock().unwrap();
                tamagotchi.feed();
            }
            "3" => {
                println!("¡Adiós!");
                break;
            }
            _ => println!("Opción no válida. Inténtalo de nuevo."),
        }
    }

}

