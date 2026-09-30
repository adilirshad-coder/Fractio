use axum::{extract::{Extension, Path, Request, State}, http::{header::AUTHORIZATION, StatusCode}, middleware::{self, Next}, response::Response, routing::{get,post}, Json, Router};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize,Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use crate::{config::Settings,errors::ApiError};
#[derive(Clone)] pub struct AppState { pub db: PgPool, pub settings: Settings }
#[derive(Clone,Debug,Deserialize)] struct Claims { sub:String, exp:usize, #[serde(default)] roles:Vec<String> }
#[derive(Clone)] struct User { subject:String, roles:Vec<String> }
#[derive(Deserialize,Validate)] pub struct CreateProjectRequest { #[validate(length(min=1,max=160))] name:String, #[validate(length(max=4000))] description:Option<String> }
#[derive(Serialize)] struct Envelope<T> { data:T, meta:serde_json::Value, request_id:String }
#[derive(Serialize)] struct ProjectResponse { id:Uuid, status:&'static str }
pub fn router(state:AppState)->Router { Router::new().route("/health/live",get(live)).route("/health/ready",get(ready)).route("/v1/projects",post(create_project)).route("/v1/projects/{id}",get(get_project)).route("/v1/portfolio",get(portfolio)).route("/v1/projects/{id}/invest",post(invest)).route_layer(middleware::from_fn_with_state(state.clone(),auth)).with_state(state) }
async fn auth(State(s):State<AppState>,mut req:Request,next:Next)->Result<Response,ApiError>{ if req.uri().path().starts_with("/health") { return Ok(next.run(req).await); } let h=req.headers().get(AUTHORIZATION).and_then(|v|v.to_str().ok()).ok_or(ApiError::Unauthorized)?; let t=h.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?; let c=decode::<Claims>(t,&DecodingKey::from_secret(s.settings.jwt_secret.as_bytes()),&Validation::new(Algorithm::HS256)).map_err(|_|ApiError::Unauthorized)?.claims; if c.sub.is_empty(){return Err(ApiError::Unauthorized)} req.extensions_mut().insert(User{subject:c.sub,roles:c.roles}); Ok(next.run(req).await) }
async fn live()->StatusCode{StatusCode::NO_CONTENT}
async fn ready(State(s):State<AppState>)->Result<StatusCode,ApiError>{sqlx::query("SELECT 1").execute(&s.db).await.map_err(|e|ApiError::Internal(e.into()))?;Ok(StatusCode::NO_CONTENT)}
async fn create_project(Extension(u):Extension<User>,Json(r):Json<CreateProjectRequest>)->Result<Json<Envelope<ProjectResponse>>,ApiError>{r.validate().map_err(|e|ApiError::Validation(e.to_string()))?;tracing::info!(actor=%u.subject,roles=?u.roles,name=%r.name,"project creation requested"); // TODO: Implement business logic
 Err(ApiError::NotImplemented)}
async fn get_project(Extension(u):Extension<User>,Path(_id):Path<Uuid>)->Result<Json<Envelope<ProjectResponse>>,ApiError>{tracing::info!(actor=%u.subject,"project read requested"); // TODO: Implement business logic
 Err(ApiError::NotImplemented)}
#[derive(Deserialize,Validate)] struct InvestRequest { amount_minor:i64, #[validate(length(min=1,max=128))] idempotency_key:String }
async fn invest(Extension(u):Extension<User>,Path(_id):Path<Uuid>,Json(r):Json<InvestRequest>)->Result<Json<Envelope<ProjectResponse>>,ApiError>{r.validate().map_err(|e|ApiError::Validation(e.to_string()))?;if r.amount_minor<=0{return Err(ApiError::Validation("amount must be positive".into()))}tracing::info!(actor=%u.subject,"investment intent requested"); // TODO: Implement business logic
 Err(ApiError::NotImplemented)}
async fn portfolio(Extension(u):Extension<User>)->Result<Json<Envelope<serde_json::Value>>,ApiError>{tracing::info!(actor=%u.subject,"portfolio requested"); // TODO: Implement business logic
 Err(ApiError::NotImplemented)}
