use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        Uuid::parse_str(s)
            .map(EntityId)
            .map_err(|e| e.to_string())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
