#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
            // println!("my cards are :{:?}", cards);
        }
    }

    let deck = Deck { cards: cards };
    print!("Here is my deck {:#?}", deck);
    // print!("Here is my deck cards {:?}", deck.cards)
}
