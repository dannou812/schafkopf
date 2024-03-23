use crate::card::{Card, Color, Symbol};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn generate_full() -> Self {
        Deck::generate(vec![])
    }

    pub fn generate(leave_out: Vec<Symbol>) -> Self {
        let mut result = Deck { cards: Vec::new() };

        // iterate through all the symbols except for the ones contained in leave out
        for symbol in Symbol::each().iter().filter(|s| {
            !leave_out
                .iter()
                .any(|leave_out_symbol| leave_out_symbol == *s)
        }) {
            for color in Color::each() {
                result.cards.push(Card::new(color, *symbol))
            }
        }

        result
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    //todo
    //pub fn get_hand(&self) -> Vec<Card> {}
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Color, Symbol};
    use crate::deck::Deck;

    fn full_deck_hardcoded() -> Vec<Card> {
        vec![
            Card::new(Color::Clamp, Symbol::Seven),
            Card::new(Color::Clamp, Symbol::Eight),
            Card::new(Color::Clamp, Symbol::Nine),
            Card::new(Color::Clamp, Symbol::Ten),
            Card::new(Color::Clamp, Symbol::A),
            Card::new(Color::Clamp, Symbol::U),
            Card::new(Color::Clamp, Symbol::O),
            Card::new(Color::Clamp, Symbol::K),
            Card::new(Color::Leaf, Symbol::Seven),
            Card::new(Color::Leaf, Symbol::Eight),
            Card::new(Color::Leaf, Symbol::Nine),
            Card::new(Color::Leaf, Symbol::Ten),
            Card::new(Color::Leaf, Symbol::A),
            Card::new(Color::Leaf, Symbol::U),
            Card::new(Color::Leaf, Symbol::O),
            Card::new(Color::Leaf, Symbol::K),
            Card::new(Color::Acorn, Symbol::Seven),
            Card::new(Color::Acorn, Symbol::Eight),
            Card::new(Color::Acorn, Symbol::Nine),
            Card::new(Color::Acorn, Symbol::Ten),
            Card::new(Color::Acorn, Symbol::A),
            Card::new(Color::Acorn, Symbol::U),
            Card::new(Color::Acorn, Symbol::O),
            Card::new(Color::Acorn, Symbol::K),
            Card::new(Color::Heart, Symbol::Seven),
            Card::new(Color::Heart, Symbol::Eight),
            Card::new(Color::Heart, Symbol::Nine),
            Card::new(Color::Heart, Symbol::Ten),
            Card::new(Color::Heart, Symbol::A),
            Card::new(Color::Heart, Symbol::U),
            Card::new(Color::Heart, Symbol::O),
            Card::new(Color::Heart, Symbol::K),
        ]
    }

    #[test]
    fn generate_full_deck() {
        let deck = Deck::generate_full();

        for card in full_deck_hardcoded().iter() {
            assert!(deck.cards.contains(card));
        }
    }
}
