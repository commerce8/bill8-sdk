// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-webhooks-contracts
//!
//! Wire types for the Bill8:Webhooks HTTP API — endpoints, events,
//! and deliveries. Signing secrets are secret-by-reference and never
//! serialize in plaintext.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A merchant-configured delivery target with its enabled event types.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct WebhookEndpoint {
    /// Endpoint id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Destination URL.
    pub url: String,
    /// Enabled event types (an endpoint receives only these).
    pub enabled_events: Vec<String>,
    /// Enabled state.
    pub enabled: bool,
    /// Secret reference holding the signing secret (Sentin8:Secrets).
    pub secret_ref: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A materialized domain event with its envelope.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Event {
    /// Event id (dedupe key — duplicate delivery materializes one row).
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Event type (e.g. `bill8:payments:charge:succeeded`).
    pub event_type: String,
    /// Full event payload.
    pub data: serde_json::Value,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A signed delivery attempt.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Delivery {
    /// Delivery id.
    pub id: Uuid,
    /// The delivered event.
    pub event_id: Uuid,
    /// The target endpoint.
    pub endpoint_id: Uuid,
    /// Lifecycle state (`pending` / `succeeded` / `retry` / `dead`).
    pub status: String,
    /// Attempt count (exponential backoff budget).
    pub attempts: i32,
    /// Last-attempt timestamp.
    pub last_attempted_at: Option<DateTime<Utc>>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/webhook-endpoints`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateWebhookEndpointRequest {
    /// Destination URL.
    pub url: String,
    /// Enabled event types.
    pub enabled_events: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_never_serializes_secret_material() {
        let e = WebhookEndpoint {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            url: "https://merchant.example/hooks".into(),
            enabled_events: vec!["bill8:payments:charge:succeeded".into()],
            enabled: true,
            secret_ref: "sec_wh_1".into(),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&e).unwrap();
        assert!(json.get("secret").is_none());
        assert_eq!(json["secret_ref"], "sec_wh_1");
    }

    #[test]
    fn event_envelope_shape() {
        let ev = Event {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            event_type: "bill8:payments:charge:succeeded".into(),
            data: serde_json::json!({ "charge_id": Uuid::new_v4() }),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&ev).unwrap();
        assert_eq!(json["event_type"], "bill8:payments:charge:succeeded");
        let de: Event = serde_json::from_value(json).unwrap();
        assert!(de.data["charge_id"].is_string());
    }

    #[test]
    fn delivery_status_spellings() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "event_id": Uuid::new_v4(),
            "endpoint_id": Uuid::new_v4(),
            "status": "retry",
            "attempts": 3,
            "last_attempted_at": "2026-09-29T00:00:00Z",
            "created_at": "2026-09-29T00:00:00Z"
        });
        let d: Delivery = serde_json::from_value(json).unwrap();
        assert_eq!(d.status, "retry");
        assert_eq!(d.attempts, 3);
    }
}