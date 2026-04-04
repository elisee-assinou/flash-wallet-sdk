use chrono::{DateTime, Utc};

/// Trait commun à tous les événements domaine
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
}
