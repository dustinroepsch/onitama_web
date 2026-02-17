use std::sync::LazyLock;

use salvo::{oapi::extract::PathParam, prelude::*};

use onitama::{
    onitama::{card::Card, cards::CARDS, game::Game},
    server::state::State as AppState,
};

static STATE: LazyLock<AppState> = LazyLock::new(AppState::new);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;

    let router = Router::new()
        .push(Router::with_path("cards").get(get_cards))
        .push(Router::with_path("room/{room_id}").get(get_game));

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn get_game(room_id: PathParam<String>) -> Json<Game> {
    Json(STATE.make_or_get(room_id.into_inner()).clone_game())
}

// #[handler]
// async fn post_action(
//     Path(room_id): Path<String>,
//     Json(action): Json<Action>,
// ) -> Json<Result<(), ActError>> {
//     Json(state.make_or_get(room_id).act(&action))
// }

// async fn get_game_display(
//     State(state): State<Arc<AppState>>,
//     Path(room_id): Path<String>,
// ) -> String {
//     format!("{}", state.make_or_get(room_id))
// }

#[handler]
async fn get_cards() -> Json<Vec<Card>> {
    Json(CARDS.values().cloned().collect())
}
