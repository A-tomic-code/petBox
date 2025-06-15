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

	// Crear un vector con algunas notificaciones iniciales
	let mut notifications: Vec<String> = Vec::new();
	notifications.push("This is a notification!".to_string());
	notifications.push("This is a notification!".to_string());
	notifications.push("This is a notification!".to_string());

	// Limpiar la pantalla
	utils::clear_screen();

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

			thread::spawn(move || {
				thread::sleep(Duration::from_secs(1)); // El tick se hace cada segundo
				let mut tamagotchi = tamagotchi_clone.lock().unwrap();
				tamagotchi.tick(); // Actualiza el estado
			});
		}

		// Limpiar la pantalla en cada iteración
		{
			let tamagotchi = tamagotchi.lock().unwrap();
			utils::clear_screen();
			tamagotchi.print_state(); // Imprime el estado del Tamagotchi
		}

		// Imprimir las notificaciones (esto se hará en cada ciclo)
		utils::print_notifications(&mut notifications);

		// Imprimir las opciones del menú
		utils::print_menu(constants::MAIN_MENU_OPTIONS);

		// Verificar la entrada del usuario
		if handle_user_action(&tamagotchi) {
			break;
		}

		// Añadir nuevas notificaciones (simulando nuevas notificaciones durante el juego)
		notifications.push("New notification!".to_string());

		// Reducir el tiempo de espera
		thread::sleep(Duration::from_millis(200)); // Se reduce el tiempo de espera.
	}

	// Finalizar raw mode y volver a la pantalla original
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
