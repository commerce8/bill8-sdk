// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # bill8-reporting-contracts
//!
//! Wire types for the Bill8:Reporting HTTP API - report types and
//! report runs. Output files are Storage references, never inline content.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A versioned report definition.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ReportType {
    /// Report-type id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// Machine name (`charges_summary` / `refunds_summary` /
    /// `payouts_summary` / `balance_history`).
    pub name: String,
    /// Definition version.
    pub version: i32,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// An execution of a report type for a period.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ReportRun {
    /// Run id.
    pub id: Uuid,
    /// Owning tenant environment.
    pub env_id: Uuid,
    /// The executed report type.
    pub report_type_id: Uuid,
    /// Reporting period start.
    pub period_start: DateTime<Utc>,
    /// Reporting period end.
    pub period_end: DateTime<Utc>,
    /// Lifecycle state (`pending` / `succeeded` / `failed`).
    pub status: String,
    /// Output-file Storage reference (never inline content).
    pub output_ref: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request body for `POST /v1/report-runs`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateReportRunRequest {
    /// The report type to run.
    pub report_type_id: Uuid,
    /// Reporting period start.
    pub period_start: DateTime<Utc>,
    /// Reporting period end.
    pub period_end: DateTime<Utc>,
    /// Caller-supplied idempotency key (a run is idempotent per
    /// `(env_id, report_type, period)`).
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_run_roundtrip() {
        let run = ReportRun {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            report_type_id: Uuid::new_v4(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            status: "succeeded".into(),
            output_ref: Some("storage://reports/run-1.csv".into()),
            created_at: Utc::now(),
        };
        let json = serde_json::to_value(&run).unwrap();
        assert_eq!(json["status"], "succeeded");
        assert!(json["output_ref"].is_string());
        let de: ReportRun = serde_json::from_value(json).unwrap();
        assert_eq!(
            de.output_ref.as_deref(),
            Some("storage://reports/run-1.csv")
        );
    }

    #[test]
    fn report_run_request_parses() {
        let json = serde_json::json!({
            "report_type_id": Uuid::new_v4(),
            "period_start": "2026-09-01T00:00:00Z",
            "period_end": "2026-09-30T00:00:00Z",
            "idempotency_key": "sep-2026"
        });
        let req: CreateReportRunRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.idempotency_key.as_deref(), Some("sep-2026"));
    }
}
