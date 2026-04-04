use chrono::{DateTime, Utc};

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
}
