use onitama::onitama::game::Game;

fn main() {
    let game = Game::new();

    println!("{}", &game);
    println!("{}", serde_json::to_string_pretty(&game).unwrap())
}
