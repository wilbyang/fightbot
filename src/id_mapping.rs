use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct IdMapping {
    mappings: Mutex<HashMap<String, String>>,
}

impl IdMapping {
    pub fn new() -> Self {
        Self {
            mappings: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_or_create_mapping(&self, original_id: &str) -> String {
        let mut mappings = self.mappings.lock().unwrap();
        
        if let Some(mapped_id) = mappings.get(original_id) {
            mapped_id.clone()
        } else {
            let new_id = format!("id_{}", Uuid::new_v4().simple());
            mappings.insert(original_id.to_string(), new_id.clone());
            new_id
        }
    }
} 