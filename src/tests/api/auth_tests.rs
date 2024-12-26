use crate::{
    api::auth::{LoginRequest, RegisterRequest},
    models::user::UserRole,
    tests::setup,
};

#[tokio::test]
async fn test_register_and_login() {
    let (_, pool) = setup();

    // 测试注册
    let register_req = RegisterRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let response = crate::api::auth::register(
        axum::extract::State(pool.clone()),
        axum::Json(register_req),
    )
    .await;

    assert!(response.is_ok());
    let register_response = response.unwrap();
    assert_eq!(register_response.0.user.email, "test@example.com");
    assert_eq!(register_response.0.user.name, "Test User");
    assert_eq!(register_response.0.user.role, UserRole::User);

    // 测试登录
    let login_req = LoginRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let response = crate::api::auth::login(
        axum::extract::State(pool),
        axum::Json(login_req),
    )
    .await;

    assert!(response.is_ok());
    let login_response = response.unwrap();
    assert_eq!(login_response.0.user.email, "test@example.com");
} 