//! API module for REST and GraphQL endpoints

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// API state
#[derive(Default)]
pub struct ApiState {
    pub connected: bool,
    pub galaxy: Option<String>,
}

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Create the API router
pub fn create_router() -> Router {
    let state = Arc::new(RwLock::new(ApiState::default()));
    
    Router::new()
        .route("/health", get(health_handler))
        .route("/v1/repos", get(list_repos_handler))
        .route("/v1/repos", post(create_repo_handler))
        .route("/v1/agents", get(list_agents_handler))
        .route("/v1/agents", post(spawn_agent_handler))
        .with_state(state)
}

/// Health check handler
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.2.0".to_string(),
    })
}

/// List repositories handler
async fn list_repos_handler() -> Result<Json<Vec<RepoResponse>>, StatusCode> {
    let repos = crate::storage::list_repos(None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<RepoResponse> = repos
        .into_iter()
        .map(|r| RepoResponse {
            did: r.did,
            name: r.name,
            cid: r.cid,
        })
        .collect();
    
    Ok(Json(response))
}

#[derive(Serialize)]
struct RepoResponse {
    did: String,
    name: String,
    cid: String,
}

/// Create repository request
#[derive(Deserialize)]
struct CreateRepoRequest {
    name: String,
}

/// Create repository handler
async fn create_repo_handler(
    Json(payload): Json<CreateRepoRequest>,
) -> Result<Json<RepoResponse>, StatusCode> {
    let repo = crate::storage::create_repo(&payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(RepoResponse {
        did: repo.did,
        name: repo.name,
        cid: repo.cid,
    }))
}

/// List agents handler
async fn list_agents_handler() -> Result<Json<Vec<AgentResponse>>, StatusCode> {
    let agents = crate::agent::list_agents()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<AgentResponse> = agents
        .into_iter()
        .map(|a| AgentResponse {
            did: a.did,
            name: a.name,
            trust_score: a.trust_score,
            status: a.status,
        })
        .collect();
    
    Ok(Json(response))
}

#[derive(Serialize)]
struct AgentResponse {
    did: String,
    name: String,
    trust_score: f64,
    status: String,
}

/// Spawn agent request
#[derive(Deserialize)]
struct SpawnAgentRequest {
    name: String,
}

/// Spawn agent handler
async fn spawn_agent_handler(
    Json(payload): Json<SpawnAgentRequest>,
) -> Result<Json<AgentResponse>, StatusCode> {
    let agent = crate::agent::spawn(&payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(AgentResponse {
        did: agent.did,
        name: agent.name,
        trust_score: agent.trust_score,
        status: agent.status,
    }))
}

/// Start the API server
pub async fn start_server(addr: &str) -> anyhow::Result<()> {
    let app = create_router();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    tracing::info!("API server listening on {}", addr);
    axum::serve(listener, app).await?;
    
    Ok(())
}
