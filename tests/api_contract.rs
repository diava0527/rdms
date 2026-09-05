//! B 层独立测试：只使用无效请求，确保不会进入 A 尚未实现的业务函数。
use rdms::{api::create_router, config::Config};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn request(addr: std::net::SocketAddr, request: &str) -> String {
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut response = Vec::new();
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        stream.read_to_end(&mut response),
    )
    .await
    .unwrap()
    .unwrap();
    String::from_utf8(response).unwrap()
}

#[tokio::test]
async fn routing_rejections_static_files_and_cors() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_lazy("sqlite::memory:")
        .unwrap();
    let config = Config {
        bind_addr: "127.0.0.1:0".into(),
        database_url: "sqlite::memory:".into(),
        static_dir: format!("{}/frontend", env!("CARGO_MANIFEST_DIR")),
    };
    let app = create_router(pool, &config);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let cases = [
        ("GET", "/api/missing", "", 404),
        ("PATCH", "/api/users", "", 405),
        ("GET", "/api/users/abc", "", 400),
        ("GET", "/api/projects/abc", "", 400),
        ("DELETE", "/api/tasks/abc", "", 400),
        ("GET", "/api/projects/abc/cost-summary", "", 400),
        ("GET", "/api/tasks?project_id=abc", "", 400),
        ("GET", "/api/attendance?user_id=abc", "", 400),
        ("GET", "/api/attendance?task_id=abc", "", 400),
        ("GET", "/api/budgets", "", 400),
        ("GET", "/api/budgets?project_id=abc", "", 400),
        ("POST", "/api/users", "{", 400),
        ("POST", "/api/users", "{}", 422),
        ("POST", "/api/projects", "{}", 422),
        ("POST", "/api/tasks", "{}", 422),
        ("POST", "/api/attendance", "{}", 422),
        ("POST", "/api/budgets", "{}", 422),
        ("PUT", "/api/users/1", "{", 400),
        ("PUT", "/api/projects/1", "{", 400),
        ("PUT", "/api/tasks/1", "{", 400),
    ];
    for (method, path, body, status) in cases {
        let response = request(addr, &format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}", body.len()
        )).await;
        assert!(
            response.starts_with(&format!("HTTP/1.1 {status}")),
            "{method} {path}: {response}"
        );
        assert!(
            response.contains("content-type: application/json"),
            "{response}"
        );
        let body = response.split_once("\r\n\r\n").unwrap().1;
        let json: serde_json::Value = serde_json::from_str(body).unwrap();
        assert!(json["error"].is_string(), "{response}");
    }
    let response = request(addr, "POST /api/users HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Length: 2\r\n\r\n{}").await;
    assert!(response.starts_with("HTTP/1.1 415"), "{response}");
    assert!(response.contains("\"error\""));
    for path in ["/", "/js/api.js", "/css/style.css"] {
        let response = request(
            addr,
            &format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
        )
        .await;
        assert!(response.starts_with("HTTP/1.1 200"), "{path}: {response}");
    }
    let response = request(addr, "OPTIONS /api/users HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nOrigin: http://localhost:3000\r\nAccess-Control-Request-Method: POST\r\nAccess-Control-Request-Headers: content-type\r\n\r\n").await;
    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
    assert!(
        response.contains("access-control-allow-origin: *"),
        "{response}"
    );
    server.abort();
}
