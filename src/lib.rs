// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-3d-renderer: 3d renderer component model.
//!
//! Real implementation: renderers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer {
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
pub enum ThreeDRendererError {
    EmptyRendererName,
    DuplicateRenderer(String),
    EmptyEventSubject,
}

impl fmt::Display for ThreeDRendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRendererName => write!(f, "renderer name must not be empty"),
            Self::DuplicateRenderer(id) => write!(f, "duplicate renderer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ThreeDRendererError {}

pub struct ThreeDRenderer {
    pub name: String,
    renderers: BTreeMap<String, Renderer>,
    events: BTreeMap<String, Event>,
}

impl ThreeDRenderer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            renderers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_renderer(&mut self, item: Renderer) -> Result<(), ThreeDRendererError> {
        if item.name.trim().is_empty() {
            return Err(ThreeDRendererError::EmptyRendererName);
        }
        let id = item.id.clone();
        if self.renderers.contains_key(&id) {
            return Err(ThreeDRendererError::DuplicateRenderer(id.clone()));
        }
        self.renderers.insert(id, item);
        Ok(())
    }

    pub fn renderer_count(&self) -> usize {
        self.renderers.len()
    }

    pub fn get_renderer(&self, id: &str) -> Option<&Renderer> {
        self.renderers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ThreeDRendererError> {
        if event.subject.trim().is_empty() {
            return Err(ThreeDRendererError::EmptyEventSubject);
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

    fn sample_renderer(id: &str, name: &str) -> Renderer {
        Renderer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ThreeDRenderer::new("ThreeDRenderer");
        assert_eq!(m.name, "ThreeDRenderer");
        assert_eq!(m.renderer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_renderer() {
        let mut m = ThreeDRenderer::new("ThreeDRenderer");
        m.add_renderer(sample_renderer("a1", "Alpha")).unwrap();
        assert_eq!(m.renderer_count(), 1);
        assert!(m.get_renderer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ThreeDRenderer::new("ThreeDRenderer");
        assert!(m.add_renderer(sample_renderer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ThreeDRenderer::new("ThreeDRenderer");
        m.add_renderer(sample_renderer("x", "One")).unwrap();
        assert!(m.add_renderer(sample_renderer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ThreeDRenderer::new("ThreeDRenderer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
