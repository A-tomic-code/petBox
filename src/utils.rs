//! # Módulo de Utilidades
//! 
//! Este módulo contiene funciones de utilidad para la entrada/salida
//! y manipulación de la interfaz de consola.

use std::io::{self, Write};
use std::process::Command;

/// Limpia la pantalla de la consola.
/// 
/// Esta función detecta automáticamente el sistema operativo:
/// - **Windows**: Usa el comando `cls`
/// - **Unix/Linux/macOS**: Usa secuencias de escape ANSI
/// 
/// # Errores
/// 
/// La función puede fallar si no se puede ejecutar el comando de limpieza
/// o si no se puede escribir a stdout.
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

/// Lee una línea de entrada del usuario mostrando un prompt.
/// 
/// # Argumentos
/// 
/// * `prompt` - El mensaje que se muestra al usuario antes de esperar entrada
/// 
/// # Retorna
/// 
/// * `String` - La entrada del usuario sin espacios en blanco al inicio/final
/// 
/// # Panics
/// 
/// Esta función entrará en pánico si no se puede leer desde stdin.
/// 
/// # Ejemplo
/// 
/// ```
/// let name = read_input("¿Cuál es tu nombre?");
/// println!("Hola, {}!", name);
/// ```
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input.trim().to_string()
}

pub fn print_menu(items: [&str; 3]) {
    println!("\n--------------------------------\n");
    println!("Selecciona una opción:\n");
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
        .expect("Error al leer la línea");

    input
        .trim()
        .parse()
        .expect("Por favor, introduce un número válido")
}

pub fn read_from_user() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    input.trim().to_string()
}
}

/// Lee entrada del usuario sin mostrar un prompt específico.
/// 
/// Esta función es utilizada en el bucle principal del juego
/// para capturar las acciones del usuario (1, 2, 3, etc.).
/// 
/// # Retorna
/// 
/// * `String` - La entrada del usuario sin espacios en blanco al inicio/final
/// 
/// # Panics
/// 
/// Esta función entrará en pánico si no se puede leer desde stdin.
pub fn read_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input.trim().to_string()
}

/// Muestra el menú principal del juego.
/// 
/// # Argumentos
/// 
/// * `options` - Array de opciones a mostrar en el menú
/// 
/// # Formato de salida
/// 
/// ```text
/// Opciones:
/// 1. [opción 1]
/// 2. [opción 2]
/// 3. [opción 3]
/// ```
pub fn print_menu(options: [&str; 3]) {
    println!("\nOpciones:");
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
}
