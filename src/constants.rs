use crossterm::style::Color;

pub const INITIAL_HUNGER: u8 = 0;
pub const HUNGER_DECREASE_BIG: u8 = 10;
pub const HUNGER_DECREASE_SMALL: u8 = 5;
pub const HUNGER_INCREASE_BIG: u8 = 5;
pub const HUNGER_INCREASE_SMALL: u8 = 1;
pub const HUNGER_WARNING: u8 = 80;
pub const MAX_HUNGER: u8 = 100;

pub const INITIAL_HAPPINESS: u8 = 100;
pub const HAPPINESS_DECREASE: u8 = 5;
pub const HAPPINESS_INCREASE: u8 = 5;
pub const PLAY_HAPPINESS_INCREASE: u8 = 10;
pub const HAPPINESS_WARNING: u8 = 20;
pub const MAX_HAPPINESS: u8 = 100;

pub const INITIAL_HEALTH: u8 = 100;
pub const HEALTH_DECREASE: u8 = 5;
pub const HEALTH_INCREASE: u8 = 10;
pub const HEALTH_WARNING: u8 = 20;
pub const MAX_HEALTH: u8 = 100;

pub const MAIN_MENU_OPTIONS: [&str; 3] = ["Play", "Feed", "Exit"];

pub const DANGER_COLOR: Color = Color::DarkRed;
pub const WARNING_COLOR: Color = Color::Yellow;
pub const INFO_COLOR: Color = Color::Cyan;
pub const NORMAL_COLOR: Color = Color::Green;

pub const NOTIFICATION_DURATION: u64 = 2;
