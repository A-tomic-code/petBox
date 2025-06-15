use std::{
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
use models::tamagotchi::Tamagotchi;
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    // Limpiar la pantalla
    utils::clear_screen();

    // println!("Welcome to PetBox!");
    // let pet_name = utils::read_input("Enter your Tamagotchi's name:");

    let pet_name = "Tamagotchi".to_string();
    let tamagotchi = Arc::new(Mutex::new(Tamagotchi::new(pet_name)));

    utils::change_text_color(constants::NORMAL_COLOR);

    // Establecer el terminal en raw mode
    stdout.execute(crossterm::cursor::EnableBlinking).unwrap();
    stdout.execute(EnterAlternateScreen).unwrap(); // Entramos en el modo de pantalla alterna (opcional)
    crossterm::terminal::enable_raw_mode().unwrap(); // Habilitar raw mode

    let mut last_tick = std::time::Instant::now();
    loop {
        if last_tick.elapsed() >= Duration::from_secs(1) {
            last_tick = std::time::Instant::now();
            let tamagotchi_clone = Arc::clone(&tamagotchi);

            let mut tamagotchi = tamagotchi_clone.lock().unwrap();
            tamagotchi.tick();
        }

        {
            let mut tamagotchi = tamagotchi.lock().unwrap();
            utils::clear_screen();
            tamagotchi.print_state();
            tamagotchi.print_notifications();
        }

        utils::print_menu(constants::MAIN_MENU_OPTIONS);

        if handle_user_action(&tamagotchi) {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }

    utils::print_error("Game Over!");

    crossterm::terminal::disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();

    println!("Exiting the game...");
}

fn handle_user_action(tamagotchi: &Arc<Mutex<Tamagotchi>>) -> bool {
    let mut should_exit = false;

    if event::poll(Duration::from_millis(50)).unwrap() {
        if let event::Event::Key(event) = event::read().unwrap() {
            match event.code {
                KeyCode::Char('1') => {
                    let mut tamagotchi = tamagotchi.lock().unwrap();
                    tamagotchi.play();
                }
                KeyCode::Char('2') => {
                    let mut tamagotchi = tamagotchi.lock().unwrap();
                    tamagotchi.feed();
                }
                KeyCode::Char('3') => {
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
