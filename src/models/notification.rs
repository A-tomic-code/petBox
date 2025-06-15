use std::time::Instant;

#[derive(Clone)]
pub struct Notification {
    pub message: String,
    pub timestamp: Instant,
    pub level: NotificationLevel,
    pub removable: bool,
}

#[derive(Clone)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

impl Notification {
    pub fn new(message: String, level: NotificationLevel) -> Self {
        Notification {
            message,
            timestamp: Instant::now(),
            level,
            removable: true,
        }
    }
}
