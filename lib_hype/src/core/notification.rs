use notify_rust::Notification as Not;

pub struct Notification;

impl Notification {
    pub fn send(title: &str, message: &str, urgency: Urgency) {
        let _ = Not::new()
            .summary(title)
            .body(message)
            .urgency(urgency.into())
            .show();
    }
}

#[derive(Debug, Clone)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl From<Urgency> for notify_rust::Urgency {
    fn from(value: Urgency) -> Self {
        match value {
            Urgency::Low => notify_rust::Urgency::Low,
            Urgency::Normal => notify_rust::Urgency::Normal,
            Urgency::Critical => notify_rust::Urgency::Critical,
        }
    }
}