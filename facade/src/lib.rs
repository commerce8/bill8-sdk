// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8 — Bill8 workspace namespace facade
//!
//! The unified Rust entry point for the bill8 vertical: SDK clients
//! and contracts for all eight services, imported as
//! `bill8::{service}::{…}`. Consumers never import a leaf crate
//! directly (architecture-principles §2.1).
//!
//! # Modules
//!
//! | Module | Service | Purpose |
//! |--------|---------|---------|
//! | [`merchants`] | Merchants | Connected accounts, persons, API keys |
//! | [`payments`] | Payments | Charges, refunds, disputes, channels |
//! | [`checkout`] | Checkout | Sessions, payment links |
//! | [`billing`] | Billing | Subscriptions, invoices, usage |
//! | [`ledger`] | Ledger | Balances, payouts, transfers |
//! | [`webhooks`] | Webhooks | Endpoints, events, deliveries |
//! | [`risk`] | Risk | Rules, screenings, reviews |
//! | [`reporting`] | Reporting | Report types, runs |
//!
//! # Quick start
//!
//! ```no_run
//! # use bill8::ClientBuilder;
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let ledger = bill8::ledger::LedgerClient::new(
//!     "https://bill8-ledger.ops.sys.sentin8.local",
//! ).with_token("m2m-token");
//! let balances = ledger.list_balances().await?;
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};

/// Bill8:Merchants — SDK client + contracts.
pub mod merchants {
    pub use bill8_merchants_contracts as contracts;
    mod sdk;
    pub use sdk::{MerchantsClient, MerchantsError};
}

/// Bill8:Payments — SDK client + contracts.
pub mod payments {
    pub use bill8_payments_contracts as contracts;
    mod sdk;
    pub use sdk::{PaymentsClient, PaymentsError};
}

/// Bill8:Checkout — SDK client + contracts.
pub mod checkout {
    pub use bill8_checkout_contracts as contracts;
    mod sdk;
    pub use sdk::{CheckoutClient, CheckoutError};
}

/// Bill8:Billing — SDK client + contracts.
pub mod billing {
    pub use bill8_billing_contracts as contracts;
    mod sdk;
    pub use sdk::{BillingClient, BillingError};
}

/// Bill8:Ledger — SDK client + contracts.
pub mod ledger {
    pub use bill8_ledger_contracts as contracts;
    mod sdk;
    pub use sdk::{LedgerClient, LedgerError};
}

/// Bill8:Webhooks — SDK client + contracts.
pub mod webhooks {
    pub use bill8_webhooks_contracts as contracts;
    mod sdk;
    pub use sdk::{WebhooksClient, WebhooksError};
}

/// Bill8:Risk — SDK client + contracts.
pub mod risk {
    pub use bill8_risk_contracts as contracts;
    mod sdk;
    pub use sdk::{RiskClient, RiskError};
}

/// Bill8:Reporting — SDK client + contracts.
/// Bill8:Reporting — SDK client + contracts.
pub mod reporting {
    pub use bill8_reporting_contracts as contracts;
    mod sdk;
    pub use sdk::{ReportingClient, ReportingError};
}

/// The cross-service wire conventions — the uniform error envelope,
/// cursor pagination shape, and shared header conventions (WS-4).
/// Import as `bill8::wire::{…}`.
pub mod wire {
    pub use bill8_wire_contracts as contracts;
}

// ── Shared client plumbing ─────────────────────────────────────────────

/// The uniform builder contract every service client implements.
pub trait ClientBuilder: Sized {
    /// Set the service base URL (trailing slash is stripped).
    fn with_base_url(self, url: impl Into<String>) -> Self;
    /// Attach a bearer token used for every request.
    fn with_token(self, token: impl Into<String>) -> Self;
    /// Provide a custom `reqwest::Client`.
    fn with_http_client(self, client: reqwest::Client) -> Self;
}

/// HTTP header used to propagate the correlation ID across services.
pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

static CORRELATION_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a new correlation ID (UUID v4, monotonic-entropy suffix so
/// same-microsecond IDs still order).
pub fn generate_correlation_id() -> String {
    let n = CORRELATION_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{:06}", uuid::Uuid::new_v4(), n)
}

/// Prepare a request: method + URL + correlation header + bearer token.
pub(crate) fn prepare_request(
    http: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    token: Option<&str>,
) -> reqwest::RequestBuilder {
    let req = http.request(method, url);
    let req = req.header(CORRELATION_ID_HEADER, generate_correlation_id());
    if let Some(t) = token {
        req.bearer_auth(t)
    } else {
        req
    }
}

/// The transport-level error every service client surfaces.
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    /// The HTTP transport failed (connect/timeout).
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),
    /// The response body did not decode into the contract type.
    #[error("decode error: {0}")]
    Decode(String),
}

impl SdkError {
    /// The HTTP status of the failed response, when one arrived.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            SdkError::Transport(e) => e.status(),
            SdkError::Decode(_) => None,
        }
    }
}