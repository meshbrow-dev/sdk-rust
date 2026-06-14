#[cfg(test)]
mod tests {
    use meshbrow::*;
    use meshbrow::Client;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn test_client(server: &MockServer) -> Client {
        Client::with_base_url("test-api-key", &server.uri())
    }

    #[tokio::test]
    async fn test_create_session_basic() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions"))
            .and(header("Authorization", "Bearer test-api-key"))
            .and(header("Content-Type", "application/json"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(json!({
                    "data": {
                        "id": "mb_test1",
                        "status": "ready",
                        "cdp_endpoint": "wss://api.meshbrow.dev/cdp/mb_test1",
                        "token": "tok1",
                        "created_at": "2026-06-14T00:00:00Z",
                        "expires_at": "2026-06-14T01:00:00Z"
                    }
                })),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let session = client.create_session(None).await.unwrap();

        assert_eq!(session.id, "mb_test1");
        assert_eq!(session.status, "ready");
        assert_eq!(session.cdp_endpoint, "wss://api.meshbrow.dev/cdp/mb_test1");
        assert_eq!(session.token, "tok1");
    }

    #[tokio::test]
    async fn test_create_session_with_proxy() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions"))
            .and(body_json(json!({
                "stealth": "max",
                "proxy": {"type": "residential", "country": "US"}
            })))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(json!({
                    "data": {"id": "mb_proxy1", "status": "ready"}
                })),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let session = client
            .create_session(Some(CreateSessionParams {
                stealth: Some("max".into()),
                proxy: Some(ProxyParams {
                    proxy_type: "residential".into(),
                    country: Some("US".into()),
                }),
                profile_id: None,
                viewport: None,
            }))
            .await
            .unwrap();

        assert_eq!(session.id, "mb_proxy1");
    }

    #[tokio::test]
    async fn test_get_session() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/sessions/mb_abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "id": "mb_abc123",
                    "status": "ready",
                    "stealth": "max",
                    "created_at": "2026-06-14T00:00:00Z"
                }
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let session = client.get_session("mb_abc123").await.unwrap();

        assert_eq!(session.id, "mb_abc123");
        assert_eq!(session.stealth, "max");
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/sessions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "sessions": [
                    {"id": "mb_1", "status": "ready"},
                    {"id": "mb_2", "status": "ready"}
                ],
                "metrics": {"active_sessions": 2}
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let resp = client.list_sessions().await.unwrap();

        assert_eq!(resp.sessions.len(), 2);
        assert_eq!(resp.sessions[0].id, "mb_1");
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/sessions/mb_destroy1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        client.destroy_session("mb_destroy1", false).await.unwrap();
    }

    #[tokio::test]
    async fn test_navigate() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/navigate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {"url": "https://example.com", "title": "Example Domain", "status": 200}
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let resp = client
            .navigate("mb_1", "https://example.com", None)
            .await
            .unwrap();

        assert_eq!(resp.url, "https://example.com");
        assert_eq!(resp.title, "Example Domain");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_screenshot() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/screenshot"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {"data": "iVBORw0KGgo=", "format": "png"}
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let screenshot = client.screenshot("mb_1", None, false).await.unwrap();

        assert_eq!(screenshot.data, "iVBORw0KGgo=");
        assert_eq!(screenshot.format, "png");
    }

    #[tokio::test]
    async fn test_click() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/click"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {"clicked": true}})))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        client.click("mb_1", "button.submit").await.unwrap();
    }

    #[tokio::test]
    async fn test_type_text() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/type"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {"typed": true}})))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        client
            .type_text("mb_1", "input#email", "test@example.com", true)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_extract() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/extract"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"data": {"text": "Hello World"}})),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let resp = client.extract("mb_1", None, None).await.unwrap();

        assert_eq!(resp.text, "Hello World");
    }

    #[tokio::test]
    async fn test_execute() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sessions/mb_1/execute"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"data": {"result": 42}})),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let resp = client.execute("mb_1", "return 21 * 2").await.unwrap();

        assert_eq!(resp.result, json!(42));
    }

    #[tokio::test]
    async fn test_create_fleet() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/fleet"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "data": {
                    "id": "fleet_1",
                    "status": "ready",
                    "sessions": [
                        {"id": "mb_1", "status": "ready"},
                        {"id": "mb_2", "status": "ready"}
                    ],
                    "count": 2
                }
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let fleet = client
            .create_fleet(CreateFleetParams {
                count: 5,
                stealth: Some("max".into()),
                proxy: Some(ProxyParams {
                    proxy_type: "residential".into(),
                    country: Some("DE".into()),
                }),
            })
            .await
            .unwrap();

        assert_eq!(fleet.id, "fleet_1");
        assert_eq!(fleet.sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_get_fleet() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/fleet/fleet_abc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "id": "fleet_abc",
                    "status": "ready",
                    "sessions": [{"id": "mb_1", "status": "ready"}],
                    "count": 1
                }
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let fleet = client.get_fleet("fleet_abc").await.unwrap();

        assert_eq!(fleet.id, "fleet_abc");
        assert_eq!(fleet.count, 1);
    }

    #[tokio::test]
    async fn test_destroy_fleet() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/fleet/fleet_abc"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        client.destroy_fleet("fleet_abc").await.unwrap();
    }

    #[tokio::test]
    async fn test_api_error_401() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/sessions/mb_1"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(json!({"error": {"code": "unauthorized", "message": "Invalid API key"}})),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let result = client.get_session("mb_1").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            meshbrow::Error::Api { status, .. } => assert_eq!(status, 401),
            _ => panic!("expected Api error, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_api_error_404() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/sessions/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(json!({"error": {"code": "not_found", "message": "session not found"}})),
            )
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let result = client.get_session("nonexistent").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            meshbrow::Error::Api { status, .. } => assert_eq!(status, 404),
            _ => panic!("expected Api error, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_user_agent() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/sessions/mb_1"))
            .and(header("User-Agent", "meshbrow-rust/0.1.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {"id": "mb_1", "status": "ready"}
            })))
            .mount(&server)
            .await;

        let client = test_client(&server).await;
        let session = client.get_session("mb_1").await.unwrap();
        assert_eq!(session.id, "mb_1");
    }
}
