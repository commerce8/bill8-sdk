// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-ledger-contracts
//!
//! Wire types for the Bill8:Ledger HTTP API — the double-entry money
//! engine. Money is a JSON string on the wire (never a float); balance
//! transactions are append-only double-entry movements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A named money position per currency, e.g. `available` or `platform_fee`.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Balance {
    /// Balance id.
    pub id: Uuid,
    /// Owning ledger account.
    pub account_id: Uuid,
    /// Balance name (`available` / `pending` / `connected_reserved` /
    /// `platform_fee`).
    pub name: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Amount as a JSON string on the wire (money is never a float).
    pub amount: String,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// An append-only double-entry movement debiting one balance and
/// crediting another.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BalanceTransaction {
    /// Transaction id.
    pub id: Uuid,
    /// Debit leg (source balance).
    pub debit_balance_id: Uuid,
    /// Credit leg (destination balance).
    pub credit_balance_id: Uuid,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Movement type (`charge` / `refund` / `payout` / `transfer` / `fee`
    /// / `fx` / `dispute_reversal`).
    pub transaction_type: String,
    /// Originating resource (`charge` / `payout` / …).
    pub source_type: Option<String>,
    /// Originating resource id.
    pub source_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A withdrawal of available balance to an external account.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Payout {
    /// Payout id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Ledger account being paid out.
    pub account_id: Uuid,
    /// External account destination (secret-by-reference).
    pub external_account_ref: String,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Lifecycle state (`pending` / `paid` / `failed`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/payouts`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePayoutRequest {
    /// Ledger account being paid out.
    pub account_id: Uuid,
    /// External account destination (secret-by-reference).
    pub external_account_ref: String,
    /// Amount as a decimal string.
    pub amount: String,
    /// ISO 4217 currency code of the payout.
    pub currency: String,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// An internal movement between connected-account balances.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Transfer {
    /// Transfer id.
    pub id: Uuid,
    /// Debit balance (source).
    pub from_balance_id: Uuid,
    /// Credit balance (destination).
    pub to_balance_id: Uuid,
    /// Amount as a JSON string on the wire.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A request to post one double-entry movement (used by the payments and
/// billing services via their cross-service M2M workloads).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostTransactionRequest {
    /// Debit leg (source balance).
    pub debit_balance_id: Uuid,
    /// Credit leg (destination balance).
    pub credit_balance_id: Uuid,
    /// Amount as a decimal string.
    pub amount: String,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Movement type.
    pub transaction_type: String,
    /// Originating resource (`charge` / `payout` / …).
    pub source_type: Option<String>,
    /// Originating resource id.
    pub source_id: Option<Uuid>,
    /// Caller-supplied idempotency key.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_transaction_roundtrip_money_is_string() {
        let tx = BalanceTransaction {
            id: Uuid::new_v4(),
            debit_balance_id: Uuid::new_v4(),
            credit_balance_id: Uuid::new_v4(),
            amount: "99.99".into(),
            currency: "USD".into(),
            transaction_type: "charge".into(),
            source_type: Some("charge".into()),
            source_id: Some(Uuid::new_v4()),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&tx).unwrap();
        assert!(json["amount"].is_string());
        let de: BalanceTransaction = serde_json::from_value(json).unwrap();
        assert_eq!(de.amount, "99.99");
        assert_eq!(de.transaction_type, "charge");
    }

    #[test]
    fn post_transaction_request_parses() {
        let json = serde_json::json!({
            "debit_balance_id": Uuid::new_v4(),
            "credit_balance_id": Uuid::new_v4(),
            "amount": "100.00",
            "currency": "USD",
            "transaction_type": "charge",
            "source_type": "charge",
            "source_id": Uuid::new_v4(),
            "idempotency_key": "k-1"
        });
        let req: PostTransactionRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.amount, "100.00");
        assert_eq!(req.idempotency_key.as_deref(), Some("k-1"));
    }

    #[test]
    fn payout_status_spellings() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "env_id": Uuid::new_v4(),
            "account_id": Uuid::new_v4(),
            "external_account_ref": "ext_1",
            "amount": "500.00",
            "currency": "USD",
            "status": "pending",
            "created_at": "2026-09-29T00:00:00Z"
        });
        let p: Payout = serde_json::from_value(json).unwrap();
        assert_eq!(p.status, "pending");
    }
}