use super::card::{Card, CardNumber, Suit, NUM_CARD_NUMBERS, NUM_SUITS, TWO};
use rand::{thread_rng, Rng};
use std::collections::HashSet;

const NUM_COMMUNITY_CARDS: usize = 5;
const NUM_CARDS_IN_HAND: usize = 2;

pub struct Dealer {
    dealt_cards: HashSet<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Self {
            dealt_cards: HashSet::new(),
        }
    }

    fn generate_card() -> Card {
        let mut rng = thread_rng();
        let card_suit: u32 = rng.gen_range(0..=NUM_SUITS);
        let card_number: u32 = rng.gen_range(TWO..=NUM_CARD_NUMBERS);

        let new_card = Card {
            suit: Suit::match_int_with_suit(card_suit),
            number: CardNumber::match_int_with_card_num(card_number),
        };
        return new_card;
    }

    fn generate_new_card(&mut self) -> Card {
        let mut new_card = Dealer::generate_card();
        while self.dealt_cards.contains(&new_card) {
            new_card = Dealer::generate_card();
        }
        self.dealt_cards.insert(new_card.clone());
        return new_card;
    }

    pub fn deal(&mut self, num_players: u32) -> (Vec<Card>, Vec<Vec<Card>>) {
        let mut community_cards: Vec<Card> = vec![];
        let mut player_cards: Vec<Vec<Card>> = vec![];

        for _ in 0..NUM_COMMUNITY_CARDS {
            let new_card_ref = Dealer::generate_new_card(self);
            community_cards.push(new_card_ref);
        }
        for _ in 0..num_players {
            let mut two_card_hand: Vec<Card> = vec![];
            for _ in 0..NUM_CARDS_IN_HAND {
                let new_card_ref = Dealer::generate_new_card(self);
                two_card_hand.push(new_card_ref);
            }
            two_card_hand.sort_by_key(|card| std::cmp::Reverse(card.number.get_value()));
            player_cards.push(two_card_hand);
        }

        return (community_cards, player_cards);
    }
}
