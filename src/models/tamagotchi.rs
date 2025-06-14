use super::super::constants::{
    HUNGER_DECREASE, HUNGER_INCREASE, INITIAL_HAPPINESS, INITIAL_HUNGER, PLAY_HAPPINESS_INCREASE,
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
        if self.hunger < HUNGER_DECREASE {
            self.hunger += HUNGER_INCREASE;
        }
    }

    pub fn feed(&mut self) {
        println!("{} está comiendo!", self.name);

        self.hunger = if self.hunger >= HUNGER_INCREASE {
            self.hunger - HUNGER_INCREASE
        } else {
            0
        };
    }

    pub fn tick(&mut self) {
        let now = SystemTime::now();
        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Tiempo anterior a la época Unix");
        println!(
            "Tick ! {} está pasando el tiempo...",
            duration_since_epoch.as_secs()
        );

        if self.hunger < HUNGER_INCREASE {
            self.happiness = if self.happiness >= HUNGER_INCREASE {
                self.happiness - HUNGER_INCREASE
            } else {
                0
            };
        }
        self.hunger += HUNGER_INCREASE;
    }
}
