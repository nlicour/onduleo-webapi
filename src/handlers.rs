use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::models::{Exercice, ExerciceCreate, ExerciceUpdate};
use crate::store::ExerciceStore;

pub type AppState = Arc<ExerciceStore>;

/// GET /exercices — list all exercices
pub async fn list_exercices(State(state): State<AppState>) -> Json<Vec<Exercice>> {
    Json(state.list())
}

/// GET /exercices/:code — get a single exercice by code
pub async fn get_exercice(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Exercice>, StatusCode> {
    state.get(&code).map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// POST /exercices — create a new exercice
pub async fn create_exercice(
    State(state): State<AppState>,
    Json(payload): Json<ExerciceCreate>,
) -> Result<(StatusCode, Json<Exercice>), (StatusCode, Json<serde_json::Value>)> {
    match state.create(payload) {
        Ok(exercice) => Ok((StatusCode::CREATED, Json(exercice))),
        Err(msg) => Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": msg })),
        )),
    }
}

/// PUT /exercices/:code — update an existing exercice (partial update)
pub async fn update_exercice(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Json(payload): Json<ExerciceUpdate>,
) -> Result<Json<Exercice>, StatusCode> {
    state
        .update(&code, payload)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /exercices/:code — delete an exercice by code
pub async fn delete_exercice(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> StatusCode {
    if state.delete(&code).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
