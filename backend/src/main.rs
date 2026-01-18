use onitama::onitama::{
    board::Board,
    cards::{CARDS, CardId},
};

fn main() {
    for (card_id, card) in CARDS.iter() {
        println!("cardId:{:?}\n{}", card_id, card)
    }

    println!("{}", CardId::Mantis.get());

    println!("{}", Board::new())
}
