# R3E Event

Event handling system for the R3E FaaS platform.

## Features

- Event source management
- Event registry for storing and retrieving events
- Trigger system for event-based actions
- Support for multiple blockchain sources (Neo, Bitcoin)
- RocksDB-based persistent storage

## Usage

```rust
use r3e_event::{Event, Context, EventData, Trigger, Source};
use r3e_event::registry::Registry;
use r3e_event::source::Source as EventSource;
use std::sync::Arc;

// Create a registry
let registry = Registry::new();

// Create an event
let context = Context {
    trigger: Trigger::NeoNewBlock,
    triggered_time: std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(),
    source: Source::Neo,
};

let event_data = EventData {
    id: uuid::Uuid::new_v4().to_string(),
    payload: serde_json::json!({
        "block_number": 12345,
        "block_hash": "0x1234567890abcdef",
    }),
};

let event = Event {
    context,
    data: event_data,
};

// Register the event
registry.register_event(event).await?;

// Get events by trigger
let events = registry.get_events_by_trigger(Trigger::NeoNewBlock).await?;
```

## Configuration

```rust
use r3e_event::config::{Config, StorageType, SourceConfig, RegistryConfig, TriggerConfig};

// Create a configuration with default values
let config = Config::default();

// Create a custom configuration
let custom_config = Config {
    storage_type: StorageType::RocksDB,
    rocksdb_path: Some("/path/to/rocksdb".to_string()),
    source: SourceConfig {
        enabled_sources: vec!["Neo".to_string(), "Bitcoin".to_string()],
        sources: serde_json::json!({
            "Neo": {
                "rpc_url": "http://localhost:10332",
            },
            "Bitcoin": {
                "rpc_url": "http://localhost:8332",
            },
        }),
    },
    registry: RegistryConfig {
        max_events: 10000,
        event_ttl: 86400 * 7, // 7 days
    },
    trigger: TriggerConfig {
        enabled_triggers: vec!["NeoNewBlock".to_string(), "NeoNewTx".to_string()],
        triggers: serde_json::json!({}),
    },
};
```
