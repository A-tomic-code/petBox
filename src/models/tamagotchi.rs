//! # Tamagotchi Module
//!
//! This module contains the main structure `Tamagotchi` and all the logic
//! related to the behavior of the virtual pet.

use super::super::constants::{
    HAPPINESS_DECREASE, HUNGER_DECREASE_BIG, HUNGER_DECREASE_SMALL, HUNGER_INCREASE_BIG,
    HUNGER_INCREASE_SMALL, HUNGER_WARNING, INFO_COLOR, INITIAL_HAPPINESS, INITIAL_HUNGER,
    PLAY_HAPPINESS_INCREASE,
};

use super::super::utils;

use std::time::Duration;

use super::notification::{Notification, NotificationLevel};

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
        self.notifications.push(Notification::new(
            format!("{} is playing!", self.name),
            NotificationLevel::Info,
        ));

        self.happiness += PLAY_HAPPINESS_INCREASE;
        if self.hunger < HUNGER_DECREASE_BIG {
            self.hunger += HUNGER_INCREASE_BIG;
        }
    }

    pub fn feed(&mut self) {
        self.notifications.push(Notification::new(
            format!("{} is eating!", self.name),
            NotificationLevel::Info,
        ));

        self.hunger = if self.hunger >= HUNGER_INCREASE_BIG {
            self.hunger - HUNGER_DECREASE_BIG
        } else {
            0
        };
    }

    pub fn tick(&mut self) {
        let hungry = self.hunger >= 20;

        if hungry {
            // Buscar si ya existe la notificación de hambre
            if let Some(existing) = self
                .notifications
                .iter_mut()
                .find(|n| n.message.contains("is hungry!"))
            {
                // Ya existe, solo actualizamos el timestamp
                existing.timestamp = std::time::Instant::now();
            } else {
                // No existe, la creamos
                self.notifications.push(Notification::new(
                    format!("{} is hungry!", self.name),
                    NotificationLevel::Warning,
                ));

                // Penalización por hambre
                self.happiness = self.happiness.saturating_sub(HAPPINESS_DECREASE);
            }
        }

        // Siempre aumenta el hambre un poco
        self.hunger += HUNGER_INCREASE_SMALL;
    }

    pub fn print_state(&self) {
        utils::print_line("--------------------------------");
        utils::print_line(format!("Status of {}", self.name).as_str());
        utils::print_line("--------------------------------");
        utils::print_line(format!("Happiness: {}", self.happiness).as_str());
        utils::print_line(format!("Hunger: {}", self.hunger).as_str());
    }

    pub fn print_notifications(&mut self) {

        self.notifications
            .retain(|n| n.timestamp.elapsed() < Duration::from_secs(5) && n.removable);

        // Imprime las que quedan
        for notification in &self.notifications {
            match notification.level {
                NotificationLevel::Info => utils::print_info(notification.message.as_str()),
                NotificationLevel::Warning => utils::print_warning(notification.message.as_str()),
                NotificationLevel::Error => utils::print_error(notification.message.as_str()),
            }
        }
    }
}
