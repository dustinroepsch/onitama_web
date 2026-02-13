use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use onitama::{
    onitama::cards::CardId,
    onitama::game::{ActError, Action, Game},
    server::state::State as AppState,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    println!("Example Action:");
    println!(
        "{}",
        serde_json::to_string_pretty(&Action {
            from: (1u8, 2u8).try_into().unwrap(),
            to: (3u8, 4u8).try_into().unwrap(),
            card: CardId::Dragon
        })
        .unwrap()
    );

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/room/{room_id}", get(get_game))
        .route("/act/{room_id}", post(post_action))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_game(State(state): State<Arc<AppState>>, Path(room_id): Path<String>) -> Json<Game> {
    Json(state.make_or_get(room_id).clone_game())
}

async fn post_action(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
    Json(action): Json<Action>,
) -> Json<Result<(), ActError>> {
    Json(state.make_or_get(room_id).act(&action))
}
