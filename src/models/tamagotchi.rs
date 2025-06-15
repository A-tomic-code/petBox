//! # Tamagotchi Module
//!
//! This module contains the main structure `Tamagotchi` and all the logic
//! related to the behavior of the virtual pet.

use super::super::constants::{
    HAPPINESS_DECREASE, HUNGER_DECREASE_BIG, HUNGER_DECREASE_SMALL, HUNGER_INCREASE_BIG,
    HUNGER_INCREASE_SMALL, HUNGER_WARNING, INITIAL_HAPPINESS, INITIAL_HUNGER,
    PLAY_HAPPINESS_INCREASE,
};

use super::super::utils::{print_line, print_warning};

use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

use std::io::stdout;

#[derive(Clone)]
pub struct Tamagotchi {
    pub name: String,
    pub happiness: u8,
    pub hunger: u8,
}

impl Tamagotchi {
    /// Creates a new Tamagotchi with initial values.
    ///
    /// # Arguments
    ///
    /// * `name` - The name the user wants to give their pet
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of Tamagotchi with maximum happiness and minimum hunger
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            happiness: INITIAL_HAPPINESS,
            hunger: INITIAL_HUNGER,
        }
    }

    /// Playing with the Tamagotchi increases its happiness.
    ///
    /// This action:
    /// - Increases happiness by `PLAY_HAPPINESS_INCREASE` points
    /// - Can slightly increase hunger if below the threshold
    ///
    /// # Side Effects
    ///
    /// Prints a message indicating the pet is playing.
    pub fn play(&mut self) {
        println!("{} is playing!", self.name);

        self.happiness += PLAY_HAPPINESS_INCREASE;
        if self.hunger < HUNGER_DECREASE_BIG {
            self.hunger += HUNGER_INCREASE_BIG;
        }
    }

    /// Feeding the Tamagotchi reduces its hunger level.
    ///
    /// This action:
    /// - Reduces hunger by `HUNGER_DECREASE_BIG` points
    /// - Hunger cannot go below 0
    ///
    /// # Side Effects
    ///
    /// Prints a message indicating the pet is eating.
    pub fn feed(&mut self) {
        println!("{} is eating!", self.name);

        self.hunger = if self.hunger >= HUNGER_INCREASE_BIG {
            self.hunger - HUNGER_DECREASE_BIG
        } else {
            0
        };
    }

    /// Updates the state of the Tamagotchi over time.
    ///
    /// This method is called automatically every second and:
    /// - Increases hunger by `HUNGER_INCREASE_SMALL`
    /// - If hunger exceeds `HUNGER_WARNING`, happiness decreases
    /// - Shows an alert if the pet is hungry
    ///
    /// # Mechanics
    ///
    /// - Hunger continuously increases over time
    /// - A hungry pet gradually loses happiness
    pub fn tick(&mut self) {
        if self.hunger > HUNGER_WARNING {
            // Show alert if the pet is hungry
            print_warning(format!("{} is hungry!", self.name).as_str());
            self.happiness = if self.happiness >= HAPPINESS_DECREASE {
                self.happiness - HAPPINESS_DECREASE
            } else {
                0
            };
        }

        self.hunger += HUNGER_INCREASE_SMALL;
    }

    /// Displays the current state of the Tamagotchi in the console.
    ///
    /// It prints formatted:
    /// - The name of the pet
    /// - Its current happiness level  
    /// - Its current hunger level
    ///
    /// # Output Format
    ///
    /// ```text
    /// Status of [name]
    /// --------------------------------
    /// Happiness: [value]
    /// Hunger: [value]
    /// ```
    pub fn print_state(&self) {
        print_line(format!("Status of {}", self.name).as_str());

        print_line("--------------------------------");
        print_line(format!("Happiness: {}", self.happiness).as_str());
        print_line(format!("Hunger: {}", self.hunger).as_str());
    }
}
