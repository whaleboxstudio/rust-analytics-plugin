use serde_json::json;
use std::collections::HashMap;
use game_events_sdk::{WhalyticsClient, WhalyticsEventBuilder, WhalyticsSession};

fn main() {
    // Initialize the client
    let mut client = WhalyticsClient::new("YOUR_API_KEY_HERE");

    println!("game-events.io Rust SDK Example\n");

    // Example 1: Simple event (old way)
    println!("1. Logging a simple event (without session)...");
    let event1 = WhalyticsEventBuilder::default()
        .event("app_start")
        .user_id("rust_user_123")
        .session_id("rust_session_456")
        .build()
        .unwrap();
    client.log_event(event1);

    // Example 2: Using WhalyticsSession (recommended way)
    println!("2. Creating a session and logging events...");
    // New: Defaults to random UUIDs if not specified, but here we specify them
    let mut session = WhalyticsSession::new("rust_user_123", "rust_session_456");

    // Set user properties for the session
    session.set_user_property("platform", json!("rust"));
    session.set_user_property("subscription_type", json!("premium"));
    session.set_user_property("level", json!(10));

    // Create events in the session
    session.push_event("level_started", HashMap::new());

    // Example 3: Event with additional event properties
    println!("3. Logging an event with event properties...");
    let mut event_props = HashMap::new();
    event_props.insert("level_id".to_string(), json!(5));
    event_props.insert("score".to_string(), json!(1500));
    event_props.insert("difficulty".to_string(), json!("hard"));

    session.push_event("level_completed", event_props);

    // Example 4: Purchase event
    println!("4. Logging a purchase event...");
    let mut purchase_props = HashMap::new();
    purchase_props.insert("item_id".to_string(), json!("sword_legendary"));
    purchase_props.insert("price".to_string(), json!(9.99));
    purchase_props.insert("currency".to_string(), json!("USD"));

    session.push_event("purchase", purchase_props);

    // Move events from session to client
    println!("Moving events from session to client...");
    let session_events = session.take_events(100); // Take up to 100 events
    for event in session_events {
        client.log_event(event);
    }

    // Check pending events
    println!("\nPending events: {}", client.pending_events_count());

    // Flush all events
    println!("\nSending events to backend...");
    match client.flush() {
        Ok(response) => println!("✓ Success! Response: {}", response),
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    println!(
        "\nPending events after flush: {}",
        client.pending_events_count()
    );
}
