pub struct Card {
    color: Color,
    symbol: Symbol,
}

impl Card {
    pub fn new(color: Color, symbol: Symbol) -> Self {
        Card { color, symbol }
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card::new(self.color, self.symbol)
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color && self.symbol == other.symbol;
    }
}

impl Eq for Card {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
    A,
    TEN,
    K,
    O,
    U,
    NINE,
    EIGHT,
    SEVEN,
    NONE
}

impl Symbol {
    pub fn each() -> Vec<Symbol> {
        vec![
            Symbol::A,
            Symbol::TEN,
            Symbol::K,
            Symbol::O,
            Symbol::U,
            Symbol::NINE,
            Symbol::EIGHT,
            Symbol::SEVEN,
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    ACORN,
    LEAF,
    HEART,
    CLAMP,
    NONE
}

impl Color {
    pub fn each() -> Vec<Color> {
        vec![Color::ACORN, Color::LEAF, Color::HEART, Color::CLAMP]
    }
}
