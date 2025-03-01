// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Secret created
    SecretCreated,

    /// Secret accessed
    SecretAccessed,

    /// Secret updated
    SecretUpdated,

    /// Secret rotated
    SecretRotated,

    /// Secret deleted
    SecretDeleted,

    /// Vault key rotated
    VaultKeyRotated,

    /// Unauthorized access attempt
    UnauthorizedAccess,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event ID
    pub id: String,

    /// Event type
    pub event_type: AuditEventType,

    /// User ID
    pub user_id: String,

    /// Function ID
    pub function_id: Option<String>,

    /// Secret ID
    pub secret_id: Option<String>,

    /// Event timestamp
    pub timestamp: u64,

    /// Event details
    pub details: String,

    /// Source IP address
    pub source_ip: Option<String>,

    /// User agent
    pub user_agent: Option<String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: AuditEventType,
        user_id: String,
        function_id: Option<String>,
        secret_id: Option<String>,
        details: String,
        source_ip: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            event_type,
            user_id,
            function_id,
            secret_id,
            timestamp,
            details,
            source_ip,
            user_agent,
        }
    }
}

/// Audit logger trait
pub trait AuditLogger: Send + Sync {
    /// Log an audit event
    fn log_event(&self, event: AuditEvent);
}

/// Memory-based audit logger
pub struct MemoryAuditLogger {
    /// Maximum number of events to keep
    max_events: usize,

    /// Events
    events: Vec<AuditEvent>,
}

impl MemoryAuditLogger {
    /// Create a new memory-based audit logger
    pub fn new(max_events: usize) -> Self {
        Self {
            max_events,
            events: Vec::new(),
        }
    }

    /// Get all events
    pub fn get_events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Get events for a user
    pub fn get_user_events(&self, user_id: &str) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Get events for a secret
    pub fn get_secret_events(&self, secret_id: &str) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.secret_id.as_deref() == Some(secret_id))
            .cloned()
            .collect()
    }
}

impl AuditLogger for MemoryAuditLogger {
    fn log_event(&self, event: AuditEvent) {
        let mut events = self.events.clone();
        events.push(event);

        // Keep only the last max_events
        if events.len() > self.max_events {
            events = events.split_off(events.len() - self.max_events);
        }

        self.events = events;
    }
}
