#![allow(non_snake_case)]

#[macro_use]
extern crate derive_builder;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event structure for game-events.io
#[derive(Serialize, Deserialize, Clone, Debug, Builder, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct WhalyticsEvent {
    /// Event name (e.g., "level_completed", "purchase")
    pub event: String,

    /// Unique user identifier
    pub user_id: String,

    /// Session identifier
    pub session_id: String,

    /// Unix timestamp in seconds
    #[builder(default = "self.default_time()")]
    pub time: u64,

    /// Event-specific properties
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub event_properties: HashMap<String, serde_json::Value>,

    /// User properties (will be merged with existing user data)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub user_properties: HashMap<String, serde_json::Value>,
}

impl WhalyticsEventBuilder {
    fn default_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

use uuid::Uuid;

/// Session structure that holds common properties for events
#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct WhalyticsSession {
    /// Unique user identifier
    #[builder(default = "Uuid::new_v4().to_string()")]
    user_id: String,

    /// Session identifier
    #[builder(default = "Uuid::new_v4().to_string()")]
    session_id: String,

    /// Events
    #[builder(default)]
    events: Vec<WhalyticsEvent>,

    /// User properties that will be added to all events in this session
    #[builder(default)]
    user_properties: HashMap<String, serde_json::Value>,
}

impl Default for WhalyticsSession {
    fn default() -> Self {
        WhalyticsSessionBuilder::default()
            .build()
            .expect("Failed to create default WhalyticsSession")
    }
}

impl WhalyticsSession {
    /// Create a new session with user_id and session_id
    pub fn new(user_id: impl Into<String>, session_id: impl Into<String>) -> Self {
        WhalyticsSessionBuilder::default()
            .user_id(user_id)
            .session_id(session_id)
            .build()
            .expect("Failed to create WhalyticsSession")
    }

    /// Add an event to the session
    pub fn push_event(
        &mut self,
        event: impl Into<String>,
        event_properties: HashMap<String, serde_json::Value>,
    ) {
        // Determine user_id: check properties first, then session
        let user_id = if let Some(uid) = event_properties.get("user_id").and_then(|v| v.as_str()) {
            uid.to_string()
        } else {
            self.user_id.clone()
        };

        // Determine session_id: check properties first, then session
        let session_id =
            if let Some(sid) = event_properties.get("session_id").and_then(|v| v.as_str()) {
                sid.to_string()
            } else {
                self.session_id.clone()
            };

        // Create the event
        let event = WhalyticsEventBuilder::default()
            .event(event)
            .user_id(user_id)
            .session_id(session_id)
            .user_properties(self.user_properties.clone())
            .event_properties(event_properties)
            .build()
            .expect("Failed to build event");

        self.events.push(event);
    }

    /// Add or update a user property for this session
    pub fn set_user_property(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.user_properties.insert(key.into(), value);
    }

    /// Set new user properties
    pub fn set_user_properties(&mut self, user_properties: HashMap<String, serde_json::Value>) {
        self.user_properties = user_properties;
    }

    /// Get the user_id for this session
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get the session_id for this session
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get all user properties for this session
    pub fn user_properties(&self) -> &HashMap<String, serde_json::Value> {
        &self.user_properties
    }

    /// Take all events from this session
    pub fn take_events(&mut self, max_count: usize) -> Vec<WhalyticsEvent> {
        let count = std::cmp::min(self.events.len(), max_count);
        self.events.drain(0..count).collect()
    }
}

/// game-events.io SDK client
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct WhalyticsClient {
    /// API key for authentication
    api_key: String,

    /// Backend URL (default: https://api.game-events.io/v1/events)
    #[builder(default = "\"https://api.game-events.io/v1/events\".to_string()")]
    backend_url: String,

    /// HTTP client for making requests
    #[builder(setter(skip))]
    #[builder(
        default = "reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).build().unwrap()"
    )]
    client: reqwest::blocking::Client,

    /// Buffered events waiting to be sent
    #[builder(setter(skip))]
    #[builder(default)]
    events: Vec<WhalyticsEvent>,
}

impl WhalyticsClient {
    /// Create a new Whalytics client
    pub fn new(api_key: impl Into<String>) -> Self {
        WhalyticsClientBuilder::default()
            .api_key(api_key)
            .build()
            .expect("Failed to create WhalyticsClient")
    }

    /// Log an event (adds to buffer)
    pub fn log_event(&mut self, event: WhalyticsEvent) {
        self.events.push(event);
    }

