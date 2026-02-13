use onitama::onitama::game::Game;
use serde_json;

fn main() {
    let game = Game::new();

    println!("{}", &game);
    println!("{}", serde_json::to_string_pretty(&game).unwrap())
}
