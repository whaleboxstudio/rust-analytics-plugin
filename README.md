# Whalytics Rust SDK

A simple and efficient Rust SDK for Whalytics analytics.

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
whalytics-sdk = { git = "https://github.com/whaleboxstudio/whalytics-rust-sdk.git" }
```

## Quick Start

```rust
use whalytics_sdk::{WhalyticsClient, WhalyticsEventBuilder};
use std::collections::HashMap;

fn main() {
    // Initialize the client with your API key
    let mut client = WhalyticsClient::new("YOUR_API_KEY");
    
    // Create and log an event
    let event = WhalyticsEventBuilder::default()
        .event("level_completed")
        .user_id("user_123")
        .session_id("session_456")
        .build()
        .unwrap();
    
    client.log_event(event);
    
    // Send all buffered events
    match client.flush() {
        Ok(response) => println!("Events sent: {}", response),
        Err(e) => eprintln!("Error sending events: {}", e),
    }
}
```

## Usage Examples

### Basic Event

```rust
let event = WhalyticsEventBuilder::default()
    .event("button_click")
    .user_id("user_123")
    .session_id("session_456")
    .build()
    .unwrap();

client.log_event(event);
```

### Event with Properties

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

### User Properties

```rust
use serde_json::json;

let mut user_props = HashMap::new();
user_props.insert("subscription_type".to_string(), json!("premium"));
user_props.insert("level".to_string(), json!(10));

let event = WhalyticsEventBuilder::default()
    .event("session_start")
    .user_id("user_123")
    .session_id("session_456")
    .user_properties(user_props)
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
use whalytics_sdk::WhalyticsClientBuilder;

let client = WhalyticsClientBuilder::default()
    .api_key("YOUR_API_KEY")
    .backend_url("https://your-custom-backend.com/v1/events")
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
