// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-checkout-contracts
//!
//! Wire types for the Bill8:Checkout HTTP API — sessions, line items,
//! and payment links. Money is a JSON string on the wire (never a float).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A checkout session with a mode and guarded single-use lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Session {
    /// Session id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Session mode (`payment` / `subscription` / `setup`).
    pub mode: String,
    /// Lifecycle state (`open` / `complete` / `expired`).
    pub status: String,
    /// Success URL (validated at creation — no open redirect).
    pub success_url: String,
    /// Cancel URL (validated at creation).
    pub cancel_url: String,
    /// Expiry timestamp.
    pub expires_at: DateTime<Utc>,
    /// Originating payment link (when created from one).
    pub payment_link_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// An item on a session — references a billing price or inline data.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LineItem {
    /// Item id.
    pub id: Uuid,
    /// Owning session.
    pub session_id: Uuid,
    /// Billing price reference (optional when `inline_data` is used).
    pub price_id: Option<Uuid>,
    /// Inline price data (optional when `price_id` is used).
    pub inline_data: Option<serde_json::Value>,
    /// Quantity.
    pub quantity: i64,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A reusable, shareable link that spawns a fresh session per visit.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaymentLink {
    /// Link id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Public link token.
    pub token: String,
    /// Enabled state (disabled links spawn no sessions).
    pub enabled: bool,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/sessions`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSessionRequest {
    /// Session mode (`payment` / `subscription` / `setup`).
    pub mode: String,
    /// Success URL (validated at creation).
    pub success_url: String,
    /// Cancel URL (validated at creation).
    pub cancel_url: String,
    /// Line items (price references or inline data).
    pub line_items: Vec<CreateLineItemInput>,
}

/// A line-item input on session creation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateLineItemInput {
    /// Billing price reference.
    #[serde(default)]
    pub price_id: Option<Uuid>,
    /// Inline price data.
    #[serde(default)]
    pub inline_data: Option<serde_json::Value>,
    /// Quantity.
    #[serde(default = "one")]
    pub quantity: i64,
}

fn one() -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_roundtrip() {
        let s = Session {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            mode: "payment".into(),
            status: "open".into(),
            success_url: "https://shop.example/success".into(),
            cancel_url: "https://shop.example/cancel".into(),
            expires_at: Utc::now(),
            payment_link_id: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json["mode"], "payment");
        let de: Session = serde_json::from_value(json).unwrap();
        assert_eq!(de.success_url, "https://shop.example/success");
    }

    #[test]
    fn session_request_defaults_quantity() {
        let json = serde_json::json!({
            "mode": "subscription",
            "success_url": "https://a.example/s",
            "cancel_url": "https://a.example/c",
            "line_items": [{ "price_id": Uuid::new_v4() }]
        });
        let req: CreateSessionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.line_items[0].quantity, 1);
    }

    #[test]
    fn payment_link_token_serializes() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "env_id": Uuid::new_v4(),
            "token": "plink_abc123",
            "enabled": true,
            "created_at": "2026-09-29T00:00:00Z"
        });
        let l: PaymentLink = serde_json::from_value(json).unwrap();
        assert_eq!(l.token, "plink_abc123");
        assert!(l.enabled);
    }
}
