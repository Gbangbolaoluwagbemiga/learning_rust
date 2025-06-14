use rand::{rng, seq::SliceRandom};

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

    fn shuffler(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_card: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_card)
    }

    // fn add(&mut self) {
    //     let newcard = String::from("Gbenga philip");
    //     self.cards.push(newcard);
    // }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffler();
    print!("Here is my deck {:#?}", deck);

    // print!("Here is my sliced card {:#?}", deck.cards.len());
    let deal = deck.deal(3);

    print!("Here is my sliced card {:#?}", deal);
    // deck.shuffler();
    // deck.add();
    // print!("Here is my deck {:#?}", deck);
    // print!("Here is my deck cards {:?}", deck.cards)
}
