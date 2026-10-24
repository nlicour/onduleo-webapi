mod handlers;
mod models;
mod seed;
mod store;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use handlers::AppState;
use store::ExerciceStore;

#[tokio::main]
async fn main() {
    let store = ExerciceStore::new();
    let count = store.seed(seed::mock_exercices());
    println!("Seeded {} mock exercices", count);

    let state: AppState = Arc::new(store);

    let app = Router::new()
        .route("/", get(|| async { "Onduleo API — see /exercices" }))
        .route("/exercices", get(handlers::list_exercices))
        .route("/exercices", post(handlers::create_exercice))
        .route("/exercices/{code}", get(handlers::get_exercice))
        .route("/exercices/{code}", put(handlers::update_exercice))
        .route("/exercices/{code}", delete(handlers::delete_exercice))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
