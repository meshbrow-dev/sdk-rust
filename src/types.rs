use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A browser session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub cdp_endpoint: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub stealth: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expires_at: String,
}

/// Parameters for creating a session.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
}

/// Proxy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyParams {
    #[serde(rename = "type")]
    pub proxy_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Viewport dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

/// Session list response with metrics.
#[derive(Debug, Clone, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<Session>,
    pub metrics: HashMap<String, serde_json::Value>,
}

/// A screenshot result.
#[derive(Debug, Clone, Deserialize)]
pub struct Screenshot {
    /// Base64-encoded PNG data.
    pub data: String,
    pub format: String,
}

/// Navigation response.
#[derive(Debug, Clone, Deserialize)]
pub struct NavigateResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: u16,
}

/// Text extraction response.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtractResponse {
    pub text: String,
}

/// JavaScript execution response.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteResponse {
    pub result: serde_json::Value,
}

/// A fleet of browser sessions.
#[derive(Debug, Clone, Deserialize)]
pub struct Fleet {
    pub id: String,
    pub status: String,
    pub sessions: Vec<Session>,
    #[serde(default)]
    pub count: u32,
}

/// Parameters for creating a fleet.
#[derive(Debug, Clone, Serialize)]
pub struct CreateFleetParams {
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyParams>,
}
