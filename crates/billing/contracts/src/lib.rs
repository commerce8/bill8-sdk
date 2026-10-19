// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-billing-contracts
//!
//! Wire types for the Bill8:Billing HTTP API - products, prices,
//! subscriptions, invoices, usage, coupons, credits, and credit notes.
//! Money is a JSON string on the wire (never a float).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A catalog product.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Product {
    /// Product id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Product name.
    pub name: String,
    /// Lifecycle state (`active` / `archived`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A recurring or one-off price point.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Price {
    /// Price id.
    pub id: Uuid,
    /// Owning product.
    pub product_id: Uuid,
    /// Recurring interval (`day` / `week` / `month` / `year`, or `null`
    /// for one-off).
    pub interval: Option<String>,
    /// Pricing model for metered prices (`flat` / `tiered` / `volume` /
    /// `packaged`).
    pub pricing_model: String,
    /// Unit amount as a JSON string on the wire.
    pub unit_amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A customer's plan instance.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Subscription {
    /// Subscription id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Subscribing customer.
    pub customer_id: Uuid,
    /// Lifecycle state (`incomplete` / `trialing` / `active` /
    /// `past_due` / `paused` / `canceled`).
    pub status: String,
    /// Trial end (when the subscription starts with a trial).
    pub trial_end: Option<DateTime<Utc>>,
    /// Billing-cycle anchor (calendar alignment or custom date).
    pub billing_cycle_anchor: Option<DateTime<Utc>>,
    /// Current period end.
    pub current_period_end: DateTime<Utc>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A quantified line on a subscription.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SubscriptionItem {
    /// Item id.
    pub id: Uuid,
    /// Owning subscription.
    pub subscription_id: Uuid,
    /// Price subscribed to.
    pub price_id: Uuid,
    /// Quantity.
    pub quantity: i64,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A generated bill.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Invoice {
    /// Invoice id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Billed customer.
    pub customer_id: Uuid,
    /// Billed subscription (when generated from one).
    pub subscription_id: Option<Uuid>,
    /// Amount due as a JSON string on the wire.
    pub amount_due: String,
    /// Amount paid as a JSON string on the wire.
    pub amount_paid: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Lifecycle state (`draft` / `open` / `paid` / `void` / `retrying`).
    pub status: String,
    /// Period start.
    pub period_start: DateTime<Utc>,
    /// Period end.
    pub period_end: DateTime<Utc>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A line item on an invoice.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct InvoiceItem {
    /// Item id.
    pub id: Uuid,
    /// Owning invoice.
    pub invoice_id: Uuid,
    /// Description.
    pub description: String,
    /// Amount as a JSON string on the wire (tax lines are explicit items).
    pub amount: String,
    /// Line kind (`subscription` / `usage` / `proration` / `coupon` /
    /// `credit` / `tax`).
    pub kind: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A metered consumption event.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UsageRecord {
    /// Record id.
    pub id: Uuid,
    /// Owning subscription item.
    pub subscription_item_id: Uuid,
    /// Quantity consumed.
    pub quantity: i64,
    /// Event timestamp (cutoff lands late events on the next cycle).
    pub recorded_at: DateTime<Utc>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A discount applied to invoices.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Coupon {
    /// Coupon id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Coupon code.
    pub code: String,
    /// Discount kind (`percentage` / `fixed`).
    pub discount_kind: String,
    /// Discount value (percent or minor-unit fixed amount) as a string.
    pub value: String,
    /// Redemption limit (row-lock bounded).
    pub redemption_limit: Option<i64>,
    /// Expiry timestamp.
    pub expires_at: Option<DateTime<Utc>>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Prepaid value applied to invoices before charges.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Credit {
    /// Credit id.
    pub id: Uuid,
    /// Owning customer.
    pub customer_id: Uuid,
    /// Remaining amount as a JSON string on the wire.
    pub remaining: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// An invoice-level adjustment (charge-level refunds belong to payments).
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreditNote {
    /// Credit-note id.
    pub id: Uuid,
    /// The adjusted invoice.
    pub invoice_id: Uuid,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// Lifecycle state (`issued` / `void`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/subscriptions`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSubscriptionRequest {
    /// Subscribing customer.
    pub customer_id: Uuid,
    /// Price to subscribe to.
    pub price_id: Uuid,
    /// Quantity.
    #[serde(default = "one")]
    pub quantity: i64,
    /// Trial end (when starting with a trial).
    #[serde(default)]
    pub trial_end: Option<DateTime<Utc>>,
    /// Billing-cycle anchor.
    #[serde(default)]
    pub billing_cycle_anchor: Option<DateTime<Utc>>,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

fn one() -> i64 {
    1
}

/// Request body for `POST /v1/usage-records`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUsageRecordRequest {
    /// Owning subscription item.
    pub subscription_item_id: Uuid,
    /// Quantity consumed.
    pub quantity: i64,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoice_roundtrip_money_is_string() {
        let inv = Invoice {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            subscription_id: Some(Uuid::new_v4()),
            amount_due: "49.99".into(),
            amount_paid: "0.00".into(),
            currency: "USD".into(),
            status: "open".into(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&inv).unwrap();
        assert!(json["amount_due"].is_string());
        let de: Invoice = serde_json::from_value(json).unwrap();
        assert_eq!(de.amount_due, "49.99");
    }

    #[test]
    fn subscription_request_defaults_quantity() {
        let json = serde_json::json!({
            "customer_id": Uuid::new_v4(),
            "price_id": Uuid::new_v4()
        });
        let req: CreateSubscriptionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.quantity, 1);
        assert!(req.trial_end.is_none());
    }

    #[test]
    fn price_carries_pricing_model() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "product_id": Uuid::new_v4(),
            "interval": "month",
            "pricing_model": "tiered",
            "unit_amount": "100.00",
            "currency": "USD",
            "created_at": "2026-09-29T00:00:00Z"
        });
        let p: Price = serde_json::from_value(json).unwrap();
        assert_eq!(p.pricing_model, "tiered");
        // Money stays a string on the wire (invariant holds on the
        // deserialized form - the raw JSON assertion lives in the
        // round-trip tests).
        assert!(p.unit_amount.contains('.'));
    }

    #[test]
    fn invoice_item_kinds_cover_tax() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "invoice_id": Uuid::new_v4(),
            "description": "Sales tax (8.875%)",
            "amount": "4.43",
            "kind": "tax",
            "created_at": "2026-09-29T00:00:00Z"
        });
        let item: InvoiceItem = serde_json::from_value(json).unwrap();
        assert_eq!(item.kind, "tax");
    }
}
