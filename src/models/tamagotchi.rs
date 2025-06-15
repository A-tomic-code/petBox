//! # Tamagotchi Module
//!
//! This module contains the main structure `Tamagotchi` and all the logic
//! related to the behavior of the virtual pet.

use super::super::constants::{
    HAPPINESS_DECREASE, HAPPINESS_INCREASE, HAPPINESS_WARNING, HEALTH_DECREASE, HEALTH_INCREASE,
    HEALTH_WARNING, HUNGER_DECREASE_BIG, HUNGER_INCREASE_BIG, HUNGER_INCREASE_SMALL,
    HUNGER_WARNING, INITIAL_HAPPINESS, INITIAL_HEALTH, INITIAL_HUNGER, MAX_HAPPINESS, MAX_HEALTH,
    MAX_HUNGER, NOTIFICATION_DURATION,
};

use super::super::utils;

use std::time::{Duration, Instant};

use super::notification::{
    Notification, NotificationLevel, NOTIFICATION_EATING_INFO, NOTIFICATION_HAPPINESS_WARNING,
    NOTIFICATION_HEALTH_WARNING, NOTIFICATION_HUNGER_WARNING, NOTIFICATION_PLAYING_INFO,
};

#[derive(Clone)]
pub struct Tamagotchi {
    pub name: String,
    pub health: u8,
    pub happiness: u8,
    pub hunger: u8,
    pub notifications: Vec<Notification>,
}

impl Tamagotchi {
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            health: INITIAL_HEALTH,
            happiness: INITIAL_HAPPINESS,
            hunger: INITIAL_HUNGER,
            notifications: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        let notification = Notification::with_key(
            format!("{} is playing!", self.name),
            NotificationLevel::Info,
            NOTIFICATION_PLAYING_INFO,
        );

        self.add_notification(notification);

        self.add_happiness(HAPPINESS_INCREASE);
        self.add_hunger(HUNGER_INCREASE_BIG);
    }

    pub fn feed(&mut self) {
        let notification = Notification::with_key(
            format!("{} is eating!", self.name),
            NotificationLevel::Info,
            NOTIFICATION_EATING_INFO,
        );

        self.add_notification(notification);
        self.sub_hunger(HUNGER_DECREASE_BIG);
    }

    pub fn tick(&mut self) {
        let hungry = self.hunger >= HUNGER_WARNING;
        let sad = self.happiness <= HAPPINESS_WARNING;
        let sick = self.health <= HEALTH_WARNING;

        if hungry {
            let notification = Notification::with_key(
                format!("{} is hungry!", self.name),
                NotificationLevel::Warning,
                NOTIFICATION_HUNGER_WARNING,
            );

            self.add_notification(notification);
            self.sub_happiness(HAPPINESS_DECREASE);
            self.sub_health(HEALTH_INCREASE);
        }

        if sad {
            let notification = Notification::with_key(
                format!("{} is sad!", self.name),
                NotificationLevel::Warning,
                NOTIFICATION_HAPPINESS_WARNING,
            );
            self.add_notification(notification);

            self.sub_health(HAPPINESS_DECREASE);
        }

        if sick {
            let notification = Notification::with_key(
                format!("{} is sick!", self.name),
                NotificationLevel::Warning,
                NOTIFICATION_HEALTH_WARNING,
            );
            self.add_notification(notification);

            self.sub_happiness(HAPPINESS_DECREASE);
        }

        if !hungry && !sad {
            self.add_health(HAPPINESS_INCREASE);
        }

        // Siempre aumenta el hambre un poco
        self.add_hunger(HUNGER_INCREASE_SMALL);
    }

    pub fn add_notification(&mut self, notification: Notification) {
        if let Some(existing) = self
            .notifications
            .iter_mut()
            .find(|n| n.key == notification.key)
        {
            existing.timestamp = Instant::now();
        } else {
            self.notifications.push(notification);
        }
    }

    pub fn print_state(&self) {
        utils::print_line("--------------------------------");
        utils::print_line(format!("Status of {}", self.name).as_str());
        utils::print_line("--------------------------------");
        utils::print_line(format!("Happiness: {}", self.happiness).as_str());
        utils::print_line(format!("Hunger: {}", self.hunger).as_str());
        utils::print_line(format!("Health: {}", self.health).as_str());
    }

    pub fn print_notifications(&mut self) {
        self.notifications.retain(|n| {
            n.timestamp.elapsed() < Duration::from_secs(NOTIFICATION_DURATION) && n.removable
        });

        // Imprime las que quedan
        for notification in &self.notifications {
            match notification.level {
                NotificationLevel::Info => utils::print_info(notification.message.as_str()),
                NotificationLevel::Warning => utils::print_warning(notification.message.as_str()),
                NotificationLevel::Error => utils::print_error(notification.message.as_str()),
            }
        }
    }

    fn add_hunger(&mut self, amount: u8) {
        self.hunger = Self::add_stat(self.hunger, amount, MAX_HUNGER);
    }
    fn sub_hunger(&mut self, amount: u8) {
        self.hunger = Self::sub_stat(self.hunger, amount, 0);
    }

    fn add_happiness(&mut self, amount: u8) {
        self.happiness = Self::add_stat(self.happiness, amount, MAX_HAPPINESS);
    }

    fn sub_happiness(&mut self, amount: u8) {
        self.happiness = Self::sub_stat(self.happiness, amount, 0);
    }

    fn add_health(&mut self, amount: u8) {
        self.health = Self::add_stat(self.health, amount, MAX_HEALTH);
    }

    fn sub_health(&mut self, amount: u8) {
        self.health = Self::sub_stat(self.health, amount, 0);
    }

    fn add_stat(current: u8, amount: u8, max: u8) -> u8 {
        (current.saturating_add(amount)).min(max)
    }

    fn sub_stat(current: u8, amount: u8, min: u8) -> u8 {
        (current.saturating_sub(amount)).max(min)
    }
}
