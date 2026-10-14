// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-merchants-contracts
//!
//! Wire types for the Bill8:Merchants HTTP API — connected accounts,
//! persons, external accounts, and links. Key material never appears in
//! these DTOs (the platform issuer is its only holder); bank details are
//! secret-by-reference.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A connected account with a verification state machine.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Account {
    /// Account id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Display name.
    pub display_name: String,
    /// Verification state (`unverified` / `pending` / `verified` /
    /// `rejected`).
    pub verification_status: String,
    /// Ledger account reference backing the account's balances.
    pub ledger_account_id: Option<Uuid>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// An individual associated with an account (owner, director).
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Person {
    /// Person id.
    pub id: Uuid,
    /// Owning account.
    pub account_id: Uuid,
    /// Full name.
    pub full_name: String,
    /// Relationship (`owner` / `director`).
    pub relationship: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A payout destination. Bank details are secret-by-reference.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExternalAccount {
    /// External-account id.
    pub id: Uuid,
    /// Owning account.
    pub account_id: Uuid,
    /// Instrument family (`bank`).
    pub bank_type: String,
    /// Opaque display descriptor (masked) — no secrets.
    pub display: String,
    /// Secret reference holding the bank details (Sentin8:Secrets).
    pub secret_ref: String,
    /// Verification state (`pending` / `verified` / `rejected`).
    pub verification_status: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A short-lived, single-use hosted-onboarding token.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccountLink {
    /// Link id.
    pub id: Uuid,
    /// Owning account.
    pub account_id: Uuid,
    /// Single-use token value (short-TTL).
    pub token: String,
    /// Expiry timestamp.
    pub expires_at: DateTime<Utc>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// A short-lived, single-use dashboard-access token.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginLink {
    /// Link id.
    pub id: Uuid,
    /// Owning account.
    pub account_id: Uuid,
    /// Single-use token value (short-TTL).
    pub token: String,
    /// Expiry timestamp.
    pub expires_at: DateTime<Utc>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Merchant API-key metadata. Key material is never persisted here —
/// the platform issuer is its only holder; this DTO is metadata + masked
/// display only.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiKey {
    /// Api-key record id.
    pub id: Uuid,
    /// Owning account.
    pub account_id: Uuid,
    /// Display name.
    pub display_name: String,
    /// Key mode (`live` / `test`).
    pub mode: String,
    /// Credential variant (`secret` / `publishable`).
    pub variant: String,
    /// Enabled state.
    pub enabled: bool,
    /// Masked display of the key material.
    pub masked: String,
    /// Platform credential reference (secret-by-reference).
    pub credential_ref: String,
    /// Last-use timestamp (best-effort).
    pub last_used_at: Option<DateTime<Utc>>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/accounts`.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateAccountRequest {
    /// Display name.
    pub display_name: String,
}

/// Request body for `POST /v1/persons`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePersonRequest {
    /// Owning connected account.
    pub account_id: Uuid,
    /// Full legal name.
    pub full_name: String,
    /// Relationship to the account (`owner` / `director`).
    pub relationship: String,
}

/// Request body for `POST /v1/external-accounts`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateExternalAccountRequest {
    /// Owning connected account.
    pub account_id: Uuid,
    /// Instrument family (`bank`; defaults when absent).
    pub bank_type: Option<String>,
    /// Masked display descriptor (last-4 style) — never the number.
    pub display: String,
}

/// Request body for `POST /v1/api-keys`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyRequest {
    /// Owning account.
    pub account_id: Uuid,
    /// Display name.
    pub display_name: String,
    /// Key mode (`live` / `test`).
    pub mode: String,
    /// Credential variant (`secret` / `publishable`).
    pub variant: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_roundtrip() {
        let a = Account {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            display_name: "Acme Seller".into(),
            verification_status: "unverified".into(),
            ledger_account_id: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&a).unwrap();
        assert_eq!(json["verification_status"], "unverified");
        let de: Account = serde_json::from_value(json).unwrap();
        assert_eq!(de.display_name, "Acme Seller");
    }

    #[test]
    fn api_key_never_carries_key_material() {
        let k = ApiKey {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            display_name: "prod key".into(),
            mode: "live".into(),
            variant: "secret".into(),
            enabled: true,
            masked: "sk_live_…9f2a".into(),
            credential_ref: "sec_1".into(),
            last_used_at: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&k).unwrap();
        // Only the masked display and a secret reference serialize.
        assert!(json.get("secret").is_none());
        assert!(json.get("key").is_none());
        assert_eq!(json["masked"], "sk_live_…9f2a");
    }

    #[test]
    fn external_account_is_secret_by_reference() {
        let e = ExternalAccount {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            bank_type: "bank".into(),
            display: "•••• 1234".into(),
            secret_ref: "sec_2".into(),
            verification_status: "verified".into(),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&e).unwrap();
        assert!(json.get("account_number").is_none());
        assert_eq!(json["secret_ref"], "sec_2");
    }
}