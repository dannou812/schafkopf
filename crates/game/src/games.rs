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
            return compare_cards_no_trumps(card1, card2);
        }

        return Ordering::Less;
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
    }

    compare_cards_no_trumps(card1, card2)
}

fn compare_cards_no_trumps(card1: &Card, card2: &Card) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }

    let ordering = compare_colors(card1, card2);

    if ordering == Ordering::Equal {
        ordering
    } else {
        compare_symbols(card1, card2)
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
    // analyzes a given deck and returns what part what card will take in a given game
    fn get_cards_in_game_descending(&self, deck: &Deck) -> Vec<CardInGame> {
        let mut cards_descending: Vec<Card> = deck.get_cards();

        let card_type: CardType;

        match self {
            SubGame::Rufspiel(color) => {

                cards_descending.sort_by(compare_cards_regular_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_symbol() == Symbol::U || card.get_color() == Color::Heart {
                        card_type = CardType::Trump;
                    }else if card.get_color() == *color{
                        card_type = CardType::RufColor;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
            SubGame::Wenz => {

                cards_descending.sort_by(compare_cards_geier);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::U {
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
            SubGame::Geier => {

                cards_descending.sort_by(compare_cards_geier);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O {
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
            SubGame::Farbwenz(color) => {

                cards_descending.sort_by(compare_cards_wenz);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::U || card.get_color() == *color {
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
            SubGame::Farbgeier(color) => {

                cards_descending.sort_by(compare_cards_geier);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_color() == *color {
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
            SubGame::Solo(color) => {

                cards_descending.sort_by(compare_cards_regular_trumps);
                cards_descending.iter().map(|card| -> CardInGame {

                    if card.get_symbol() == Symbol::O || card.get_symbol() == Symbol::U || card.get_color() == *color {
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

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
                        card_type = CardType::Trump;
                    }else{
                        card_type = CardType::Color;
                    }

                    CardInGame::new(card, card_type)

                }).collect()

            }
        }
    }
}
