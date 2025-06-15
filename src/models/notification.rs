use std::time::Instant;

#[derive(Clone)]
pub struct Notification {
    pub message: String,
    pub timestamp: Instant,
    pub level: NotificationLevel,
    pub removable: bool,
    pub key: Option<String>,
}

#[derive(Clone)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

impl Notification {
    pub fn new(message: String, level: NotificationLevel) -> Self {
        Self {
            message,
            level,
            timestamp: Instant::now(),
            removable: true,
            key: None,
        }
    }

    pub fn with_key(message: String, level: NotificationLevel, key: &str) -> Self {
        Self {
            message,
            level,
            timestamp: Instant::now(),
            removable: true,
            key: Some(key.to_string()),
        }
    }
}

pub const NOTIFICATION_HUNGER_WARNING: &str = "hunger_warning";
pub const NOTIFICATION_EATING_INFO: &str = "eating_info";
pub const NOTIFICATION_PLAYING_INFO: &str = "playing_info";
pub const NOTIFICATION_HEALTH_WARNING: &str = "health_warning";
pub const NOTIFICATION_HAPPINESS_WARNING: &str = "happiness_warning";
