use crate::card::Card;

// struct for one trick (0 to 4 cards that are played)
pub struct Trick {
    cards: Vec<Card>,
}

impl Trick {
    fn new() -> Self {
        Trick { cards: Vec::new() }
    }

    fn add_card(&mut self, mut card: &Card) {
        self.cards.push(card.clone())
    }

    pub fn see_cards(self) -> Vec<Card>{
        Vec::from(self.cards)
    }
}
