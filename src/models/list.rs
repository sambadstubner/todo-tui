use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: u32,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl List {
    pub fn new(id: u32, name: String) -> Self {
        let now = Local::now();
        Self {
            id,
            name,
            color: None,
            created_at: now,
            updated_at: now,
        }
    }

}

impl Default for List {
    fn default() -> Self {
        Self::new(0, "Default List".to_string())
    }
}
