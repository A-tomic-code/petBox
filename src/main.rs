//! # Tamagotchi Virtual Pet Simulator
//! 
//! Este es un simulador de mascota virtual basado en el clásico juego Tamagotchi.
//! La aplicación permite al usuario cuidar de una mascota virtual que tiene estados
//! de hambre y felicidad que cambian con el tiempo.
//! 
//! ## Arquitectura
//! 
//! El programa utiliza dos hilos:
//! - **Hilo principal**: Maneja la entrada del usuario y las acciones
//! - **Hilo secundario**: Actualiza el estado del Tamagotchi cada segundo
//! 
//! ## Módulos
//! 
//! - `constants`: Contiene todas las constantes del juego
//! - `models`: Contiene la estructura y lógica del Tamagotchi  
//! - `utils`: Funciones de utilidad para entrada/salida

use std::sync::{Arc, Mutex};
use std::{thread, time};

mod constants;
mod models;
mod utils;

use models::tamagotchi::Tamagotchi;

/// Punto de entrada principal de la aplicación Tamagotchi.
/// 
/// Esta función:
/// 1. Inicializa el juego y solicita el nombre de la mascota
/// 2. Crea un Tamagotchi compartido entre hilos usando Arc<Mutex<T>>
/// 3. Lanza un hilo secundario para actualizar el estado cada segundo
/// 4. Ejecuta el bucle principal para manejar las acciones del usuario
fn main() {
    thread::sleep(time::Duration::from_secs(2));
    utils::clear_screen();

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

/// Maneja las acciones del usuario basadas en su entrada.
/// 
/// # Argumentos
/// 
/// * `action` - La acción seleccionada por el usuario como string
/// * `tamagotchi` - Referencia al Tamagotchi compartido entre hilos
/// 
/// # Retorna
/// 
/// * `bool` - `true` si el usuario quiere salir del juego, `false` en caso contrario
/// 
/// # Acciones disponibles
/// 
/// * "1" - Jugar con el Tamagotchi (aumenta felicidad)
/// * "2" - Alimentar al Tamagotchi (reduce hambre)  
/// * "3" - Salir del juego
/// * Cualquier otra entrada - Muestra mensaje de error
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
            println!("¡Adiós!");
            true
        }
        _ => {
            println!("Opción no válida. Inténtalo de nuevo.");
            false
        }
    }
}
