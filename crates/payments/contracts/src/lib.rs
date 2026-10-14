// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-payments-contracts
//!
//! Wire types for the Bill8:Payments HTTP API. See the crate `README.md`
//! for the design overview and the wire-format invariants: money is a JSON
//! string on the wire (never a float), status spellings are canonical, and
//! secret-bearing fields never appear in response DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// utoipa::ToSchema is derived on wire types so services can reference
// them in #[utoipa::path] request/response annotations.
use utoipa::ToSchema;
// ── Customer ────────────────────────────────────────────────────────────

/// A payment customer, optionally linked to a platform identity.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Customer {
    /// Customer id (globally unique).
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Display name.
    pub display_name: String,
    /// Email address.
    pub email: Option<String>,
    /// Lifecycle state (`active` / `deleted`).
    pub status: String,
    /// Linked platform identity (one linkage per customer).
    pub sentin8_auth_user_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/customers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCustomerRequest {
    /// Display name.
    pub display_name: String,
    /// Email address.
    pub email: Option<String>,
}

// ── Payment method ──────────────────────────────────────────────────────

/// A saved payment instrument. Instrument data is secret-by-reference;
/// this DTO never carries PANs or account numbers.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaymentMethod {
    /// Payment-method id.
    pub id: Uuid,
    /// Owning customer.
    pub customer_id: Uuid,
    /// Instrument family (`card` / `bank_debit` / `wallet`).
    pub method_type: String,
    /// Opaque display descriptor (e.g. masked last digits) — no secrets.
    pub display: String,
    /// Secret reference holding the instrument data (Sentin8:Secrets).
    pub secret_ref: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/payment-methods`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePaymentMethodRequest {
    /// Owning customer.
    pub customer_id: Uuid,
    /// Instrument family.
    pub method_type: String,
    /// Secret reference to the instrument data.
    pub secret_ref: String,
}

// ── Payment intent ──────────────────────────────────────────────────────

/// An intent to pay a specific amount.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaymentIntent {
    /// Intent id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Paying customer.
    pub customer_id: Option<Uuid>,
    /// Payment method bound to the intent.
    pub payment_method_id: Option<Uuid>,
    /// Amount as a JSON string on the wire (money is never a float).
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Lifecycle state (`requires_confirmation` / `processing` /
    /// `requires_action` / `succeeded` / `failed`).
    pub status: String,
    /// Settlement channel selected for this intent (routing outcome).
    pub channel_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/payment-intents`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePaymentIntentRequest {
    /// Paying customer.
    pub customer_id: Option<Uuid>,
    /// Payment method bound up-front (optional; can be attached later).
    pub payment_method_id: Option<Uuid>,
    /// Amount as a decimal string.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// A setup intent — saving a payment method for later use without a
/// charge (PAY-5). The lifecycle mirrors the payment intent's
/// confirmation phase minus the money movement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SetupIntent {
    /// Setup intent id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Customer whose method is being saved.
    pub customer_id: Option<Uuid>,
    /// The method being saved (attached on confirm when the channel
    /// provides it).
    pub payment_method_id: Option<Uuid>,
    /// Lifecycle state (`requires_confirmation` / `processing` /
    /// `succeeded` / `failed`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/setup-intents`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSetupIntentRequest {
    /// Customer whose method is being saved.
    pub customer_id: Option<Uuid>,
    /// Payment method bound up-front (optional; can be attached later).
    pub payment_method_id: Option<Uuid>,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

// ── Charge ───────────────────────────────────────────────────────────────

/// An immutable money capture routed through a settlement channel.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Charge {
    /// Charge id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Paying customer.
    pub customer_id: Option<Uuid>,
    /// Payment method used.
    pub payment_method_id: Option<Uuid>,
    /// Originating intent (when created from one).
    pub payment_intent_id: Option<Uuid>,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// Amount already refunded, as a JSON string (compensating records only;
    /// `amount` itself is append-only and never changes).
    pub amount_refunded: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Lifecycle state (`pending` / `succeeded` / `failed` /
    /// `partially_refunded` / `refunded` / `disputed`).
    pub status: String,
    /// Settlement channel that executed the charge (routing outcome).
    pub channel_id: Option<Uuid>,
    /// Ledger balance-transaction reference proving the money movement.
    pub balance_transaction_id: Option<Uuid>,
    /// Connected account the charge is on behalf of (split-at-charge).
    pub on_behalf_of: Option<Uuid>,
    /// Platform fee taken on the charge, as a JSON string.
    pub application_fee_amount: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/charges`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateChargeRequest {
    /// Paying customer.
    pub customer_id: Option<Uuid>,
    /// Payment method to charge.
    pub payment_method_id: Option<Uuid>,
    /// Originating intent (optional — direct charges carry none).
    pub payment_intent_id: Option<Uuid>,
    /// Amount as a decimal string.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Connected account the charge is on behalf of (split-at-charge).
    #[serde(default)]
    pub on_behalf_of: Option<Uuid>,
    /// Platform fee taken on the charge.
    #[serde(default)]
    pub application_fee_amount: Option<String>,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

// ── Refund ───────────────────────────────────────────────────────────────

/// A compensating record reversing (part of) a charge.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Refund {
    /// Refund id.
    pub id: Uuid,
    /// The charge being refunded.
    pub charge_id: Uuid,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// Lifecycle state (`succeeded` / `failed`).
    pub status: String,
    /// Ledger reversal-transaction reference.
    pub balance_transaction_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/refunds`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRefundRequest {
    /// The charge to refund.
    pub charge_id: Uuid,
    /// Amount as a decimal string (defaults to the remaining refundable).
    pub amount: Option<String>,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

// ── Dispute ──────────────────────────────────────────────────────────────

/// A challenge to a charge that can claw back funds.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Dispute {
    /// Dispute id.
    pub id: Uuid,
    /// The challenged charge.
    pub charge_id: Uuid,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// Dispute reason code.
    pub reason: String,
    /// Lifecycle state (`needs_response` / `under_review` / `won` / `lost`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/disputes/{id}/evidence`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubmitEvidenceRequest {
    /// Storage references to evidence documents.
    pub evidence_refs: Vec<String>,
}

// ── Settlement channel ───────────────────────────────────────────────────

/// A runtime-registered settlement channel with its capability matrix.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Channel {
    /// Channel id.
    pub id: Uuid,    /// Owning tenant environment.
    pub env_id: Uuid,    /// Machine name (`finance8` / `simulation` / external adapters).
    pub name: String,
    /// Display label.
    pub label: String,
    /// Supported method types (routing input).
    pub methods: Vec<String>,
    /// Supported regions (routing input).
    pub regions: Vec<String>,
    /// Whether the channel supports recurring charges.
    pub recurring: bool,
    /// Channel mode (`live` / `test`).
    pub mode: String,
    /// Enabled state.
    pub enabled: bool,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `PUT /v1/routing-rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRoutingRuleRequest {
    /// Priority (lower evaluated first).
    pub priority: i32,
    /// Method-type matcher (optional).
    pub method_type: Option<String>,
    /// Region matcher (optional).
    pub region: Option<String>,
    /// Target channel.
    pub channel_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charge_roundtrip_keeps_money_as_string() {
        let charge = Charge {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            customer_id: None,
            payment_method_id: None,
            payment_intent_id: None,
            amount: "1000.00".into(),
            amount_refunded: "0.00".into(),
            currency: "USD".into(),
            status: "succeeded".into(),
            channel_id: Some(Uuid::new_v4()),
            balance_transaction_id: Some(Uuid::new_v4()),
            on_behalf_of: None,
            application_fee_amount: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&charge).unwrap();
        assert!(json["amount"].is_string());
        assert_eq!(json["amount"], "1000.00");
        let de: Charge = serde_json::from_value(json).unwrap();
        assert_eq!(de.amount, "1000.00");
        assert_eq!(de.status, "succeeded");
    }

    #[test]
    fn payment_intent_deserializes_from_request_shape() {
        let json = serde_json::json!({
            "customer_id": Uuid::new_v4(),
            "payment_method_id": null,
            "amount": "2500.50",
            "currency": "EUR",
            "idempotency_key": "order-42"
        });
        let req: CreatePaymentIntentRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.amount, "2500.50");
        assert_eq!(req.currency, "EUR");
        assert_eq!(req.idempotency_key.as_deref(), Some("order-42"));
    }

    #[test]
    fn refund_request_amount_optional() {
        let json = serde_json::json!({ "charge_id": Uuid::new_v4() });
        let req: CreateRefundRequest = serde_json::from_value(json).unwrap();
        assert!(req.amount.is_none());
    }

    #[test]
    fn channel_carries_capability_matrix() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "env_id": Uuid::new_v4(),
            "name": "simulation",
            "label": "Simulation",
            "methods": ["card", "bank_debit"],
            "regions": ["global"],
            "recurring": true,
            "mode": "test",
            "enabled": true,
            "created_at": "2026-09-29T00:00:00Z"
        });
        let ch: Channel = serde_json::from_value(json).unwrap();
        assert_eq!(ch.mode, "test");
        assert!(ch.methods.contains(&"card".to_string()));
    }
}