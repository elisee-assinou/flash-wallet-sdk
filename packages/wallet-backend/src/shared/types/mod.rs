use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
