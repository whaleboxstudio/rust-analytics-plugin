# game-events.io Rust SDK

A simple and efficient Rust SDK for game-events.io analytics.

## Features

- 🚀 **Simple API**: Easy-to-use builder pattern for events
- 📦 **Event Buffering**: Automatically buffers events for batch sending
- 🔄 **Batch Upload**: Send events in configurable batches
- 🛡️ **Type-Safe**: Leverages Rust's type system for safety
- ⚡ **Lightweight**: Minimal dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
game-events-sdk = { git = "https://github.com/game-events-io/rust-sdk.git" }
```

## Quick Start

```rust
use game_events_sdk::{WhalyticsClient, WhalyticsSession};
use std::collections::HashMap;

fn main() {
    // Initialize the client with your API key
    let mut client = WhalyticsClient::new("YOUR_API_KEY");
    
    // Create a session (automatically generates UUIDs for user_id and session_id)
    let mut session = WhalyticsSession::default();
    
    // Log an event
    session.push_event("level_completed", HashMap::new());
    
    // Move events from session to client
    for event in session.take_events(100) {
        client.log_event(event);
    }
    
    // Send all buffered events
    match client.flush() {
        Ok(response) => println!("Events sent: {}", response),
        Err(e) => eprintln!("Error sending events: {}", e),
    }
}
```

## Usage Examples

### Using Sessions (Recommended)

`WhalyticsSession` helps manage user_id, session_id, and user properties automatically.

```rust
use game_events_sdk::WhalyticsSession;
use serde_json::json;
use std::collections::HashMap;

// Create session with specific IDs
let mut session = WhalyticsSession::new("user_123", "session_456");

// Set user properties (added to all events)
session.set_user_property("platform", json!("rust"));
session.set_user_property("subscription", json!("premium"));

// Log events (stored internally)
session.push_event("app_start", HashMap::new());

let mut props = HashMap::new();
props.insert("level", json!(5));
session.push_event("level_start", props);

// Retrieve events to send
let events = session.take_events(10);
```

### Manual Event Creation

You can still create events manually if you prefer:

```rust
let event = WhalyticsEventBuilder::default()
    .event("button_click")
    .user_id("user_123")
    .session_id("session_456")
    .build()
    .unwrap();

client.log_event(event);
```

### Event with Properties (Manual)

```rust
use serde_json::json;

let mut event_props = HashMap::new();
event_props.insert("level_id".to_string(), json!(5));
event_props.insert("score".to_string(), json!(1500));
event_props.insert("difficulty".to_string(), json!("hard"));

let event = WhalyticsEventBuilder::default()
    .event("level_completed")
    .user_id("user_123")
    .session_id("session_456")
    .event_properties(event_props)
    .build()
    .unwrap();

client.log_event(event);
```

### Batch Upload

```rust
// Send events in batches of 100
while client.pending_events_count() > 0 {
    match client.flush_batch(100) {
        Ok(response) => println!("Batch sent: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Custom Backend URL

```rust
use game_events_sdk::WhalyticsClientBuilder;

let client = WhalyticsClientBuilder::default()
    .api_key("YOUR_API_KEY")
    .backend_url("https://api.game-events.io/v1/events") // Updated to new domain
    .build()
    .unwrap();
```

## API Reference

### `WhalyticsClient`

#### Methods

- `new(api_key: impl Into<String>) -> Self` - Create a new client
- `log_event(&mut self, event: WhalyticsEvent)` - Add an event to the buffer
- `flush(&mut self) -> Result<String, reqwest::Error>` - Send all buffered events
- `flush_batch(&mut self, batch_size: usize) -> Result<String, reqwest::Error>` - Send events in batches
- `pending_events_count(&self) -> usize` - Get the number of buffered events

### `WhalyticsEvent`

#### Fields

- `event: String` - Event name (required)
- `user_id: String` - Unique user identifier (required)
- `session_id: String` - Session identifier (required)
- `time: u64` - Unix timestamp in seconds (auto-generated if not provided)
- `event_properties: HashMap<String, serde_json::Value>` - Event-specific properties
- `user_properties: HashMap<String, serde_json::Value>` - User properties

## Requirements

- Rust 1.70 or later

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.