use whalytics_sdk::{WhalyticsClient, WhalyticsEventBuilder};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // Initialize the client
    let mut client = WhalyticsClient::new("YOUR_API_KEY_HERE");
    
    println!("Whalytics Rust SDK Example\n");
    
    // Example 1: Simple event
    println!("1. Logging a simple event...");
    let event1 = WhalyticsEventBuilder::default()
        .event("app_start")
        .user_id("rust_user_123")
        .session_id("rust_session_456")
        .build()
        .unwrap();
    client.log_event(event1);
    
    // Example 2: Event with properties
    println!("2. Logging an event with properties...");
    let mut event_props = HashMap::new();
    event_props.insert("level_id".to_string(), json!(5));
    event_props.insert("score".to_string(), json!(1500));
    event_props.insert("difficulty".to_string(), json!("hard"));
    
    let event2 = WhalyticsEventBuilder::default()
        .event("level_completed")
        .user_id("rust_user_123")
        .session_id("rust_session_456")
        .event_properties(event_props)
        .build()
        .unwrap();
    client.log_event(event2);
    
    // Example 3: Event with user properties
    println!("3. Logging an event with user properties...");
    let mut user_props = HashMap::new();
    user_props.insert("subscription_type".to_string(), json!("premium"));
    user_props.insert("level".to_string(), json!(10));
    user_props.insert("platform".to_string(), json!("rust"));
    
    let event3 = WhalyticsEventBuilder::default()
        .event("purchase")
        .user_id("rust_user_123")
        .session_id("rust_session_456")
        .user_properties(user_props)
        .build()
        .unwrap();
    client.log_event(event3);
    
    // Check pending events
    println!("\nPending events: {}", client.pending_events_count());
    
    // Flush all events
    println!("\nSending events to backend...");
    match client.flush() {
        Ok(response) => println!("✓ Success! Response: {}", response),
        Err(e) => eprintln!("✗ Error: {}", e),
    }
    
    println!("\nPending events after flush: {}", client.pending_events_count());
}
