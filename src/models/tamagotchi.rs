//! # Módulo Tamagotchi
//! 
//! Este módulo contiene la estructura principal `Tamagotchi` y toda la lógica
//! relacionada con el comportamiento de la mascota virtual.

use super::super::constants::{
    HAPPINESS_DECREASE, HUNGER_DECREASE_BIG, HUNGER_DECREASE_SMALL, HUNGER_INCREASE_BIG,
    HUNGER_INCREASE_SMALL, HUNGER_WARNING, INITIAL_HAPPINESS, INITIAL_HUNGER,
    PLAY_HAPPINESS_INCREASE,
};

use std::time::{SystemTime, UNIX_EPOCH};

/// Estructura que representa una mascota virtual Tamagotchi.
/// 
/// # Campos
/// 
/// * `name` - El nombre de la mascota asignado por el usuario
/// * `happiness` - Nivel de felicidad (0-100+)
/// * `hunger` - Nivel de hambre (0-100+, valores más altos = más hambre)
/// 
/// # Ejemplo
/// 
/// ```
/// let mut pet = Tamagotchi::new("Buddy".to_string());
/// pet.play();    // Aumenta felicidad
/// pet.feed();    // Reduce hambre
/// pet.tick();    // Actualiza estado por paso de tiempo
/// ```
#[derive(Clone)]
pub struct Tamagotchi {
    pub name: String,
    pub happiness: u32,
    pub hunger: u32,
}

impl Tamagotchi {
    /// Crea un nuevo Tamagotchi con valores iniciales.
    /// 
    /// # Argumentos
    /// 
    /// * `name` - El nombre que el usuario quiere darle a su mascota
    /// 
    /// # Retorna
    /// 
    /// * `Self` - Una nueva instancia de Tamagotchi con felicidad máxima y hambre mínima
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            happiness: INITIAL_HAPPINESS,
            hunger: INITIAL_HUNGER,
        }
    }

    /// Jugar con el Tamagotchi aumenta su felicidad.
    /// 
    /// Esta acción:
    /// - Aumenta la felicidad en `PLAY_HAPPINESS_INCREASE` puntos
    /// - Puede aumentar ligeramente el hambre si está por debajo del umbral
    /// 
    /// # Efectos secundarios
    /// 
    /// Imprime un mensaje indicando que la mascota está jugando.
    pub fn play(&mut self) {
        println!("{} está jugando!", self.name);

        self.happiness += PLAY_HAPPINESS_INCREASE;
        if self.hunger < HUNGER_DECREASE_BIG {
            self.hunger += HUNGER_INCREASE_BIG;
        }
    }

    /// Alimentar al Tamagotchi reduce su nivel de hambre.
    /// 
    /// Esta acción:
    /// - Reduce el hambre en `HUNGER_DECREASE_BIG` puntos
    /// - El hambre no puede bajar de 0
    /// 
    /// # Efectos secundarios
    /// 
    /// Imprime un mensaje indicando que la mascota está comiendo.
    pub fn feed(&mut self) {
        println!("{} está comiendo!", self.name);

        self.hunger = if self.hunger >= HUNGER_INCREASE_BIG {
            self.hunger - HUNGER_DECREASE_BIG
        } else {
            0
        };
    }

    /// Actualiza el estado del Tamagotchi por el paso del tiempo.
    /// 
    /// Este método se llama automáticamente cada segundo y:
    /// - Aumenta el hambre en `HUNGER_INCREASE_SMALL` 
    /// - Si el hambre supera `HUNGER_WARNING`, reduce la felicidad
    /// - Muestra una alerta si la mascota está hambrienta
    /// 
    /// # Mecánicas
    /// 
    /// - El hambre aumenta constantemente con el tiempo
    /// - Una mascota hambrienta pierde felicidad gradualmente
    pub fn tick(&mut self) {
        if self.hunger > HUNGER_WARNING {
            println!("{} está hambriento!", self.name);
            self.happiness = if self.happiness >= HAPPINESS_DECREASE {
                self.happiness - HAPPINESS_DECREASE
            } else {
                0
            };
        }

        self.hunger += HUNGER_INCREASE_SMALL;
    }

    /// Muestra el estado actual del Tamagotchi en la consola.
    /// 
    /// Imprime de forma formateada:
    /// - El nombre de la mascota
    /// - Su nivel actual de felicidad  
    /// - Su nivel actual de hambre
    /// 
    /// # Formato de salida
    /// 
    /// ```text
    /// Estado de [nombre]
    /// --------------------------------
    /// Felicidad: [valor]
    /// Hambre: [valor]
    /// ```
    pub fn print_state(&self) {
        println!("\n Estado de {}", self.name);
        println!("--------------------------------\n");
        println!("Felicidad: {}", self.happiness);
        println!("Hambre: {}", self.hunger);
    }
}
