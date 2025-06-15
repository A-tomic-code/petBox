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
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Notification {
    pub message: String,
    pub timestamp: Instant,
}

impl Notification {
    pub fn new(message: String) -> Self {
        Notification {
            message,
            timestamp: Instant::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() >= Duration::from_secs(5)
    }
}

#[derive(Clone)]
pub struct Tamagotchi {
    pub name: String,
    pub happiness: u8,
    pub hunger: u8,
    pub notifications: Vec<Notification>,
}

impl Tamagotchi {
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            happiness: INITIAL_HAPPINESS,
            hunger: INITIAL_HUNGER,
            notifications: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        self.notifications.push(Notification::new(format!("{} is playing!", self.name)));

        self.happiness += PLAY_HAPPINESS_INCREASE;
        if self.hunger < HUNGER_DECREASE_BIG {
            self.hunger += HUNGER_INCREASE_BIG;
        }
    }

    pub fn feed(&mut self) {
        self.notifications.push(Notification::new(format!("{} is eating!", self.name)));

        self.hunger = if self.hunger >= HUNGER_INCREASE_BIG {
            self.hunger - HUNGER_DECREASE_BIG
        } else {
            0
        };
    }

    pub fn tick(&mut self) {
        // Remove expired notifications (older than 5 seconds)
        self.notifications.retain(|notification| !notification.is_expired());
        
        // if self.hunger > HUNGER_WARNING {
        if self.hunger > 2 {
            self.notifications
                .push(Notification::new("Your Tamagotchi is hungry!".to_string()));
            self.happiness = if self.happiness >= HAPPINESS_DECREASE {
                self.happiness - HAPPINESS_DECREASE
            } else {
                0
            };
        }

        self.hunger += HUNGER_INCREASE_SMALL;
    }

    pub fn print_state(&self) {
        print_line("--------------------------------");
        print_line(format!("Status of {}", self.name).as_str());
        print_line("--------------------------------");
        print_line(format!("Happiness: {}", self.happiness).as_str());
        print_line(format!("Hunger: {}", self.hunger).as_str());
    }

    pub fn clean_expired_notifications(&mut self) {
        self.notifications.retain(|notification| !notification.is_expired());
    }

    pub fn print_notifications(&mut self) {
        for notification in self.notifications.iter() {
            print_line(&format!("⚠️ {}", notification.message));
        }
    }
}
