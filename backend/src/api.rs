use axum::{body::{to_bytes, Body}, extract::{Extension, Path, Query, Request, State}, http::{HeaderMap, HeaderValue, Method}, middleware::{self, Next}, response::Response, routing::{get, post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;
use crate::{auth::{authenticate, require_role, User}, config::Settings, errors::ApiError, ports::RoleRepository};
use std::sync::Arc;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Utc};

#[derive(Clone)] pub struct AppState { pub db: PgPool, pub settings: Settings, pub roles: Arc<dyn RoleRepository> }
#[derive(Serialize)] struct Envelope<T> { data: T, meta: serde_json::Value, request_id: String }
#[derive(Serialize)] struct Empty { status: &'static str }
#[derive(Deserialize, Validate)] struct ProjectRequest { #[validate(length(min=1,max=160))] name: String }
#[derive(Deserialize)] struct ProjectsQuery { cursor: Option<String>, limit: Option<u8>, status: Option<String> }
fn success<T: Serialize>(headers: &HeaderMap, data: T) -> Result<Json<Envelope<T>>, ApiError> {
    let request_id = headers.get("x-request-id").and_then(|h| h.to_str().ok()).filter(|s| !s.is_empty() && s.len() <= 128).map(str::to_owned).unwrap_or_else(|| Uuid::new_v4().to_string());
    Ok(Json(Envelope { data, meta: serde_json::json!({}), request_id }))
}
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(live)).route("/health/ready", get(ready))
        .route("/v1/register", post(register)).route("/v1/me", get(me))
        .route("/v1/kyc/lite", post(investor_action)).route("/v1/wallet", get(investor_action))
        .route("/v1/deposits", post(money_action)).route("/v1/withdrawals", post(money_action))
        .route("/v1/projects", get(projects).post(founder_project))
        .route("/v1/projects/{id}/invest", post(money_action)).route("/v1/projects/{id}/sell", post(money_action))
        .route("/v1/founders", post(founder_action)).route("/v1/founders/kyb", post(founder_action))
        .route("/v1/projects/{id}/landing-page", put(founder_action))
        .route("/v1/projects/{id}/listing", post(founder_action)).route("/v1/projects/{id}/updates", post(founder_action))
        .route("/v1/founders/creator-fees", get(founder_action)).route("/v1/founders/creator-fees/claim", post(founder_money_action))
        .route("/v1/projects/{id}/milestones", get(founder_action))
        .route("/v1/projects/{id}", get(project_read)).route("/v1/portfolio", get(investor_action))
        .route_layer(middleware::from_fn_with_state(state.clone(), authenticate))
        .layer(middleware::from_fn(request_ids)).with_state(state)
}

#[derive(Deserialize)]
struct RegisterBody { role: String, display_name: String, founder: Option<FounderRegistration> }
#[derive(Deserialize)]
struct FounderRegistration { entity_type: String, display_name: String }

