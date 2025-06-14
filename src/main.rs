use std::sync::mpsc;
use std::{io, process, thread, time};

mod constants;
mod models;
mod utils;

use models::tamagotchi::Tamagotchi;

fn main() {
    println!("Bienvenido a Tamagotchi!");

    let pet_name = utils::read_input("Introduce el nombre de tu Tamagotchi:");
    let mut tamagotchi = Tamagotchi::new(pet_name);

    // Create a channel for sending actions to the tamagotchi thread.
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            utils::clear_screen();
            println!("\nEstado actual de {}:", tamagotchi.name);
            println!("Felicidad: {}", tamagotchi.happiness);
            println!("Hambre: {}", tamagotchi.hunger);
            println!("--------------------------------");

            tamagotchi.tick();
            thread::sleep(time::Duration::from_secs(1));

            // Check for actions from the main thread.
            match rx.try_recv() {
                Ok(action) => match action.as_str() {
                    "1" => tamagotchi.play(),
                    "2" => tamagotchi.feed(),
                    "3" => {
                        println!("¡Adiós!");
                        process::exit(0);
                    }
                    _ => println!("Opción no válida."),
                },
                Err(_) => {} // No action received, continue looping.
            }
        }
    });

    loop {
        let action = utils::read_input("¿Qué quieres hacer?\n1. Jugar\n2. Alimentar\n3. Salir");

        match action.as_str() {
            "1" | "2" | "3" => {
                tx.send(action.clone()).unwrap();
                if action == "3" {
                    break;
                }
            }
            _ => println!("Opción no válida. Inténtalo de nuevo."),
        }
    }
}
