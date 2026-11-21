// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-tools: system tools component model.
//!
//! Real implementation: toolss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
/// System utilities collection module.
pub mod sigmasystemtools;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tools {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemToolsError {
    EmptyToolsName,
    DuplicateTools(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemToolsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyToolsName => write!(f, "tools name must not be empty"),
            Self::DuplicateTools(id) => write!(f, "duplicate tools {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemToolsError {}

pub struct SystemTools {
    pub name: String,
    toolss: BTreeMap<String, Tools>,
    events: BTreeMap<String, Event>,
}

impl SystemTools {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            toolss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_tools(&mut self, item: Tools) -> Result<(), SystemToolsError> {
        if item.name.trim().is_empty() {
            return Err(SystemToolsError::EmptyToolsName);
        }
        let id = item.id.clone();
        if self.toolss.contains_key(&id) {
            return Err(SystemToolsError::DuplicateTools(id.clone()));
        }
        self.toolss.insert(id, item);
        Ok(())
    }

    pub fn tools_count(&self) -> usize {
        self.toolss.len()
    }

    pub fn get_tools(&self, id: &str) -> Option<&Tools> {
        self.toolss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemToolsError> {
        if event.subject.trim().is_empty() {
            return Err(SystemToolsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tools(id: &str, name: &str) -> Tools {
        Tools {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemTools::new("SystemTools");
        assert_eq!(m.name, "SystemTools");
        assert_eq!(m.tools_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_tools() {
        let mut m = SystemTools::new("SystemTools");
        m.add_tools(sample_tools("a1", "Alpha")).unwrap();
        assert_eq!(m.tools_count(), 1);
        assert!(m.get_tools("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemTools::new("SystemTools");
        assert!(m.add_tools(sample_tools("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemTools::new("SystemTools");
        m.add_tools(sample_tools("x", "One")).unwrap();
        assert!(m.add_tools(sample_tools("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemTools::new("SystemTools");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
