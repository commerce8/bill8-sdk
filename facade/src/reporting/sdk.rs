// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! Reporting SDK client — typed methods over the Bill8:Reporting REST API.

use bill8_reporting_contracts as contracts;

use crate::{prepare_request, ClientBuilder, SdkError};

/// Error surface for the reporting client.
#[derive(Debug, thiserror::Error)]
pub enum ReportingError {
    /// Transport or decode failure.
    #[error(transparent)]
    Sdk(#[from] SdkError),
}

/// Typed async client over the Bill8:Reporting REST surface.
#[derive(Clone)]
pub struct ReportingClient {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl ReportingClient {
    /// Create a client for the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            token: None,
        }
    }

    async fn send_json<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<impl serde::Serialize>,
    ) -> Result<T, SdkError> {
        let mut req = prepare_request(&self.http, method, &format!("{}{}", self.base_url, path), self.token.as_deref());
        if let Some(b) = body {
            req = req.json(&b);
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.map_err(SdkError::Transport)?;
        if !status.is_success() {
            return Err(SdkError::Decode(format!("HTTP {status}: {text}")));
        }
        serde_json::from_str(&text).map_err(|e| SdkError::Decode(format!("{e}: {text}")))
    }

    /// `POST /v1/report-runs` — run a report.
    pub async fn create_report_run(
        &self,
        req: contracts::CreateReportRunRequest,
    ) -> Result<contracts::ReportRun, ReportingError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/report-runs", Some(req)).await?)
    }

    /// `GET /v1/report-types` — list report types.
    pub async fn list_report_types(&self) -> Result<Vec<contracts::ReportType>, ReportingError> {
        Ok(self.send_json::<Vec<contracts::ReportType>>(reqwest::Method::GET, "/v1/report-types", None::<()>).await?)
    }
}

impl ClientBuilder for ReportingClient {
    fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }
    fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
    fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http = client;
        self
    }
}