// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-risk-contracts
//!
//! Wire types for the Bill8:Risk HTTP API — rules, screenings, and
//! reviews. Evaluation fails closed: a malformed rule blocks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A declarative fraud rule with an action.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Rule {
    /// Rule id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Rule predicate kind (`amount_above` / `country_in` /
    /// `velocity_above`).
    pub predicate: String,
    /// Predicate parameters (threshold, list, window).
    pub params: serde_json::Value,
    /// Action on match (`allow` / `review` / `block`).
    pub action: String,
    /// Evaluation priority (lower first).
    pub priority: i32,
    /// Enabled state.
    pub enabled: bool,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// The outcome of screening a charge context against the rule set.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Screening {
    /// Screening id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// The screened charge context.
    pub charge_context: serde_json::Value,
    /// Outcome (`allow` / `review` / `block`) — fail-closed on evaluation
    /// error (a `block`, never an `allow`).
    pub outcome: String,
    /// The rule that decided (first match by priority).
    pub matched_rule_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A queued manual adjudication case.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Review {
    /// Review id.
    pub id: Uuid,
    /// The originating screening.
    pub screening_id: Uuid,
    /// Lifecycle state (`open` / `approved` / `declined`).
    pub status: String,
    /// Resolution timestamp.
    pub resolved_at: Option<DateTime<Utc>>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRuleRequest {
    /// Rule predicate kind.
    pub predicate: String,
    /// Predicate parameters.
    pub params: serde_json::Value,
    /// Action on match.
    pub action: String,
    /// Evaluation priority.
    #[serde(default)]
    pub priority: i32,
}

/// Request body for `POST /v1/screenings` (M2M from payments).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScreenRequest {
    /// The charge context to screen (amount, currency, method, region).
    pub charge_context: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_roundtrip() {
        let r = Rule {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            predicate: "amount_above".into(),
            params: serde_json::json!({ "threshold": "5000.00" }),
            action: "review".into(),
            priority: 10,
            enabled: true,
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&r).unwrap();
        assert_eq!(json["action"], "review");
        let de: Rule = serde_json::from_value(json).unwrap();
        assert_eq!(de.predicate, "amount_above");
    }

    #[test]
    fn screening_outcome_spellings() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "env_id": Uuid::new_v4(),
            "charge_context": { "amount": "12000.00", "currency": "USD" },
            "outcome": "block",
            "matched_rule_id": Uuid::new_v4(),
            "created_at": "2026-09-29T00:00:00Z"
        });
        let s: Screening = serde_json::from_value(json).unwrap();
        assert_eq!(s.outcome, "block");
    }

    #[test]
    fn review_resolution_is_idempotent_shape() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "screening_id": Uuid::new_v4(),
            "status": "approved",
            "resolved_at": "2026-09-29T12:00:00Z",
            "created_at": "2026-09-29T11:00:00Z"
        });
        let r: Review = serde_json::from_value(json).unwrap();
        assert_eq!(r.status, "approved");
        assert!(r.resolved_at.is_some());
    }
}
