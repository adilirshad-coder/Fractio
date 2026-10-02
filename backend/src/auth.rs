use axum::{extract::{Request, State}, http::header::AUTHORIZATION, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

use crate::{api::AppState, errors::ApiError};

#[derive(Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    #[serde(default)]
    iss: Option<String>,
    #[serde(default)]
    aud: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct User {
    pub subject: String,
    pub roles: Vec<String>,
}

pub async fn authenticate(State(state): State<AppState>, mut request: Request, next: Next) -> Result<Response, ApiError> {
    if request.uri().path().starts_with("/health") {
        return Ok(next.run(request).await);
    }
    let header = request.headers().get(AUTHORIZATION).and_then(|value| value.to_str().ok()).ok_or(ApiError::Unauthorized)?;
    let token = header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
    let algorithm = match state.settings.jwt_algorithm.as_str() {
        "HS256" => Algorithm::HS256,
        "RS256" => Algorithm::RS256,
        "ES256" => Algorithm::ES256,
        "EdDSA" => Algorithm::EdDSA,
        _ => return Err(ApiError::Unauthorized),
    };
    let key = match algorithm {
        Algorithm::HS256 => DecodingKey::from_secret(state.settings.jwt_secret.as_bytes()),
        Algorithm::RS256 => DecodingKey::from_rsa_pem(state.settings.jwt_public_key.as_bytes()).map_err(|_| ApiError::Unauthorized)?,
        Algorithm::ES256 => DecodingKey::from_ec_pem(state.settings.jwt_public_key.as_bytes()).map_err(|_| ApiError::Unauthorized)?,
        Algorithm::EdDSA => DecodingKey::from_ed_pem(state.settings.jwt_public_key.as_bytes()).map_err(|_| ApiError::Unauthorized)?,
        _ => return Err(ApiError::Unauthorized),
    };
    let mut validation = Validation::new(algorithm);
    if !state.settings.jwt_issuer.is_empty() { validation.set_issuer(&[state.settings.jwt_issuer.as_str()]); }
    if !state.settings.jwt_audience.is_empty() { validation.set_audience(&[state.settings.jwt_audience.as_str()]); }
    let claims = decode::<Claims>(token, &key, &validation).map_err(|_| ApiError::Unauthorized)?.claims;
    if claims.sub.is_empty() || claims.exp == 0 { return Err(ApiError::Unauthorized); }
    let _issuer = claims.iss;
    let _audience = claims.aud;
    let subject = claims.sub;
    let roles = state.roles.roles_for(&subject).await.map_err(|_| ApiError::Unauthorized)?;
    request.extensions_mut().insert(User { subject, roles });
    Ok(next.run(request).await)
}

pub fn require_role(user: &User, role: &str) -> Result<(), crate::errors::ApiError> {
    if user.roles.iter().any(|assigned| assigned == role) {
        Ok(())
    } else {
        Err(crate::errors::ApiError::Forbidden)
    }
}
