use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::api::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub role: String, // user role
    pub exp: usize,   // expiration time
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let app_state = parts.extensions.get::<Arc<AppState>>().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error - missing app state"})),
            )
                .into_response()
        })?;

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Missing authorization header"})),
                )
                    .into_response()
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid authorization header format"})),
            )
                .into_response());
        }

        let token = &auth_header["Bearer ".len()..];
        let key = &app_state.config.jwt_secret;
        let claims = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => token_data.claims,
            Err(_) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid token"})),
                )
                    .into_response())
            }
        };

        Ok(AuthUser {
            user_id: uuid::Uuid::parse_str(&claims.sub).unwrap(),
            role: claims.role,
        })
    }
}
