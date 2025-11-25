# WhalyticsSession Usage Guide

## Overview

`WhalyticsSession` provides a convenient way to manage session-level data (user_id, session_id, and user properties) and automatically include them in all events created from that session. It also buffers events internally until you are ready to send them.

## Benefits

- **Less repetition**: Set user_id, session_id, and user properties once per session
- **Consistency**: All events from the same session automatically share the same properties
- **Convenience**: Simpler API for creating multiple events with shared context
- **Automatic Defaults**: Automatically generates UUIDs for user_id and session_id if not provided

## Basic Usage

### Creating a Session

```rust
use whalytics_sdk::WhalyticsSession;

// Session with auto-generated UUIDs for user_id and session_id
let session = WhalyticsSession::default();

// Session with specific IDs
let session = WhalyticsSession::new("user_123", "session_456");
```

### Creating a Session with User Properties

```rust
use whalytics_sdk::WhalyticsSessionBuilder;
use serde_json::json;
use std::collections::HashMap;

let mut user_props = HashMap::new();
user_props.insert("platform".to_string(), json!("rust"));
user_props.insert("version".to_string(), json!("1.0"));

let session = WhalyticsSessionBuilder::default()
    .user_id("user_123")
    .session_id("session_456")
    .user_properties(user_props)
    .build()
    .unwrap();
```

### Adding User Properties Dynamically

```rust
let mut session = WhalyticsSession::new("user_123", "session_456");

// Add user properties one at a time
session.set_user_property("platform", json!("rust"));
session.set_user_property("subscription_type", json!("premium"));
session.set_user_property("level", json!(10));
```

### Creating Events in a Session

```rust
use std::collections::HashMap;

// Simple event - user_id, session_id, and user_properties are automatically included
session.push_event("level_started", HashMap::new());

// Event with additional event properties
let mut event_props = HashMap::new();
event_props.insert("level_id".to_string(), json!(5));
event_props.insert("score".to_string(), json!(1500));

session.push_event("level_completed", event_props);
```

### Overriding IDs per Event

If you pass `user_id` or `session_id` in the event properties, they will override the session's default values for that specific event:

```rust
let mut props = HashMap::new();
props.insert("user_id".to_string(), json!("temp_user"));
session.push_event("anonymous_action", props);
```

## Complete Example

```rust
use whalytics_sdk::{WhalyticsClient, WhalyticsSession};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // Initialize client
    let mut client = WhalyticsClient::new("YOUR_API_KEY");
    
    // Create a session
    let mut session = WhalyticsSession::new("user_123", "session_456");
    
    // Set user properties for the session
    session.set_user_property("platform", json!("rust"));
    
    // Log multiple events - stored internally in session
    session.push_event("app_start", HashMap::new());
    
    let mut level_props = HashMap::new();
    level_props.insert("level_id".to_string(), json!(5));
    session.push_event("level_completed", level_props);
    
    // Move events from session to client to send them
    for event in session.take_events(100) {
        client.log_event(event);
    }
    
    // Send all events
    client.flush().unwrap();
}
```

## API Reference

### WhalyticsSession Methods

- `new(user_id, session_id)` - Create a new session with specific IDs
- `default()` - Create a new session with random UUIDs
- `push_event(event_name, properties)` - Create an event and store it in the session buffer
- `take_events(count)` - Retrieve and remove up to `count` events from the session buffer
- `set_user_property(key, value)` - Add or update a user property
- `set_user_properties(map)` - Replace all user properties
- `user_id()` - Get the user_id
- `session_id()` - Get the session_id
- `user_properties()` - Get all user properties
