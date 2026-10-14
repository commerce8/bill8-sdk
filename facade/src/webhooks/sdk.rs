// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! Webhooks SDK client — typed methods over the Bill8:Webhooks REST API.

use bill8_webhooks_contracts as contracts;

use crate::{prepare_request, ClientBuilder, SdkError};

/// Error surface for the webhooks client.
#[derive(Debug, thiserror::Error)]
pub enum WebhooksError {
    /// Transport or decode failure.
    #[error(transparent)]
    Sdk(#[from] SdkError),
}

/// Typed async client over the Bill8:Webhooks REST surface.
#[derive(Clone)]
pub struct WebhooksClient {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl WebhooksClient {
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

    /// `POST /v1/webhook-endpoints` — register an endpoint.
    pub async fn create_endpoint(
        &self,
        req: contracts::CreateWebhookEndpointRequest,
    ) -> Result<contracts::WebhookEndpoint, WebhooksError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/webhook-endpoints", Some(req)).await?)
    }

    /// `GET /v1/events` — list materialized events.
    pub async fn list_events(&self) -> Result<Vec<contracts::Event>, WebhooksError> {
        Ok(self.send_json::<Vec<contracts::Event>>(reqwest::Method::GET, "/v1/events", None::<()>).await?)
    }
}

impl ClientBuilder for WebhooksClient {
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