//! # Módulo de Constantes
//! 
//! Este módulo contiene todas las constantes del juego que definen
//! el comportamiento y balance del Tamagotchi.
//! 
//! ## Categorías de Constantes
//! 
//! - **Hambre**: Valores relacionados con el sistema de hambre
//! - **Felicidad**: Valores relacionados con el sistema de felicidad  
//! - **Interfaz**: Opciones del menú principal

// === CONSTANTES DE HAMBRE ===

/// Nivel inicial de hambre al crear un nuevo Tamagotchi.
/// Un valor de 0 significa que no tiene hambre al comenzar.
pub const INITIAL_HUNGER: u32 = 0;
/// Cantidad en que disminuye el hambre al alimentar al Tamagotchi.
pub const HUNGER_DECREASE_BIG: u32 = 10;

/// Cantidad menor de disminución de hambre (no utilizada actualmente).
pub const HUNGER_DECREASE_SMALL: u32 = 5;

/// Cantidad en que puede aumentar el hambre al jugar.
pub const HUNGER_INCREASE_BIG: u32 = 5;

/// Cantidad en que aumenta el hambre cada segundo (tick).
pub const HUNGER_INCREASE_SMALL: u32 = 1;

/// Nivel máximo teórico de hambre.
pub const MAX_HUNGER: u32 = 100;

/// Umbral de hambre que activa las alertas y reduce la felicidad.
/// Cuando el hambre supera este valor, el Tamagotchi estará "hambriento".
pub const HUNGER_WARNING: u32 = 80;

// === CONSTANTES DE FELICIDAD ===

/// Nivel inicial de felicidad al crear un nuevo Tamagotchi.
/// Un valor de 100 representa felicidad máxima.
pub const INITIAL_HAPPINESS: u32 = 100;

/// Cantidad en que disminuye la felicidad cuando el Tamagotchi está hambriento.
/// Esta disminución ocurre cada segundo mientras el hambre > HUNGER_WARNING.
pub const HAPPINESS_DECREASE: u32 = 5;

/// Cantidad en que aumenta la felicidad al jugar con el Tamagotchi.
pub const PLAY_HAPPINESS_INCREASE: u32 = 10;

// === CONSTANTES DE INTERFAZ ===

/// Opciones disponibles en el menú principal del juego.
/// 
/// - **Jugar**: Aumenta la felicidad del Tamagotchi
/// - **Alimentar**: Reduce el hambre del Tamagotchi
/// - **Salir**: Termina el juego
pub const MAIN_MENU_OPTIONS: [&str; 3] = ["Jugar", "Alimentar", "Salir"];