async fn register(State(state): State<AppState>, Extension(user): Extension<User>, headers: HeaderMap, Json(body): Json<RegisterBody>) -> Result<Json<Envelope<serde_json::Value>>, ApiError> {
    if body.role == "admin" { return Err(ApiError::Forbidden); }
    if !["investor", "founder"].contains(&body.role.as_str()) { return Err(ApiError::Validation("role must be investor or founder".into())); }
    if body.display_name.trim().is_empty() || body.display_name.len() > 160 { return Err(ApiError::Validation("display_name must be 1 to 160 characters".into())); }
    if body.role == "founder" {
        let founder = body.founder.as_ref().ok_or_else(|| ApiError::Validation("founder details are required".into()))?;
        if !["sole_proprietor", "partner", "founder", "co_founder", "agentic_founder"].contains(&founder.entity_type.as_str()) || founder.display_name.trim().is_empty() || founder.display_name.len() > 160 {
            return Err(ApiError::Validation("invalid founder entity details".into()));
        }
    } else if body.founder.is_some() { return Err(ApiError::Validation("founder details are only accepted for founder registration".into())); }

    let mut tx = state.db.begin().await.map_err(|e| ApiError::Internal(e.into()))?;
    sqlx::query("INSERT INTO users(auth_subject, display_name) VALUES ($1,$2) ON CONFLICT(auth_subject) DO NOTHING")
        .bind(&user.subject).bind(&body.display_name).execute(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
    let (user_id, display_name): (Uuid, Option<String>) = sqlx::query_as("SELECT id, display_name FROM users WHERE auth_subject=$1")
        .bind(&user.subject).fetch_one(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
    sqlx::query("INSERT INTO user_roles(user_id,role) VALUES ($1,$2) ON CONFLICT(user_id,role) DO NOTHING")
        .bind(user_id).bind(&body.role).execute(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
    let founder_entity = if body.role == "investor" {
        sqlx::query("INSERT INTO investor_identities(user_id,kyc_level,eligibility_status) VALUES ($1,'none','unknown') ON CONFLICT(user_id) DO NOTHING")
            .bind(user_id).execute(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
        None
    } else {
        let details = body.founder.as_ref().expect("validated founder details");
        let existing: Option<Uuid> = sqlx::query_scalar("SELECT fm.entity_id FROM founder_memberships fm JOIN founder_entities fe ON fe.id=fm.entity_id WHERE fm.user_id=$1 AND fe.entity_type=$2 ORDER BY fm.created_at LIMIT 1")
            .bind(user_id).bind(&details.entity_type).fetch_optional(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
        let entity_id = if let Some(id) = existing { id } else {
            let id: Uuid = sqlx::query_scalar("INSERT INTO founder_entities(entity_type,display_name,kyb_status) VALUES ($1,$2,'none') RETURNING id")
                .bind(&details.entity_type).bind(&details.display_name).fetch_one(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
            sqlx::query("INSERT INTO founder_memberships(entity_id,user_id,role) VALUES ($1,$2,'owner') ON CONFLICT DO NOTHING")
                .bind(id).bind(user_id).execute(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
            id
        };
        Some(entity_id)
    };
    let request_id = headers.get("x-request-id").and_then(|h| h.to_str().ok()).unwrap_or_default();
    sqlx::query("INSERT INTO audit_logs(actor_id,action,target_type,target_id,request_id) VALUES ($1,'register','user',$2,$3)")
        .bind(user_id).bind(user_id.to_string()).bind(request_id).execute(&mut *tx).await.map_err(|e| ApiError::Internal(e.into()))?;
    tx.commit().await.map_err(|e| ApiError::Internal(e.into()))?;
    success(&headers, serde_json::json!({"id": user_id, "display_name": display_name.unwrap_or(body.display_name), "role": body.role, "founder_entity_id": founder_entity}))
}

async fn me(State(state): State<AppState>, Extension(user): Extension<User>, headers: HeaderMap) -> Result<Json<Envelope<serde_json::Value>>, ApiError> {
    let row: Option<(Uuid, Option<String>)> = sqlx::query_as("SELECT id,display_name FROM users WHERE auth_subject=$1")
        .bind(&user.subject).fetch_optional(&state.db).await.map_err(|e| ApiError::Internal(e.into()))?;
    let Some((id, display_name)) = row else { return Err(ApiError::Forbidden); };
    let roles = state.roles.roles_for(&user.subject).await.map_err(|e| ApiError::Internal(e.into()))?;
    let investor = sqlx::query("SELECT kyc_level,eligibility_status FROM investor_identities WHERE user_id=$1")
        .bind(id).fetch_optional(&state.db).await.map_err(|e| ApiError::Internal(e.into()))?;
    let investor = investor.map(|r| serde_json::json!({"kyc_level": r.get::<String,_>("kyc_level"), "eligibility_status": r.get::<String,_>("eligibility_status")}));
    let founders = sqlx::query("SELECT fe.id,fe.display_name,fe.kyb_status FROM founder_memberships fm JOIN founder_entities fe ON fe.id=fm.entity_id WHERE fm.user_id=$1 ORDER BY fe.created_at")
        .bind(id).fetch_all(&state.db).await.map_err(|e| ApiError::Internal(e.into()))?;
    let founders: Vec<_> = founders.into_iter().map(|r| serde_json::json!({"id": r.get::<Uuid,_>("id"), "display_name": r.get::<String,_>("display_name"), "kyb_status": r.get::<String,_>("kyb_status")})).collect();
    success(&headers, serde_json::json!({"id": id,"display_name": display_name,"roles": roles,"investor": investor,"founder_memberships": founders}))
}
async fn request_ids(mut req: Request, next: Next) -> Response {
    let id = req.headers().get("x-request-id").and_then(|v| v.to_str().ok()).filter(|v| !v.is_empty() && v.len() <= 128).map(str::to_owned).unwrap_or_else(|| Uuid::new_v4().to_string());
    if let Ok(value) = HeaderValue::from_str(&id) { req.headers_mut().insert("x-request-id", value); }
    let mut response = next.run(req).await;
    if let Ok(value) = HeaderValue::from_str(&id) { response.headers_mut().insert("x-request-id", value); }
    if response.status().is_client_error() || response.status().is_server_error() {
        let (mut parts, body) = response.into_parts();
        if let Ok(bytes) = to_bytes(body, 1024 * 1024).await {
            if let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                value["request_id"] = serde_json::Value::String(id);
                parts.headers.remove(axum::http::header::CONTENT_LENGTH);
                return Response::from_parts(parts, Body::from(serde_json::to_vec(&value).unwrap_or_default()));
            }
            parts.headers.remove(axum::http::header::CONTENT_LENGTH);
            parts.headers.insert(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            let payload = serde_json::json!({"error":{"code":"REQUEST_ERROR","message":"Request could not be processed"},"request_id":id});
            return Response::from_parts(parts, Body::from(serde_json::to_vec(&payload).unwrap_or_default()));
        }
        return Response::from_parts(parts, Body::empty());
    }
    response
}
async fn live(headers: HeaderMap) -> Result<Json<Envelope<Empty>>, ApiError> { success(&headers, Empty { status: "ok" }) }
async fn ready(State(s): State<AppState>, headers: HeaderMap) -> Result<Json<Envelope<Empty>>, ApiError> { sqlx::query("SELECT 1").execute(&s.db).await.map_err(|e| ApiError::Internal(e.into()))?; success(&headers, Empty { status: "ok" }) }
async fn not_implemented() -> Result<Json<Envelope<Empty>>, ApiError> { Err(ApiError::NotImplemented) }
async fn investor_action(Extension(user): Extension<User>, method: Method, body: Option<Json<serde_json::Value>>) -> Result<Json<Envelope<Empty>>, ApiError> {
    require_role(&user, "investor")?;
    if method != Method::GET && body.is_none() { return Err(ApiError::Validation("request body is required".into())); }
    if body.as_ref().is_some_and(|Json(value)| !value.is_object()) { return Err(ApiError::Validation("request body must be an object".into())); }
    tracing::debug!(actor=%user.subject,"investor route requested"); not_implemented().await
}
async fn founder_action(Extension(user): Extension<User>, method: Method, body: Option<Json<serde_json::Value>>) -> Result<Json<Envelope<Empty>>, ApiError> {
    require_role(&user, "founder")?;
    if method != Method::GET && body.is_none() { return Err(ApiError::Validation("request body is required".into())); }
    if body.as_ref().is_some_and(|Json(value)| !value.is_object()) { return Err(ApiError::Validation("request body must be an object".into())); }
    tracing::debug!(actor=%user.subject,"founder route requested"); not_implemented().await
}
async fn founder_project(Extension(user): Extension<User>, Json(request): Json<ProjectRequest>) -> Result<Json<Envelope<Empty>>, ApiError> { require_role(&user, "founder")?; request.validate().map_err(|e| ApiError::Validation(e.to_string()))?; let _name = request.name; not_implemented().await }
async fn money_action(Extension(user): Extension<User>, headers: HeaderMap, body: Option<Json<serde_json::Value>>) -> Result<Json<Envelope<Empty>>, ApiError> {
    require_role(&user, "investor")?;
    money_action_inner(user, headers, body, true).await
}
async fn founder_money_action(Extension(user): Extension<User>, headers: HeaderMap, body: Option<Json<serde_json::Value>>) -> Result<Json<Envelope<Empty>>, ApiError> {
    require_role(&user, "founder")?;
    money_action_inner(user, headers, body, false).await
}
async fn money_action_inner(user: User, headers: HeaderMap, body: Option<Json<serde_json::Value>>, amount_required: bool) -> Result<Json<Envelope<Empty>>, ApiError> {
    tracing::debug!(actor=%user.subject,"money route requested");
    let body_key = body.as_ref().and_then(|Json(value)| value.get("idempotency_key")).and_then(serde_json::Value::as_str);
    let key = headers.get("idempotency-key").and_then(|v| v.to_str().ok()).or(body_key);
    let amount = body.as_ref().and_then(|Json(value)| value.get("amount_minor"));
    if amount_required && amount.is_none() { return Err(ApiError::Validation("amount_minor is required".into())); }
    if let Some(value) = amount { let amount = value.as_i64().ok_or_else(|| ApiError::Validation("amount_minor must be an integer".into()))?; if amount <= 0 { return Err(ApiError::Validation("amount must be positive".into())); } }
    if key.is_some_and(|v| v.is_empty() || v.len() > 128) { return Err(ApiError::Validation("Idempotency-Key must be 1 to 128 characters".into())); }
    if key.is_none() { return Err(ApiError::Validation("Idempotency-Key or idempotency_key is required".into())); }
    not_implemented().await
}
#[derive(Serialize)]
struct ProjectItem { id: Uuid, name: String, description: Option<String>, status: String, total_supply: Option<String>, created_at: DateTime<Utc> }
#[derive(Serialize)]
struct ProjectPage { items: Vec<ProjectItem>, next_cursor: Option<String> }
#[derive(Deserialize, Serialize)]
struct ProjectCursor { created_at: DateTime<Utc>, id: Uuid }

fn encode_cursor(cursor: &ProjectCursor) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_vec(cursor).expect("project cursor serialization"))
}

fn decode_cursor(value: &str) -> Result<ProjectCursor, ApiError> {
    let bytes = URL_SAFE_NO_PAD.decode(value).map_err(|_| ApiError::Validation("invalid cursor".into()))?;
    serde_json::from_slice::<ProjectCursor>(&bytes).map_err(|_| ApiError::Validation("invalid cursor".into()))
}

fn public_status_filter(filter: Option<&str>) -> Option<&'static [&'static str]> {
    match filter {
        Some("New") => Some(&["seed_live"]),
        Some("Bonding") => Some(&["bonding_live"]),
        Some("Graduated") => Some(&["graduated"]),
        Some("Explore") | None => None,
        _ => Some(&[]),
    }
}

async fn projects(State(state): State<AppState>, Extension(user): Extension<User>, headers: HeaderMap, Query(query): Query<ProjectsQuery>) -> Result<Json<Envelope<ProjectPage>>, ApiError> {
    let _authenticated_subject = user.subject;
    let limit = query.limit.unwrap_or(20);
    if limit == 0 || limit > 100 { return Err(ApiError::Validation("limit must be between 1 and 100".into())); }
    if query.cursor.as_ref().is_some_and(|c| c.is_empty() || c.len() > 256) { return Err(ApiError::Validation("cursor must be 1 to 256 characters".into())); }
    if query.status.as_ref().is_some_and(|s| !["Explore", "New", "Bonding", "Graduated"].contains(&s.as_str())) { return Err(ApiError::Validation("invalid project status filter".into())); }
    let cursor = query.cursor.as_deref().map(decode_cursor).transpose()?;
    let statuses: Vec<&str> = match public_status_filter(query.status.as_deref()) {
        Some(statuses) => statuses.to_vec(),
        None => vec!["verified", "seed_live", "seed_closed", "bonding_live", "raise_closed", "graduating", "graduated"],
    };
    if statuses.is_empty() { return Err(ApiError::Validation("invalid project status filter".into())); }
    let rows = if let Some(cursor) = cursor {
        sqlx::query_as::<_, (Uuid, String, Option<String>, String, Option<String>, DateTime<Utc>)>("SELECT id,name,description,status::text,total_supply::text,created_at FROM projects WHERE status::text = ANY($1) AND (created_at,id) < ($2,$3) ORDER BY created_at DESC,id DESC LIMIT $4")
            .bind(&statuses).bind(cursor.created_at).bind(cursor.id).bind(i64::from(limit) + 1).fetch_all(&state.db).await
    } else {
        sqlx::query_as::<_, (Uuid, String, Option<String>, String, Option<String>, DateTime<Utc>)>("SELECT id,name,description,status::text,total_supply::text,created_at FROM projects WHERE status::text = ANY($1) ORDER BY created_at DESC,id DESC LIMIT $2")
            .bind(&statuses).bind(i64::from(limit) + 1).fetch_all(&state.db).await
    }.map_err(|e| ApiError::Internal(e.into()))?;
    let has_more = rows.len() > usize::from(limit);
    let mut items: Vec<ProjectItem> = rows.into_iter().take(usize::from(limit)).map(|(id,name,description,status,total_supply,created_at)| ProjectItem { id,name,description,status,total_supply,created_at }).collect();
    let next_cursor = if has_more { items.last().map(|item| encode_cursor(&ProjectCursor { created_at: item.created_at, id: item.id })) } else { None };
    items.shrink_to_fit();
    success(&headers, ProjectPage { items, next_cursor })
}
async fn project_read(Extension(user): Extension<User>, Path(_id): Path<Uuid>) -> Result<Json<Envelope<Empty>>, ApiError> { tracing::debug!(actor=%user.subject,"project read requested"); not_implemented().await }

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{header::AUTHORIZATION, Request, StatusCode}};
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    #[test]
    fn project_cursor_round_trips_and_rejects_invalid_data() {
        let cursor = ProjectCursor { created_at: Utc::now(), id: Uuid::new_v4() };
        assert_eq!(decode_cursor(&encode_cursor(&cursor)).unwrap().id, cursor.id);
        assert!(decode_cursor("not-base64!").is_err());
    }

    #[test]
    fn public_status_filters_follow_the_documented_mapping() {
        assert_eq!(public_status_filter(Some("New")), Some(&["seed_live"][..]));
        assert_eq!(public_status_filter(Some("Bonding")), Some(&["bonding_live"][..]));
        assert_eq!(public_status_filter(Some("Graduated")), Some(&["graduated"][..]));
        assert_eq!(public_status_filter(Some("Explore")), None);
        assert_eq!(public_status_filter(Some("Other")), Some(&[][..]));
    }

    #[test]
    fn admin_role_does_not_satisfy_investor_role() {
        let user = User { subject: "admin".into(), roles: vec!["admin".into()] };
        assert!(require_role(&user, "admin").is_ok());
        assert!(require_role(&user, "investor").is_err());
    }

    fn state() -> AppState {
        AppState {
            db: PgPoolOptions::new().connect_lazy("postgres://fractio:fractio@localhost/fractio").unwrap(),
            roles: { let repo = crate::ports::MemoryRoleRepository::default(); repo.set_roles("user-1", vec!["investor".into()]); std::sync::Arc::new(repo) },
            settings: Settings {
                port: 8080, database_url: "postgres://fractio:fractio@localhost/fractio".into(), database_max_connections: 1,
                redis_url: "redis://localhost".into(), solana_rpc_url: "http://localhost".into(), solana_ws_url: "ws://localhost".into(),
                solana_cluster: "localnet".into(), fractio_program_id: crate::config::FRACTIO_PROGRAM_ID.into(),
                jwt_secret: "test-secret-long-enough-for-tests".into(), jwt_algorithm: "HS256".into(), jwt_public_key: String::new(),
                jwt_issuer: String::new(), jwt_audience: String::new(), kyc_provider: "mock".into(), environment: "test".into(), ownership_cap_bps: 500,
                total_supply: 100_000_000, allocation_seed_bps: 1500, allocation_curve_bps: 4500, allocation_capital_bps: 1500,
                allocation_open_market_bps: 2500, seed_vesting_days: 365, milestone_cap_pkr: 50_000_000, pkr_fx_source: "test".into(),
            },
        }
    }
    fn token(roles: &[&str], exp: usize) -> String {
        #[derive(Serialize)] struct TestClaims<'a> { sub: &'a str, exp: usize }
        let _ = roles;
        encode(&Header::new(Algorithm::HS256), &TestClaims { sub: "user-1", exp }, &EncodingKey::from_secret(b"test-secret-long-enough-for-tests")).unwrap()
    }
    async fn request(path: &str, auth: Option<String>) -> axum::response::Response {
        let mut builder = Request::builder().uri(path).method("GET");
        if let Some(token) = auth { builder = builder.header(AUTHORIZATION, format!("Bearer {token}")); }
        router(state()).oneshot(builder.body(Body::empty()).unwrap()).await.unwrap()
    }
    #[tokio::test]
    async fn auth_missing_invalid_expired_and_role_are_checked() {
        assert_eq!(request("/v1/wallet", None).await.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(request("/v1/wallet", Some("not-a-token".into())).await.status(), StatusCode::UNAUTHORIZED);
        let expired = request("/v1/wallet", Some(token(&["investor"], 1))).await;
        let wrong_role = request("/v1/founders/creator-fees", Some(token(&["founder"], usize::MAX))).await;
        let valid = request("/v1/wallet", Some(token(&["investor"], usize::MAX))).await;
        assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(wrong_role.status(), StatusCode::FORBIDDEN);
        assert_eq!(valid.status(), StatusCode::NOT_IMPLEMENTED);
    }
    #[tokio::test]
    async fn public_health_and_request_validation_work() {
        assert_eq!(request("/health/live", None).await.status(), StatusCode::OK);
        let response = router(state()).oneshot(Request::builder().method("POST").uri("/v1/register").header(AUTHORIZATION, format!("Bearer {}", token(&["investor"], usize::MAX))).header("content-type", "application/json").body(Body::from(r#"{"role":"investor","display_name":""}"#)).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
