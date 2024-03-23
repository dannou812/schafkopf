use crate::card::Card;
use crate::trick::Trick;

pub trait Player {
    fn get_name() -> str;
    fn request_move(current_trick: &Trick, legal_moves: &[Card]) -> Card;
    fn advance_round(finished_trick: &Trick);
    fn deal_hand(hand: &[Card]);
    fn get_hand() -> Vec<Card>;
}
