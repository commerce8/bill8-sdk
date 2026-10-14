// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bill8:SDK
// SPDX-License-Identifier: Apache-2.0

//! Payments SDK client — typed methods over the Bill8:Payments REST API.

use bill8_payments_contracts as contracts;

use crate::{prepare_request, ClientBuilder, SdkError};

/// Error surface for the payments client.
#[derive(Debug, thiserror::Error)]
pub enum PaymentsError {
    /// Transport or decode failure.
    #[error(transparent)]
    Sdk(#[from] SdkError),
}

/// Typed async client over the Bill8:Payments REST surface.
#[derive(Clone)]
pub struct PaymentsClient {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl PaymentsClient {
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

    /// `POST /v1/customers` — create a customer.
    pub async fn create_customer(
        &self,
        req: contracts::CreateCustomerRequest,
    ) -> Result<contracts::Customer, PaymentsError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/customers", Some(req)).await?)
    }

    /// `POST /v1/payment-intents` — create a payment intent.
    pub async fn create_intent(
        &self,
        req: contracts::CreatePaymentIntentRequest,
    ) -> Result<contracts::PaymentIntent, PaymentsError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/payment-intents", Some(req)).await?)
    }

    /// `POST /v1/charges` — create a charge.
    pub async fn create_charge(
        &self,
        req: contracts::CreateChargeRequest,
    ) -> Result<contracts::Charge, PaymentsError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/charges", Some(req)).await?)
    }

    /// `GET /v1/charges` — list charges.
    pub async fn list_charges(&self) -> Result<Vec<contracts::Charge>, PaymentsError> {
        Ok(self.send_json::<Vec<contracts::Charge>>(reqwest::Method::GET, "/v1/charges", None::<()>).await?)
    }

    /// `POST /v1/refunds` — refund a charge.
    pub async fn create_refund(
        &self,
        req: contracts::CreateRefundRequest,
    ) -> Result<contracts::Refund, PaymentsError> {
        Ok(self.send_json(reqwest::Method::POST, "/v1/refunds", Some(req)).await?)
    }
}

impl ClientBuilder for PaymentsClient {
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