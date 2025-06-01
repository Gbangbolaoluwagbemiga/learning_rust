#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
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

        Deck { cards: cards }
    }
}

fn main() {
    let deck = Deck::new();
    print!("Here is my deck {:#?}", deck);
    // print!("Here is my deck cards {:?}", deck.cards)
}
