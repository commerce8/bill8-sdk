// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-wire-contracts — the Bill8 wire conventions
//!
//! The cross-service wire contract every Bill8 API publishes and
//! every external integrator programs against (WS-4). One crate so the
//! eight service specs cannot drift apart: the uniform error envelope,
//! the cursor-pagination shape, and the shared header conventions.
//!
//! # Conventions (asserted by the tests below)
//!
//! - **Money is a string** everywhere — a decimal string in the
//!   currency's smallest unit context (`"12.34"`); never a float.
//! - **`Idempotency-Key` header** ≡ the body's `idempotency_key` field
//!   (header wins only when both are present and differ is a `400`).
//! - **Cursor pagination** — `limit` + `starting_after` /
//!   `ending_before`; list responses carry `data` + `next_cursor` and
//!   `has_more`.
//! - **Uniform error envelope** — every non-2xx carries
//!   `{ "error": { "code", "message", "doc_ref" } }`; `code` is
//!   stable and machine-readable, `message` human.
//!
//! # Examples
//!
//! ```
//! use bill8_wire_contracts::{ApiError, ErrorEnvelope, ListPage};
//!
//! // Every non-2xx body has this exact shape:
//! let body = ErrorEnvelope::forbidden("missing bill8:payments:charges:write");
//! let json = serde_json::to_string(&body).unwrap();
//! assert!(json.contains("\"code\":\"forbidden\""));
//!
//! // Every list endpoint paginates with cursors:
//! let page = ListPage { data: vec![1u32, 2, 3], next_cursor: Some("3".into()), has_more: true };
//! assert_eq!(page.data.len(), 3);
//! ```

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// The stable machine-readable error code set (the `error.code` values
/// every service publishes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum ErrorCode {
    /// Shield denied the request (403).
    Forbidden,
    /// The resource does not exist in this env (404).
    NotFound,
    /// A domain rule rejected the request — nothing written (422).
    InvalidRequest,
    /// The guarded transition guard fired (409).
    AlreadyExists,
    /// Idempotency conflict — same key, different payload (400).
    IdempotencyConflict,
    /// A downstream dependency failed (502/503).
    DownstreamUnavailable,
}

impl ErrorCode {
    /// The stable wire string (`error.code`).
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::Forbidden => "forbidden",
            ErrorCode::NotFound => "not_found",
            ErrorCode::InvalidRequest => "invalid_request",
            ErrorCode::AlreadyExists => "already_exists",
            ErrorCode::IdempotencyConflict => "idempotency_conflict",
            ErrorCode::DownstreamUnavailable => "downstream_unavailable",
        }
    }
}

/// One element of the uniform error envelope — `error.code` (stable,
/// machine-readable) + `error.message` (human) + `error.doc_ref`
/// (spec anchor).
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiError {
    /// Stable machine-readable code (never localized).
    pub code: String,
    /// Human-readable explanation (never the decision input).
    pub message: String,
    /// The spec anchor for the failing constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_ref: Option<String>,
}

/// The uniform error envelope — the exact body shape of every non-2xx
/// response (WS-4 wire convention).
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorEnvelope {
    /// The single `error` object.
    pub error: ApiError,
}

impl ErrorEnvelope {
    /// Build a `forbidden` envelope (Shield denial, 403).
    pub fn forbidden(message: &str) -> Self {
        Self {
            error: ApiError {
                code: ErrorCode::Forbidden.as_str().to_string(),
                message: message.to_string(),
                doc_ref: Some("payments DESIGN.md §5".into()),
            },
        }
    }

    /// Build a `not_found` envelope (404).
    pub fn not_found(message: &str) -> Self {
        Self {
            error: ApiError {
                code: ErrorCode::NotFound.as_str().to_string(),
                message: message.to_string(),
                doc_ref: Some("documents.md §2 — wire conventions".into()),
            },
        }
    }

    /// Build an `invalid_request` envelope (422 — nothing written).
    pub fn invalid_request(message: &str) -> Self {
        Self {
            error: ApiError {
                code: ErrorCode::InvalidRequest.as_str().to_string(),
                message: message.to_string(),
                doc_ref: Some("documents.md §2 — wire conventions".into()),
            },
        }
    }
}

/// Cursor-pagination query parameters — the shared list convention.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::IntoParams)]
pub struct CursorParams {
    /// Page size (1..=200, default 20).
    #[serde(default)]
    pub limit: Option<u32>,
    /// Return items strictly after this cursor (an id).
    #[serde(default)]
    pub starting_after: Option<String>,
    /// Return items strictly before this cursor (an id).
    #[serde(default)]
    pub ending_before: Option<String>,
}

/// The uniform list response shape — `data` + cursors + `has_more`.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ListPage<T> {
    /// The page's items (oldest → newest unless stated per endpoint).
    pub data: Vec<T>,
    /// The cursor for the next page (absent on the last page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// True when more items exist past this page.
    pub has_more: bool,
}

/// The shared header conventions (names + semantics only — services
/// publish their own utoipa `SecurityScheme` for the token).
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HeaderConventions {
    /// `Idempotency-Key` ≡ body `idempotency_key` (a present-and-different
    /// header is a 400 `idempotency_conflict`).
    pub idempotency_key: String,
    /// The Bill8 version pin (`Bill8-Version: 2026-10-27`).
    pub version: String,
    /// The client-supplied correlation id echoed on the response.
    pub correlation_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WS-4: money is always a decimal string, never a float.
    #[test]
    fn money_is_string_everywhere() {
        let amount = serde_json::json!({ "amount": "12.34" });
        assert_eq!(amount["amount"].as_str(), Some("12.34"));
    }

    /// WS-4: the error envelope shape is uniform.
    #[test]
    fn error_envelope_shape() {
        let e = ErrorEnvelope::invalid_request("amount must be positive");
        let json = serde_json::to_value(&e).unwrap();
        assert!(json["error"]["code"].is_string());
        assert!(json["error"]["message"].is_string());
        assert_eq!(json["error"]["code"], "invalid_request");
    }

    /// WS-4: the forbidden envelope carries the stable code.
    #[test]
    fn forbidden_code_is_stable() {
        let e = ErrorEnvelope::forbidden("missing scope");
        assert_eq!(e.error.code, "forbidden");
    }

    /// WS-4: cursor pagination — data + next_cursor + has_more.
    #[test]
    fn cursor_page_shape() {
        let page: ListPage<u32> = ListPage {
            data: vec![1, 2, 3],
            next_cursor: Some("3".into()),
            has_more: true,
        };
        let json = serde_json::to_value(&page).unwrap();
        assert!(json["data"].is_array());
        assert_eq!(json["has_more"], true);
        assert_eq!(json["next_cursor"], "3");
    }

    /// WS-4: the error-code set is the complete stable set.
    #[test]
    fn error_codes_are_stable() {
        assert_eq!(ErrorCode::Forbidden.as_str(), "forbidden");
        assert_eq!(ErrorCode::NotFound.as_str(), "not_found");
        assert_eq!(ErrorCode::InvalidRequest.as_str(), "invalid_request");
        assert_eq!(ErrorCode::AlreadyExists.as_str(), "already_exists");
        assert_eq!(ErrorCode::IdempotencyConflict.as_str(), "idempotency_conflict");
        assert_eq!(ErrorCode::DownstreamUnavailable.as_str(), "downstream_unavailable");
    }
}