use std::{
    io::{self, Write}, // Aquí importamos io y Write para usar stdout
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod constants;
mod models;
mod utils;

use crossterm::{
    event::{self, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;

use models::tamagotchi::Tamagotchi;

fn main() {
    // Crear un objeto stdout, que es el flujo estándar de salida
    let mut stdout = stdout();

    // Limpiar la pantalla
    utils::clear_screen();
    println!("Welcome to Tamagotchi!");

    let pet_name = utils::read_input("Enter your Tamagotchi's name:");
    let tamagotchi = Arc::new(Mutex::new(Tamagotchi::new(pet_name)));

    utils::change_text_color(constants::NORMAL_COLOR);

    // Establecer el terminal en raw mode
    stdout.execute(crossterm::cursor::EnableBlinking).unwrap();
    stdout.execute(EnterAlternateScreen).unwrap(); // Entramos en el modo de pantalla alterna (opcional)
    crossterm::terminal::enable_raw_mode().unwrap(); // Habilitar raw mode

    let mut last_tick = std::time::Instant::now();
    loop {
        // Verificar si ha pasado un segundo desde el último tick
        if last_tick.elapsed() >= Duration::from_secs(1) {
            last_tick = std::time::Instant::now();
            let tamagotchi_clone = Arc::clone(&tamagotchi);

            thread::spawn(move || {
                // Hilo secundario para hacer el "tick"
                thread::sleep(Duration::from_secs(1)); // El tick se hace cada segundo
                let mut tamagotchi = tamagotchi_clone.lock().unwrap();
                tamagotchi.tick(); // Actualiza el estado
            });
        }

        // Actualizar estado visual del Tamagotchi
        {
            let tamagotchi = tamagotchi.lock().unwrap();
            utils::clear_game_area();
            tamagotchi.print_state();
        }

        // Imprimir las opciones de menú en la parte inferior
        utils::print_menu(constants::MAIN_MENU_OPTIONS);

        // Escuchar teclas sin bloquear
        if handle_user_action(&tamagotchi) {
            break; // Salir del juego si el usuario selecciona la opción de salir
        }

        // No dormir tanto, para capturar teclas rápidamente.
        thread::sleep(Duration::from_millis(200)); // Se reduce el tiempo de espera.
    }

    // Deshabilitar el raw mode y volver al modo normal del terminal
    crossterm::terminal::disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap(); // Salir del modo de pantalla alterna (opcional)

    println!("Exiting the game...");
}

/// Maneja las acciones del usuario basadas en la entrada de teclado.
///
/// # Argumentos
/// * `tamagotchi` - Referencia al Tamagotchi compartido entre hilos
///
/// # Retorna
/// * `bool` - `true` si el usuario quiere salir del juego, `false` en caso contrario
fn handle_user_action(tamagotchi: &Arc<Mutex<Tamagotchi>>) -> bool {
    let mut should_exit = false;

    // Usamos poll para escuchar las teclas sin bloquear
    if event::poll(Duration::from_millis(50)).unwrap() {
        // Reducir el retardo aquí también
        if let event::Event::Key(event) = event::read().unwrap() {
            match event.code {
                KeyCode::Char('1') => {
                    // Acción para jugar con el Tamagotchi
                    let mut tamagotchi = tamagotchi.lock().unwrap();
                    tamagotchi.play();
                }
                KeyCode::Char('2') => {
                    // Acción para alimentar al Tamagotchi
                    let mut tamagotchi = tamagotchi.lock().unwrap();
                    tamagotchi.feed();
                }
                KeyCode::Char('3') => {
                    // Acción para salir del juego
                    should_exit = true;
                }
                _ => {
                    // Ignorar otras teclas
                }
            }
        }
    }

    should_exit
}
