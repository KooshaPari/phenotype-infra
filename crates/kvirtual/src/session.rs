//! Session management for KVirtualDesktop

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct SessionManager {
    sessions: Mutex<HashMap<String, Session>>,
    session_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub created_at: String,
    pub data: serde_json::Value,
}

impl SessionManager {
    pub fn new(session_dir: &PathBuf) -> std::io::Result<Self> {
        std::fs::create_dir_all(session_dir)?;
        Ok(Self {
            sessions: Mutex::new(HashMap::new()),
            session_dir: session_dir.clone(),
        })
    }

    pub fn list(&self) -> Vec<String> {
        self.sessions.lock().unwrap().keys().cloned().collect()
    }

    pub fn create(&self, name: &str) {
        let session = Session {
            name: name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            data: serde_json::Value::Object(serde_json::Map::new()),
        };
        self.sessions
            .lock()
            .unwrap()
            .insert(name.to_string(), session);
    }

    pub fn get(&self, name: &str) -> Option<Session> {
        self.sessions.lock().unwrap().get(name).cloned()
    }

    pub fn delete(&self, name: &str) {
        self.sessions.lock().unwrap().remove(name);
    }
}
