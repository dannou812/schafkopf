//this file will contain the rules for all the different games that are playable

use crate::card::{Card, CardInGame, CardType, Color, Symbol};
use crate::deck::Deck;
use std::cmp::Ordering;

pub enum SubGame {
    Rufspiel(Color),
    Wenz,
    Geier,
    Farbwenz(Color),
    Farbgeier(Color),
    Solo(Color),
    Bettel,
    Ramsch,
}

fn compare_cards_regular_trumps(card1: &Card, card2: &Card) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }

    if card1.get_symbol() == Symbol::O || card1.get_symbol() == Symbol::U {
        if card2.get_symbol() == Symbol::O || card2.get_symbol() == Symbol::U {
            let ordering = compare_symbols(card1, card2);
            return if ordering == Ordering::Equal {
                compare_colors(card1, card2)
            } else {
                ordering
            }
        }

        return Ordering::Less;
    }else if card2.get_symbol() == Symbol::O || card2.get_symbol() == Symbol::U {
        return Ordering::Greater;
    }

    compare_cards_no_trumps(card1, card2)
}

fn compare_cards_wenz(card1: &Card, card2: &Card) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }

    if card1.get_symbol() == Symbol::U {
        if card2.get_symbol() == Symbol::U {
            return compare_colors(card1, card2);
        }

        return Ordering::Less;
    }else if card2.get_symbol() == Symbol::U{
        return Ordering::Greater;
    }

    compare_cards_no_trumps(card1, card2)
}

fn compare_cards_geier(card1: &Card, card2: &Card) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }

    if card1.get_symbol() == Symbol::O {
        if card2.get_symbol() == Symbol::O {
            return compare_colors(card1, card2);
        }

        return Ordering::Less;
    }else if card2.get_symbol() == Symbol::O {
        return Ordering::Greater;
    }

    compare_cards_no_trumps(card1, card2)
}

fn compare_cards_no_trumps(card1: &Card, card2: &Card) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }

    let ordering = compare_colors(card1, card2);

    if ordering == Ordering::Equal {
        compare_symbols(card1, card2)
    } else {
        ordering
    }
}

fn compare_colors(card1: &Card, card2: &Card) -> Ordering {
    if card1.get_color() == card2.get_color() {
        return Ordering::Equal;
    }

    if card1 == card2 {
        return Ordering::Equal;
    }

    match card1.get_color() {
        Color::Acorn => Ordering::Less,
        Color::Leaf => match card2.get_color() {
            Color::Acorn => Ordering::Greater,
            _ => Ordering::Less,
        },
        Color::Heart => match card2.get_color() {
            Color::Clamp => Ordering::Less,
            _ => Ordering::Greater,
        },
        Color::Clamp => Ordering::Greater,
    }
}

fn compare_symbols(card1: &Card, card2: &Card) -> Ordering {
    if card1.get_symbol() == card2.get_symbol() {
        return Ordering::Equal;
    }

    if card1 == card2 {
        return Ordering::Equal;
    }

    match card1.get_symbol() {
        Symbol::A => Ordering::Less,
        Symbol::Ten => match card2.get_symbol() {
            Symbol::A => Ordering::Greater,
            _ => Ordering::Less,
        },
        Symbol::K => match card2.get_symbol() {
            Symbol::A => Ordering::Greater,
            Symbol::Ten => Ordering::Greater,
            _ => Ordering::Less,
        },
        Symbol::O => match card2.get_symbol() {
            Symbol::A => Ordering::Greater,
            Symbol::Ten => Ordering::Greater,
            Symbol::K => Ordering::Greater,
            _ => Ordering::Less,
        },
        Symbol::U => match card2.get_symbol() {
            Symbol::Nine => Ordering::Less,
            Symbol::Eight => Ordering::Less,
            Symbol::Seven => Ordering::Less,
            _ => Ordering::Greater,
        },
        Symbol::Nine => match card2.get_symbol() {
            Symbol::Eight => Ordering::Less,
            Symbol::Seven => Ordering::Less,
            _ => Ordering::Greater,
        },
        Symbol::Eight => match card2.get_symbol() {
            Symbol::Seven => Ordering::Less,
            _ => Ordering::Greater,
        },
        Symbol::Seven => Ordering::Greater,
    }
}

impl SubGame {

    fn get_cards_in_game_descending_from_deck(&self, deck: &Deck) -> Vec<CardInGame>{
        self.get_cards_in_game_descending(&*deck.get_cards())
    }

