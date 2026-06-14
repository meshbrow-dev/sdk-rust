//! Meshbrow Rust SDK — Managed Browser Fleet for AI Agents.
//!
//! ```rust,no_run
//! use meshbrow::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), meshbrow::Error> {
//!     let client = Client::new("your-api-key");
//!     let session = client.create_session(None).await?;
//!     client.navigate(&session.id, "https://example.com", None).await?;
//!     client.destroy_session(&session.id, false).await?;
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod types;

pub use client::Client;
pub use error::Error;
pub use types::*;
