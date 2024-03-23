#[derive(Debug)]
pub struct Card {
    color: Color,
    symbol: Symbol,
}
impl Card {
    pub fn new(color: Color, symbol: Symbol) -> Self {
        Card { color, symbol }
    }

    pub fn get_point_value(&self) -> u32 {
        match self.symbol {
            Symbol::A => 11,
            Symbol::Ten => 10,
            Symbol::K => 4,
            Symbol::O => 3,
            Symbol::U => 2,
            Symbol::Nine => 0,
            Symbol::Eight => 0,
            Symbol::Seven => 0,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card::new(self.color, self.symbol)
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.symbol == other.symbol
    }
}

impl Eq for Card {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Symbol {
    A,
    Ten,
    K,
    O,
    U,
    Nine,
    Eight,
    Seven,
}

impl Symbol {
    pub fn each() -> Vec<Symbol> {
        vec![
            Symbol::A,
            Symbol::Ten,
            Symbol::K,
            Symbol::O,
            Symbol::U,
            Symbol::Nine,
            Symbol::Eight,
            Symbol::Seven,
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    Acorn,
    Leaf,
    Heart,
    Clamp,
}

impl Color {
    pub fn each() -> Vec<Color> {
        vec![Color::Acorn, Color::Leaf, Color::Heart, Color::Clamp]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardType {
    Trump,
    Color,
    RufColor,
}

#[derive(Debug)]
pub struct CardInGame {
    card: Card,
    card_type: CardType,
}

impl PartialEq<Self> for CardInGame {
    fn eq(&self, other: &Self) -> bool {
        self.card == other.card && self.card_type == other.card_type
    }
}

impl CardInGame {

    pub fn new(card: &Card, card_type: CardType) -> CardInGame {

        CardInGame {

            card: card.clone(),
            card_type
        }

    }

    pub fn get_card(&self) -> Card {
        self.card.clone()
    }

    pub fn get_card_type(&self) -> CardType {
        self.card_type
    }
}
