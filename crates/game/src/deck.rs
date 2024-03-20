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

        return result;
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
            Card::new(Color::CLAMP, Symbol::SEVEN),
            Card::new(Color::CLAMP, Symbol::EIGHT),
            Card::new(Color::CLAMP, Symbol::NINE),
            Card::new(Color::CLAMP, Symbol::TEN),
            Card::new(Color::CLAMP, Symbol::A),
            Card::new(Color::CLAMP, Symbol::U),
            Card::new(Color::CLAMP, Symbol::O),
            Card::new(Color::CLAMP, Symbol::K),
            Card::new(Color::LEAF, Symbol::SEVEN),
            Card::new(Color::LEAF, Symbol::EIGHT),
            Card::new(Color::LEAF, Symbol::NINE),
            Card::new(Color::LEAF, Symbol::TEN),
            Card::new(Color::LEAF, Symbol::A),
            Card::new(Color::LEAF, Symbol::U),
            Card::new(Color::LEAF, Symbol::O),
            Card::new(Color::LEAF, Symbol::K),
            Card::new(Color::ACORN, Symbol::SEVEN),
            Card::new(Color::ACORN, Symbol::EIGHT),
            Card::new(Color::ACORN, Symbol::NINE),
            Card::new(Color::ACORN, Symbol::TEN),
            Card::new(Color::ACORN, Symbol::A),
            Card::new(Color::ACORN, Symbol::U),
            Card::new(Color::ACORN, Symbol::O),
            Card::new(Color::ACORN, Symbol::K),
            Card::new(Color::HEART, Symbol::SEVEN),
            Card::new(Color::HEART, Symbol::EIGHT),
            Card::new(Color::HEART, Symbol::NINE),
            Card::new(Color::HEART, Symbol::TEN),
            Card::new(Color::HEART, Symbol::A),
            Card::new(Color::HEART, Symbol::U),
            Card::new(Color::HEART, Symbol::O),
            Card::new(Color::HEART, Symbol::K),
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
