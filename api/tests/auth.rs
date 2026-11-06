use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use ob_api::{
    config::Config,
    routes::create_router,
    services::tokens::{TokenService, UserClaims},
    state::AppState,
};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
// #[ignore = "requires an isolated PostgreSQL database in API_TEST_DATABASE_URL"]
async fn signup_and_login_work_over_http() {
    let database_url = std::env::var("API_TEST_DATABASE_URL")
        .expect("set API_TEST_DATABASE_URL to a dedicated test database");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("connect to API_TEST_DATABASE_URL");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("apply API migrations to the test database");

    let config = Config {
        port: 0,
        db_url: database_url,
        jwt_secret: b"integration-test-secret".to_vec(),
    };
    let state = AppState::new(pool, config).await.expect("build app state");
    let app = create_router(state);
    let email = format!("{}@example.test", uuid::Uuid::new_v4());

    let signup = app
        .clone()
        .oneshot(json_request(
            "/auth/signup",
            json!({
                "first_name": "  Ada  ",
                "last_name": "  Lovelace  ",
                "other_name": null,
                "email": format!("  {email}  "),
                "password": "Strong!Pass1",
                "is_admin": true
            }),
        ))
        .await
        .unwrap();
    assert_eq!(signup.status(), StatusCode::CREATED);
    let signup_body = response_json(signup).await;
    assert_eq!(signup_body["first_name"], "ada");
    assert_eq!(signup_body["email"], email);

    let login = app
        .clone()
        .oneshot(json_request(
            "/auth/login",
            json!({"email": email, "password": "Strong!Pass1"}),
        ))
        .await
        .unwrap();
    assert_eq!(login.status(), StatusCode::OK);
    assert!(login.headers().contains_key(header::SET_COOKIE));
    let login_body = response_json(login).await;
    assert_eq!(login_body["memberships"], json!([]));
    let claims = TokenService::verify_token::<UserClaims>(
        login_body["user_token"].as_str().expect("user token"),
        b"integration-test-secret",
        "ogasbook-auth",
        "ogasbook-user",
    );
    assert!(claims.is_ok());

    let bad_login = app
        .oneshot(json_request(
            "/auth/login",
            json!({"email": email, "password": "Wrong!Pass1"}),
        ))
        .await
        .unwrap();
    assert_eq!(bad_login.status(), StatusCode::UNAUTHORIZED);
}

fn json_request(uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

async fn response_json(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}