    // analyzes a given deck and returns what part what card will take in a given game
    fn get_cards_in_game_descending(&self, cards: &[Card]) -> Vec<CardInGame> {
        let mut cards_descending: Vec<Card> = Vec::from(cards);

        let mut result: Vec<CardInGame> = match self {
            SubGame::Rufspiel(color) => {

                cards_descending.sort_by(compare_cards_regular_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_symbol() == Symbol::U || card.get_color() == Color::Heart {
                        CardInGame::new(card, CardType::Trump)
                    }else if card.get_color() == *color{
                        CardInGame::new(card, CardType::RufColor)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }



                }).collect()

            }
            SubGame::Wenz => {

                cards_descending.sort_by(compare_cards_wenz);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::U {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
            SubGame::Geier => {

                cards_descending.sort_by(compare_cards_geier);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
            SubGame::Farbwenz(color) => {

                cards_descending.sort_by(compare_cards_wenz);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::U || card.get_color() == *color {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
            SubGame::Farbgeier(color) => {

                cards_descending.sort_by(compare_cards_geier);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_color() == *color {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
            SubGame::Solo(color) => {

                cards_descending.sort_by(compare_cards_regular_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_symbol() == Symbol::U || card.get_color() == *color {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
            SubGame::Bettel => {

                cards_descending.sort_by(compare_cards_no_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    CardInGame::new(card, CardType::Color)

                }).collect()

            }
            SubGame::Ramsch => {

                cards_descending.sort_by(compare_cards_regular_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_symbol() == Symbol::U || card.get_color() == Color::Heart {
                        CardInGame::new(card, CardType::Trump)
                    }else{
                        CardInGame::new(card, CardType::Color)
                    }

                }).collect()

            }
        };

        result.sort_by(|card, other| -> Ordering {
            return if card.get_card_type() == other.get_card_type() {
                Ordering::Equal
            } else if card.get_card_type() == CardType::Trump {
                Ordering::Less
            } else if other.get_card_type() == CardType::Trump {
                Ordering::Greater
            } else if card.get_card_type() == CardType::RufColor {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, CardInGame, CardType, Color, Symbol};
    use crate::games::SubGame;

    fn get_unsorted_cards() -> Vec<Card> {


        vec![
            Card::new(Color::Heart, Symbol::O),
            Card::new(Color::Leaf, Symbol::Nine),
            Card::new(Color::Acorn, Symbol::U),
            Card::new(Color::Leaf, Symbol::A),
            Card::new(Color::Heart, Symbol::Nine),
            Card::new(Color::Acorn, Symbol::Eight),
            Card::new(Color::Acorn, Symbol::O),
            Card::new(Color::Clamp, Symbol::K),
            Card::new(Color::Acorn, Symbol::Nine),
            Card::new(Color::Heart, Symbol::Eight),
            Card::new(Color::Clamp, Symbol::U),
            Card::new(Color::Acorn, Symbol::Seven),
            Card::new(Color::Leaf, Symbol::Ten),
            Card::new(Color::Heart, Symbol::K),
            Card::new(Color::Clamp, Symbol::A),
            Card::new(Color::Heart, Symbol::U),
            Card::new(Color::Acorn, Symbol::A),
            Card::new(Color::Clamp, Symbol::Seven),
            Card::new(Color::Heart, Symbol::Seven),
            Card::new(Color::Leaf, Symbol::Seven),
            Card::new(Color::Heart, Symbol::Ten),
            Card::new(Color::Acorn, Symbol::K),
            Card::new(Color::Acorn, Symbol::Ten),
            Card::new(Color::Leaf, Symbol::K),
            Card::new(Color::Leaf, Symbol::U),
            Card::new(Color::Leaf, Symbol::O),
            Card::new(Color::Clamp, Symbol::Nine),
            Card::new(Color::Heart, Symbol::A),
            Card::new(Color::Clamp, Symbol::Eight),
            Card::new(Color::Clamp, Symbol::Ten),
            Card::new(Color::Leaf, Symbol::Eight),
            Card::new(Color::Clamp, Symbol::O),

        ]
    }

    #[test]
    fn bettel_sorted_correctly() {

        let sub_game = SubGame::Bettel;
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Color));
        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Color));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Color));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Color));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Color));
        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Color));
        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Color));

        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));


    }

    #[test]
    fn ramsch_sorted_correctly() {

        let sub_game = SubGame::Ramsch;
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Trump));
        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Trump));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Trump));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Trump));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Trump));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Trump));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Trump));
        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Trump));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Trump));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Trump));

        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn rufspiel_sorted_correctly() {

        let sub_game = SubGame::Rufspiel(Color::Clamp);
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Trump));
        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Trump));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Trump));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Trump));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Trump));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Trump));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Trump));
        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Trump));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Trump));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Trump));

        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::RufColor));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::RufColor));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::RufColor));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::RufColor));
        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::RufColor));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::RufColor));

        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn wenz_sorted_correctly() {

        let sub_game = SubGame::Wenz;
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Trump));

        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Color));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Color));
        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Color));

        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn geier_sorted_correctly() {

        let sub_game = SubGame::Geier;
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Trump));

        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Color));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Color));
        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Color));

        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn farbwenz_sorted_correctly() {

        let sub_game = SubGame::Farbwenz(Color::Leaf);
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Trump));

        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Trump));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Trump));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Trump));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Trump));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Trump));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Trump));

        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Color));
        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Color));

        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn farbgeier_sorted_correctly() {

        let sub_game = SubGame::Farbgeier(Color::Heart);
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Trump));

        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Trump));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Trump));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Trump));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Trump));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Trump));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Trump));

        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Color));
        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Color));

    }

    #[test]
    fn solo_sorted_correctly() {

        let sub_game = SubGame::Solo(Color::Clamp);
        let result = sub_game.get_cards_in_game_descending(&*get_unsorted_cards());

        assert_eq!(result[0], CardInGame::new(&Card::new(Color::Acorn, Symbol::O), CardType::Trump));
        assert_eq!(result[1], CardInGame::new(&Card::new(Color::Leaf, Symbol::O), CardType::Trump));
        assert_eq!(result[2], CardInGame::new(&Card::new(Color::Heart, Symbol::O), CardType::Trump));
        assert_eq!(result[3], CardInGame::new(&Card::new(Color::Clamp, Symbol::O), CardType::Trump));
        assert_eq!(result[4], CardInGame::new(&Card::new(Color::Acorn, Symbol::U), CardType::Trump));
        assert_eq!(result[5], CardInGame::new(&Card::new(Color::Leaf, Symbol::U), CardType::Trump));
        assert_eq!(result[6], CardInGame::new(&Card::new(Color::Heart, Symbol::U), CardType::Trump));
        assert_eq!(result[7], CardInGame::new(&Card::new(Color::Clamp, Symbol::U), CardType::Trump));
        assert_eq!(result[8], CardInGame::new(&Card::new(Color::Clamp, Symbol::A), CardType::Trump));
        assert_eq!(result[9], CardInGame::new(&Card::new(Color::Clamp, Symbol::Ten), CardType::Trump));
        assert_eq!(result[10], CardInGame::new(&Card::new(Color::Clamp, Symbol::K), CardType::Trump));
        assert_eq!(result[11], CardInGame::new(&Card::new(Color::Clamp, Symbol::Nine), CardType::Trump));
        assert_eq!(result[12], CardInGame::new(&Card::new(Color::Clamp, Symbol::Eight), CardType::Trump));
        assert_eq!(result[13], CardInGame::new(&Card::new(Color::Clamp, Symbol::Seven), CardType::Trump));

        assert_eq!(result[14], CardInGame::new(&Card::new(Color::Acorn, Symbol::A), CardType::Color));
        assert_eq!(result[15], CardInGame::new(&Card::new(Color::Acorn, Symbol::Ten), CardType::Color));
        assert_eq!(result[16], CardInGame::new(&Card::new(Color::Acorn, Symbol::K), CardType::Color));
        assert_eq!(result[17], CardInGame::new(&Card::new(Color::Acorn, Symbol::Nine), CardType::Color));
        assert_eq!(result[18], CardInGame::new(&Card::new(Color::Acorn, Symbol::Eight), CardType::Color));
        assert_eq!(result[19], CardInGame::new(&Card::new(Color::Acorn, Symbol::Seven), CardType::Color));

        assert_eq!(result[20], CardInGame::new(&Card::new(Color::Leaf, Symbol::A), CardType::Color));
        assert_eq!(result[21], CardInGame::new(&Card::new(Color::Leaf, Symbol::Ten), CardType::Color));
        assert_eq!(result[22], CardInGame::new(&Card::new(Color::Leaf, Symbol::K), CardType::Color));
        assert_eq!(result[23], CardInGame::new(&Card::new(Color::Leaf, Symbol::Nine), CardType::Color));
        assert_eq!(result[24], CardInGame::new(&Card::new(Color::Leaf, Symbol::Eight), CardType::Color));
        assert_eq!(result[25], CardInGame::new(&Card::new(Color::Leaf, Symbol::Seven), CardType::Color));

        assert_eq!(result[26], CardInGame::new(&Card::new(Color::Heart, Symbol::A), CardType::Color));
        assert_eq!(result[27], CardInGame::new(&Card::new(Color::Heart, Symbol::Ten), CardType::Color));
        assert_eq!(result[28], CardInGame::new(&Card::new(Color::Heart, Symbol::K), CardType::Color));
        assert_eq!(result[29], CardInGame::new(&Card::new(Color::Heart, Symbol::Nine), CardType::Color));
        assert_eq!(result[30], CardInGame::new(&Card::new(Color::Heart, Symbol::Eight), CardType::Color));
        assert_eq!(result[31], CardInGame::new(&Card::new(Color::Heart, Symbol::Seven), CardType::Color));

    }

}
