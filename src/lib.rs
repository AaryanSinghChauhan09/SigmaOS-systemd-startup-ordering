// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-startup-ordering: systemd startup ordering component model.
//!
//! Real implementation: orderings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ordering {
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
pub enum SystemdStartupOrderingError {
    EmptyOrderingName,
    DuplicateOrdering(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdStartupOrderingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOrderingName => write!(f, "ordering name must not be empty"),
            Self::DuplicateOrdering(id) => write!(f, "duplicate ordering {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdStartupOrderingError {}

pub struct SystemdStartupOrdering {
    pub name: String,
    orderings: BTreeMap<String, Ordering>,
    events: BTreeMap<String, Event>,
}

impl SystemdStartupOrdering {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            orderings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ordering(&mut self, item: Ordering) -> Result<(), SystemdStartupOrderingError> {
        if item.name.trim().is_empty() {
            return Err(SystemdStartupOrderingError::EmptyOrderingName);
        }
        let id = item.id.clone();
        if self.orderings.contains_key(&id) {
            return Err(SystemdStartupOrderingError::DuplicateOrdering(id.clone()));
        }
        self.orderings.insert(id, item);
        Ok(())
    }

    pub fn ordering_count(&self) -> usize {
        self.orderings.len()
    }

    pub fn get_ordering(&self, id: &str) -> Option<&Ordering> {
        self.orderings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdStartupOrderingError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdStartupOrderingError::EmptyEventSubject);
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

    fn sample_ordering(id: &str, name: &str) -> Ordering {
        Ordering {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdStartupOrdering::new("SystemdStartupOrdering");
        assert_eq!(m.name, "SystemdStartupOrdering");
        assert_eq!(m.ordering_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ordering() {
        let mut m = SystemdStartupOrdering::new("SystemdStartupOrdering");
        m.add_ordering(sample_ordering("a1", "Alpha")).unwrap();
        assert_eq!(m.ordering_count(), 1);
        assert!(m.get_ordering("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdStartupOrdering::new("SystemdStartupOrdering");
        assert!(m.add_ordering(sample_ordering("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdStartupOrdering::new("SystemdStartupOrdering");
        m.add_ordering(sample_ordering("x", "One")).unwrap();
        assert!(m.add_ordering(sample_ordering("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdStartupOrdering::new("SystemdStartupOrdering");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
