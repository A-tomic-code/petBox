//! # Constants Module
//!
//! This module contains all the game constants that define
//! the behavior and balance of the Tamagotchi.
//!
//! ## Categories of Constants
//!
//! - **Hunger**: Values related to the hunger system
//! - **Happiness**: Values related to the happiness system  
//! - **Interface**: Options for the main menu

use crossterm::style::Color;

// === HUNGER CONSTANTS ===

/// Initial hunger level when creating a new Tamagotchi.
/// A value of 0 means it has no hunger at the start.
pub const INITIAL_HUNGER: u8 = 0;
/// Amount by which hunger decreases when feeding the Tamagotchi.
pub const HUNGER_DECREASE_BIG: u8 = 10;

/// Smaller amount of hunger decrease (not currently used).
pub const HUNGER_DECREASE_SMALL: u8 = 5;

/// Amount by which hunger can increase when playing.
pub const HUNGER_INCREASE_BIG: u8 = 5;

/// Amount by which hunger increases each second (tick).
pub const HUNGER_INCREASE_SMALL: u8 = 1;

/// Theoretical maximum hunger level.
pub const MAX_HUNGER: u8 = 100;

/// Hunger threshold that activates alerts and decreases happiness.
/// When hunger exceeds this value, the Tamagotchi will be "hungry".
pub const HUNGER_WARNING: u8 = 80;

// === HAPPINESS CONSTANTS ===

/// Initial happiness level when creating a new Tamagotchi.
/// A value of 100 represents maximum happiness.
pub const INITIAL_HAPPINESS: u8 = 100;

/// Amount by which happiness decreases when the Tamagotchi is hungry.
/// This decrease occurs every second while hunger > HUNGER_WARNING.
pub const HAPPINESS_DECREASE: u8 = 5;

/// Amount by which happiness increases when playing with the Tamagotchi.
pub const PLAY_HAPPINESS_INCREASE: u8 = 10;

// === INTERFACE CONSTANTS ===

/// Available options in the main menu of the game.
///
/// - **Play**: Increases the Tamagotchi's happiness
/// - **Feed**: Reduces the Tamagotchi's hunger
/// - **Exit**: Ends the game
pub const MAIN_MENU_OPTIONS: [&str; 3] = ["Play", "Feed", "Exit"];

pub const DANGER_COLOR: Color = Color::DarkRed;
pub const WARNING_COLOR: Color = Color::Yellow;
pub const NORMAL_COLOR: Color = Color::Green;