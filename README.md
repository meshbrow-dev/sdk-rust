# Meshbrow Rust SDK

The official Rust SDK for [Meshbrow](https://meshbrow.dev) — Managed Browser Fleet for AI Agents.

## Installation

```toml
[dependencies]
meshbrow = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use meshbrow::{Client, CreateSessionParams, ProxyParams};

#[tokio::main]
async fn main() -> Result<(), meshbrow::Error> {
    let client = Client::new("your-api-key");

    // Launch a stealth browser session
    let session = client.create_session(Some(CreateSessionParams {
        stealth: Some("max".into()),
        proxy: Some(ProxyParams {
            proxy_type: "residential".into(),
            country: Some("US".into()),
        }),
        profile_id: None,
        viewport: None,
    })).await?;

    // Navigate and extract
    client.navigate(&session.id, "https://example.com", None).await?;
    let content = client.extract(&session.id, None, None).await?;
    println!("{}", content.text);

    // Screenshot
    let screenshot = client.screenshot(&session.id, None, false).await?;
    println!("Screenshot: {} bytes (base64)", screenshot.data.len());

    // Clean up
    client.destroy_session(&session.id, false).await?;
    Ok(())
}
```

## Fleet Operations

```rust
use meshbrow::{Client, CreateFleetParams, ProxyParams};

let fleet = client.create_fleet(CreateFleetParams {
    count: 10,
    stealth: Some("max".into()),
    proxy: Some(ProxyParams {
        proxy_type: "residential".into(),
        country: Some("US".into()),
    }),
}).await?;

for session in &fleet.sessions {
    client.navigate(&session.id, "https://example.com", None).await?;
}

client.destroy_fleet(&fleet.id).await?;
```

## License

MIT