    /// Send all buffered events to the backend
    pub fn flush(&mut self) -> Result<String, reqwest::Error> {
        if self.events.is_empty() {
            return Ok("No events to send".to_string());
        }

        let events_to_send: Vec<WhalyticsEvent> = self.events.drain(..).collect();

        let response = self
            .client
            .post(&self.backend_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&events_to_send)
            .send()?
            .text()?;

        Ok(response)
    }

    /// Send events in batches (useful for large event counts)
    pub fn flush_batch(&mut self, batch_size: usize) -> Result<String, reqwest::Error> {
        if self.events.is_empty() {
            return Ok("No events to send".to_string());
        }

        let events_to_send: Vec<WhalyticsEvent> = if self.events.len() > batch_size {
            self.events.drain(..batch_size).collect()
        } else {
            self.events.drain(..).collect()
        };

        let response = self
            .client
            .post(&self.backend_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&events_to_send)
            .send()?
            .text()?;

        Ok(response)
    }

    /// Get the number of buffered events
    pub fn pending_events_count(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = WhalyticsEventBuilder::default()
            .event("test_event")
            .user_id("user123")
            .session_id("session456")
            .build()
            .unwrap();

        assert_eq!(event.event, "test_event");
        assert_eq!(event.user_id, "user123");
        assert_eq!(event.session_id, "session456");
        assert!(event.time > 0);
    }

    #[test]
    fn test_client_creation() {
        let client = WhalyticsClient::new("test_api_key");
        assert_eq!(client.pending_events_count(), 0);
    }

    #[test]
    fn test_event_buffering() {
        let mut client = WhalyticsClient::new("test_api_key");

        let event = WhalyticsEventBuilder::default()
            .event("test_event")
            .user_id("user123")
            .session_id("session456")
            .build()
            .unwrap();

        client.log_event(event);
        assert_eq!(client.pending_events_count(), 1);
    }

    #[test]
    fn test_session_creation() {
        let session = WhalyticsSession::new("user123", "session456");
        assert_eq!(session.user_id(), "user123");
        assert_eq!(session.session_id(), "session456");
        assert!(session.user_properties().is_empty());
    }

    #[test]
    fn test_session_with_user_properties() {
        let mut user_props = HashMap::new();
        user_props.insert("platform".to_string(), serde_json::json!("rust"));
        user_props.insert("version".to_string(), serde_json::json!("1.0"));

        let session = WhalyticsSessionBuilder::default()
            .user_id("user123")
            .session_id("session456")
            .user_properties(user_props)
            .build()
            .unwrap();

        assert_eq!(session.user_properties().len(), 2);
    }

    #[test]
    fn test_session_event_creation() {
        let mut session = WhalyticsSession::new("user123", "session456");
        session.push_event("test_event", HashMap::new());

        let events = session.take_events(1);
        assert_eq!(events.len(), 1);
        let event = &events[0];

        assert_eq!(event.event, "test_event");
        assert_eq!(event.user_id, "user123");
        assert_eq!(event.session_id, "session456");
    }

    #[test]
    fn test_session_set_user_property() {
        let mut session = WhalyticsSession::new("user123", "session456");
        session.set_user_property("platform", serde_json::json!("rust"));
        session.set_user_property("level", serde_json::json!(10));

        assert_eq!(session.user_properties().len(), 2);

        // Events created after setting properties should include them
        session.push_event("test_event", HashMap::new());

        let events = session.take_events(1);
        let event = &events[0];

        assert_eq!(event.user_properties.len(), 2);
        assert_eq!(event.user_properties.get("platform").unwrap(), "rust");
    }

    #[test]
    fn test_session_defaults() {
        let session = WhalyticsSession::default();
        assert!(!session.user_id().is_empty());
        assert!(!session.session_id().is_empty());
        // Simple check to see if it looks like a UUID (36 chars)
        assert_eq!(session.user_id().len(), 36);
        assert_eq!(session.session_id().len(), 36);
    }

    #[test]
    fn test_session_id_precedence() {
        let mut session = WhalyticsSession::new("default_user", "default_session");

        let mut props = HashMap::new();
        props.insert("user_id".to_string(), serde_json::json!("custom_user"));
        props.insert(
            "session_id".to_string(),
            serde_json::json!("custom_session"),
        );

        session.push_event("test_event", props);

        let events = session.take_events(1);
        let event = &events[0];

        assert_eq!(event.user_id, "custom_user");
        assert_eq!(event.session_id, "custom_session");
    }
}
