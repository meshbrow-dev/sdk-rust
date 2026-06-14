use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::json;

use crate::error::Error;
use crate::types::*;

/// Meshbrow API client.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

#[derive(Deserialize)]
struct Envelope {
    data: serde_json::Value,
}

impl Client {
    /// Create a new client with the given API key.
    pub fn new(api_key: &str) -> Self {
        Self::with_base_url(api_key, "https://api.meshbrow.dev")
    }

    /// Create a new client with a custom base URL.
    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("meshbrow-rust/0.1.0"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            http,
            base_url: base_url.to_string(),
        }
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, &url);
        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if status.as_u16() >= 400 {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status: status.as_u16(),
                body,
            });
        }

        let text = resp.text().await?;
        if text.is_empty() {
            return Err(Error::Api {
                status: 204,
                body: "empty response".into(),
            });
        }

        // Try to unwrap envelope.
        if let Ok(envelope) = serde_json::from_str::<Envelope>(&text) {
            if !envelope.data.is_null() {
                return Ok(serde_json::from_value(envelope.data)?);
            }
        }

        Ok(serde_json::from_str(&text)?)
    }

    async fn request_empty(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, &url);
        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if status.as_u16() >= 400 {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status: status.as_u16(),
                body,
            });
        }

        Ok(())
    }

    // --- Sessions ---

    /// Launch a new stealth browser session.
    pub async fn create_session(
        &self,
        params: Option<CreateSessionParams>,
    ) -> Result<Session, Error> {
        let body = match params {
            Some(p) => serde_json::to_value(p).unwrap_or(json!({"stealth": "max"})),
            None => json!({"stealth": "max"}),
        };
        self.request(reqwest::Method::POST, "/v1/sessions", Some(body))
            .await
    }

    /// Get session details.
    pub async fn get_session(&self, id: &str) -> Result<Session, Error> {
        self.request(reqwest::Method::GET, &format!("/v1/sessions/{}", id), None)
            .await
    }

    /// List all active sessions.
    pub async fn list_sessions(&self) -> Result<SessionListResponse, Error> {
        self.request(reqwest::Method::GET, "/v1/sessions", None)
            .await
    }

    /// Destroy a session.
    pub async fn destroy_session(&self, id: &str, save_profile: bool) -> Result<(), Error> {
        let body = if save_profile {
            Some(json!({"save_profile": true}))
        } else {
            None
        };
        self.request_empty(
            reqwest::Method::DELETE,
            &format!("/v1/sessions/{}", id),
            body,
        )
        .await
    }

    // --- Browser Actions ---

    /// Navigate the browser to a URL.
    pub async fn navigate(
        &self,
        session_id: &str,
        url: &str,
        wait_until: Option<&str>,
    ) -> Result<NavigateResponse, Error> {
        self.request(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/navigate", session_id),
            Some(json!({
                "url": url,
                "wait_until": wait_until.unwrap_or("load"),
            })),
        )
        .await
    }

    /// Take a screenshot.
    pub async fn screenshot(
        &self,
        session_id: &str,
        selector: Option<&str>,
        full_page: bool,
    ) -> Result<Screenshot, Error> {
        let mut body = json!({"full_page": full_page});
        if let Some(sel) = selector {
            body["selector"] = json!(sel);
        }
        self.request(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/screenshot", session_id),
            Some(body),
        )
        .await
    }

    /// Click an element.
    pub async fn click(&self, session_id: &str, selector: &str) -> Result<(), Error> {
        self.request_empty(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/click", session_id),
            Some(json!({"selector": selector})),
        )
        .await
    }

    /// Type text into an input.
    pub async fn type_text(
        &self,
        session_id: &str,
        selector: &str,
        text: &str,
        clear: bool,
    ) -> Result<(), Error> {
        self.request_empty(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/type", session_id),
            Some(json!({"selector": selector, "text": text, "clear": clear})),
        )
        .await
    }

    /// Extract text from the page.
    pub async fn extract(
        &self,
        session_id: &str,
        selector: Option<&str>,
        max_length: Option<u32>,
    ) -> Result<ExtractResponse, Error> {
        let mut body = json!({});
        if let Some(sel) = selector {
            body["selector"] = json!(sel);
        }
        if let Some(ml) = max_length {
            body["max_length"] = json!(ml);
        }
        self.request(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/extract", session_id),
            Some(body),
        )
        .await
    }

    /// Execute JavaScript.
    pub async fn execute(&self, session_id: &str, script: &str) -> Result<ExecuteResponse, Error> {
        self.request(
            reqwest::Method::POST,
            &format!("/v1/sessions/{}/execute", session_id),
            Some(json!({"script": script})),
        )
        .await
    }

    // --- Fleet ---

    /// Launch multiple sessions in parallel.
    pub async fn create_fleet(&self, params: CreateFleetParams) -> Result<Fleet, Error> {
        let body = serde_json::to_value(params).unwrap();
        self.request(reqwest::Method::POST, "/v1/fleet", Some(body))
            .await
    }

    /// Get fleet status.
    pub async fn get_fleet(&self, id: &str) -> Result<Fleet, Error> {
        self.request(reqwest::Method::GET, &format!("/v1/fleet/{}", id), None)
            .await
    }

    /// Destroy all sessions in a fleet.
    pub async fn destroy_fleet(&self, id: &str) -> Result<(), Error> {
        self.request_empty(reqwest::Method::DELETE, &format!("/v1/fleet/{}", id), None)
            .await
    }
}
