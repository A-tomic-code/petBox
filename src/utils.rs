use std::io::{self, Write};
use std::process::Command;

pub fn clear_screen() {
    // Comando para limpiar la pantalla en Unix/Linux
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error al limpiar la pantalla");
    } else {
        print!("\x1B[2J\x1B[1;1H"); // Comando para sistemas Unix
    }
    io::stdout().flush().unwrap();
}

pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input.trim().to_string()
}
