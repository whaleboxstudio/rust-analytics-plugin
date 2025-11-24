#![allow(non_snake_case)]

#[macro_use]
extern crate derive_builder;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event structure for Whalytics
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

/// Whalytics SDK client
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct WhalyticsClient {
    /// API key for authentication
    api_key: String,
    
    /// Backend URL (default: https://analytics.whaleboxstudio.com/v1/events)
    #[builder(default = "\"https://analytics.whaleboxstudio.com/v1/events\".to_string()")]
    backend_url: String,
    
    /// HTTP client for making requests
    #[builder(setter(skip))]
    #[builder(default = "reqwest::blocking::Client::new()")]
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
        
        let response = self.client
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
        
        let response = self.client
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
}
