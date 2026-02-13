use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
    serve::Listener,
};
use onitama::{onitama::game::Game, server::state::State as AppState};

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/room/{room_id}/", get(get_game))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_game(State(state): State<Arc<AppState>>, Path(room_id): Path<String>) -> Json<Game> {
    return Json(state.make_or_get(room_id).clone_game());
}
