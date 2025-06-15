use super::super::constants::{
    HAPPINESS_DECREASE, HUNGER_DECREASE_BIG, HUNGER_DECREASE_SMALL, HUNGER_INCREASE_BIG,
    HUNGER_INCREASE_SMALL, HUNGER_WARNING, INITIAL_HAPPINESS, INITIAL_HUNGER,
    PLAY_HAPPINESS_INCREASE,
};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Tamagotchi {
    pub name: String,
    pub happiness: u32,
    pub hunger: u32,
}

impl Tamagotchi {
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            happiness: INITIAL_HAPPINESS,
            hunger: INITIAL_HUNGER,
        }
    }

    pub fn play(&mut self) {
        println!("{} está jugando!", self.name);

        self.happiness += PLAY_HAPPINESS_INCREASE;
        if self.hunger < HUNGER_DECREASE_BIG {
            self.hunger += HUNGER_INCREASE_BIG;
        }
    }

    pub fn feed(&mut self) {
        println!("{} está comiendo!", self.name);

        self.hunger = if self.hunger >= HUNGER_INCREASE_BIG {
            self.hunger - HUNGER_DECREASE_BIG
        } else {
            0
        };
    }

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

    pub fn print_state(&self) {
        println!("\n Estado de {}", self.name);
        println!("--------------------------------\n");
        println!("Felicidad: {}", self.happiness);
        println!("Hambre: {}", self.hunger);
    }
}
