use super::card::{Card, CardNumber, Suit};
use ::std::collections::{HashMap, HashSet};

const NUM_CARDS_IN_HAND: usize = 5;
const NUM_CARDS_IN_TRIPLET: usize = 3;
const NUM_CARDS_IN_QUAD: usize = 4;
const NUM_CARDS_IN_PAIR: usize = 2;

const STRAIGHT_INDICIES: [usize; 1] = [0]; //Royal flush, straight flush, straight
const QUAD_INDICIES: [usize; 2] = [0, 4]; // quad
const FULL_HOUSE_INDICIES: [usize; 2] = [0, 3]; //full house
const HIGH_CARD_INDICIES: [usize; 5] = [0, 1, 2, 3, 4]; //flush, high card
const TRIPS_INDICIES: [usize; 3] = [0, 3, 4]; // trips
const TWO_PAIR_INDICIES: [usize; 3] = [0, 2, 4]; //two pair
const PAIR_INDICIES: [usize; 4] = [0, 2, 3, 4]; //pair

#[derive(Clone)]
pub enum Hand {
    RoyalFlush(Vec<Card>),
    StraightFlush(Vec<Card>),
    Quads(Vec<Card>),
    FullHouse(Vec<Card>),
    Flush(Vec<Card>),
    Straight(Vec<Card>),
    Trips(Vec<Card>),
    TwoPair(Vec<Card>),
    Pair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl Hand {
    pub fn _to_string(&self) -> String {
        match self {
            Self::RoyalFlush(cards) => format!("Royal Flush: {:?}", Card::cards_to_string(cards)),
            Self::StraightFlush(cards) => {
                format!("Straight Flush: {:?}", Card::cards_to_string(cards))
            }
            Self::Quads(cards) => format!("Quads: {:?}", Card::cards_to_string(cards)),
            Self::FullHouse(cards) => format!("Full House: {:?}", Card::cards_to_string(cards)),
            Self::Flush(cards) => format!("Flush: {:?}", Card::cards_to_string(cards)),
            Self::Straight(cards) => format!("Straight: {:?}", Card::cards_to_string(cards)),
            Self::Trips(cards) => format!("Trips: {:?}", Card::cards_to_string(cards)),
            Self::TwoPair(cards) => format!("Two Pair: {:?}", Card::cards_to_string(cards)),
            Self::Pair(cards) => format!("Pair: {:?}", Card::cards_to_string(cards)),
            Self::HighCard(cards) => format!("High Card: {:?}", Card::cards_to_string(cards)),
        }
    }
    pub fn get_value(&self) -> u32 {
        match self {
            Self::RoyalFlush(_) => 10,
            Self::StraightFlush(_) => 9,
            Self::Quads(_) => 8,
            Self::FullHouse(_) => 7,
            Self::Flush(_) => 6,
            Self::Straight(_) => 5,
            Self::Trips(_) => 4,
            Self::TwoPair(_) => 3,
            Self::Pair(_) => 2,
            Self::HighCard(_) => 1,
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match self {
            Self::RoyalFlush(cards) => cards.clone(),
            Self::StraightFlush(cards) => cards.clone(),
            Self::Quads(cards) => cards.clone(),
            Self::FullHouse(cards) => cards.clone(),
            Self::Flush(cards) => cards.clone(),
            Self::Straight(cards) => cards.clone(),
            Self::Trips(cards) => cards.clone(),
            Self::TwoPair(cards) => cards.clone(),
            Self::Pair(cards) => cards.clone(),
            Self::HighCard(cards) => cards.clone(),
        }
    }
}

pub struct HandAnalyser {
    card_combinations: Vec<Vec<Card>>,
    player_cards: Vec<Vec<Card>>,
}

impl HandAnalyser {
    pub fn new(community_cards: Vec<Card>, player_cards: Vec<Vec<Card>>) -> Self {
        let mut combined_hands =
            HandAnalyser::get_card_combinations(player_cards.clone(), community_cards);

        for card_set in combined_hands.iter_mut() {
            card_set.sort_by_key(|card| std::cmp::Reverse(card.number.get_value()));
        }

        Self {
            card_combinations: combined_hands,
            player_cards,
        }
    }

    fn contains_straight_flush(grouping_by_suit: &HashMap<Suit, Vec<Card>>) -> Option<Vec<Card>> {
        for (_, cards) in grouping_by_suit.into_iter() {
            let straight_flush = HandAnalyser::contains_straight(cards);
            if straight_flush.is_some() {
                return straight_flush;
            }
        }
        return None;
    }

    fn contains_quads(
        grouping_by_card_number: &HashMap<CardNumber, Vec<Card>>,
        cards: &Vec<Card>,
    ) -> Option<Vec<Card>> {
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if cards_in_card_number.len() == NUM_CARDS_IN_QUAD {
                let mut quad_hand_cards = cards_in_card_number.clone();
                for card in cards {
                    if card.number != *card_number {
                        quad_hand_cards.push(card.clone());
                        break;
                    }
                }
                if quad_hand_cards.len() == NUM_CARDS_IN_HAND {
                    return Some(quad_hand_cards);
                }
            }
        }
        return None;
    }

    fn contains_full_house(
        grouping_by_card_number: &HashMap<CardNumber, Vec<Card>>,
    ) -> Option<Vec<Card>> {
        let mut threes_card_number = &CardNumber::Two;
        let mut full_house_cards: Vec<Card> = vec![];
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > threes_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_TRIPLET
            {
                threes_card_number = card_number;
            }
        }
        if let Some(threes) = grouping_by_card_number.get(threes_card_number) {
            full_house_cards.extend(threes.clone());
        }
        let mut twos_card_number = &CardNumber::Two;
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > twos_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_PAIR
                && card_number != threes_card_number
            {
                twos_card_number = card_number;
            }
        }
        if let Some(twos) = grouping_by_card_number.get(twos_card_number) {
            full_house_cards.extend(twos.clone());
        }
        if full_house_cards.len() == NUM_CARDS_IN_HAND {
            return Some(full_house_cards);
        }

        return None;
    }

    fn contains_flush(grouping_by_suit: &HashMap<Suit, Vec<Card>>) -> Option<Vec<Card>> {
        for (_, cards) in grouping_by_suit.into_iter() {
            if cards.len() == NUM_CARDS_IN_HAND {
                return Some(cards.clone());
            }
        }
        return None;
    }

    fn contains_straight(cards: &Vec<Card>) -> Option<Vec<Card>> {
        let mut found_card_nums = HashSet::new();
        let mut filtered_sorted_cards = vec![];

        for card in cards {
            if !found_card_nums.contains(&card.number) {
                filtered_sorted_cards.push(card.clone());
                found_card_nums.insert(&card.number);
            }
        }

        if filtered_sorted_cards[0].number == CardNumber::Ace {
            filtered_sorted_cards.push(Card {
                number: CardNumber::BottomAce,
                suit: filtered_sorted_cards[0].suit.clone(),
            });
        }

        if filtered_sorted_cards.len() < NUM_CARDS_IN_HAND {
            return None;
        }

        let mut straight_start_index: usize = 0;
        let mut len_continuous_cards: usize = 1;
        let mut res: Option<Vec<Card>> = None;

        let mut index = 1;
        while index < filtered_sorted_cards.len() {
            let cur_card = &filtered_sorted_cards[index];
            let prev_card = &filtered_sorted_cards[index - 1];

            let cur_card_value = cur_card.number.get_value();
            let prev_card_value = prev_card.number.get_value();

            if cur_card_value == prev_card_value - 1 {
                len_continuous_cards += 1;
            } else {
                len_continuous_cards = 1;
                straight_start_index = index;
            }

            if len_continuous_cards == NUM_CARDS_IN_HAND {
                let mut straight = vec![];
                for offset in 0..NUM_CARDS_IN_HAND {
                    let card = &filtered_sorted_cards[straight_start_index + offset];
                    if card.number == CardNumber::BottomAce {
                        straight.push(Card {
                            number: CardNumber::Ace,
                            suit: card.suit.clone(),
                        })
                    } else {
                        straight.push(card.clone());
                    }
                }
                res = Some(straight);
                break;
            }

            index += 1;
        }

        return res;
    }

    fn contains_trips(
        grouping_by_card_number: &HashMap<CardNumber, Vec<Card>>,
        cards: &Vec<Card>,
    ) -> Option<Vec<Card>> {
        let mut trips_card_number = &CardNumber::Two;
        let mut trips_cards: Vec<Card> = vec![];
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > trips_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_TRIPLET
            {
                trips_card_number = card_number;
            }
        }
        if let Some(trips) = grouping_by_card_number.get(trips_card_number) {
            if trips.len() != NUM_CARDS_IN_TRIPLET {
                return None;
            }
            trips_cards.extend(trips.clone());
        } else {
            return None;
        }
        for card in cards {
            if card.number != *trips_card_number {
                trips_cards.push(card.clone());
            }
            if trips_cards.len() == NUM_CARDS_IN_HAND {
                return Some(trips_cards);
            }
        }
        return None;
    }

    fn contains_two_pair(
        grouping_by_card_number: &HashMap<CardNumber, Vec<Card>>,
        cards: &Vec<Card>,
    ) -> Option<Vec<Card>> {
        let mut first_pair_card_number = &CardNumber::Two;
        let mut second_pair_card_number = &CardNumber::Two;
        let mut two_pair_cards: Vec<Card> = vec![];
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > first_pair_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_PAIR
            {
                first_pair_card_number = card_number;
            }
        }
        if let Some(first_two_pair) = grouping_by_card_number.get(first_pair_card_number) {
            if first_two_pair.len() != NUM_CARDS_IN_PAIR {
                return None;
            }
            two_pair_cards.extend(first_two_pair.clone());
        } else {
            return None;
        }

        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > second_pair_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_PAIR
                && card_number != first_pair_card_number
            {
                second_pair_card_number = card_number;
            }
        }
        if let Some(second_two_pair) = grouping_by_card_number.get(second_pair_card_number) {
            if second_two_pair.len() != NUM_CARDS_IN_PAIR
                || second_pair_card_number == first_pair_card_number
            {
                return None;
            }
            two_pair_cards.extend(second_two_pair.clone());
        } else {
            return None;
        }

        for card in cards {
            if card.number != *first_pair_card_number && card.number != *second_pair_card_number {
                two_pair_cards.push(card.clone());
            }
            if two_pair_cards.len() == NUM_CARDS_IN_HAND {
                return Some(two_pair_cards);
            }
        }
        return None;
    }

    fn contains_pair(
        grouping_by_card_number: &HashMap<CardNumber, Vec<Card>>,
        cards: &Vec<Card>,
    ) -> Option<Vec<Card>> {
        let mut pair_card_number = &CardNumber::Two;
        let mut pair_cards: Vec<Card> = vec![];
        for (card_number, cards_in_card_number) in grouping_by_card_number.into_iter() {
            if card_number.get_value() > pair_card_number.get_value()
                && cards_in_card_number.len() >= NUM_CARDS_IN_PAIR
            {
                pair_card_number = card_number;
            }
        }
        if let Some(pair) = grouping_by_card_number.get(pair_card_number) {
            if pair.len() != NUM_CARDS_IN_PAIR {
                return None;
            }
            pair_cards.extend(pair.clone());
        } else {
            return None;
        }
        for card in cards {
            if card.number != *pair_card_number {
                pair_cards.push(card.clone());
            }
            if pair_cards.len() == NUM_CARDS_IN_HAND {
                return Some(pair_cards);
            }
        }
        return None;
    }

    fn get_card_combinations(
        player_cards: Vec<Vec<Card>>,
        community_cards: Vec<Card>,
    ) -> Vec<Vec<Card>> {
        let card_combiations = player_cards
            .iter()
            .map(|cards| {
                let mut full_set_cards = vec![];
                full_set_cards.extend(community_cards.clone());
                full_set_cards.extend(cards.clone());
                return full_set_cards;
            })
            .collect::<Vec<Vec<Card>>>();

        return card_combiations;
    }

    fn group_cards_by_suit(cards: &Vec<Card>) -> HashMap<Suit, Vec<Card>> {
        let mut cards_by_suit: HashMap<Suit, Vec<Card>> = HashMap::new();

        for card in cards {
            let card_suit = card.suit.clone();
            cards_by_suit
                .entry(card_suit)
                .or_insert_with(Vec::new)
                .push(card.clone());
        }

        return cards_by_suit;
    }

    fn group_cards_by_number(cards: &Vec<Card>) -> HashMap<CardNumber, Vec<Card>> {
        let mut cards_by_number: HashMap<CardNumber, Vec<Card>> = HashMap::new();

        for card in cards {
            let card_number = card.number.clone();
            cards_by_number
                .entry(card_number)
                .or_insert_with(Vec::new)
                .push(card.clone());
        }

        return cards_by_number;
    }

    fn get_best_hand(cards: &Vec<Card>) -> Hand {
        // get grouping by suit
        let grouping_by_suit = HandAnalyser::group_cards_by_suit(cards);

        // check for straight flush -> grouping by suit
        let straight_flush = HandAnalyser::contains_straight_flush(&grouping_by_suit);
        if let Some(straight_flush_cards) = straight_flush {
            let high_card = &straight_flush_cards[0];
            if high_card.number == CardNumber::Ace {
                return Hand::RoyalFlush(straight_flush_cards);
            }
            return Hand::StraightFlush(straight_flush_cards);
        }

        // get grouping by card number
        let grouping_by_card_number = HandAnalyser::group_cards_by_number(cards);

        // check for quads
        let quads = HandAnalyser::contains_quads(&grouping_by_card_number, cards);
        if let Some(quad_cards) = quads {
            return Hand::Quads(quad_cards);
        }

        // check for full house -> grouping by number
        let full_house = HandAnalyser::contains_full_house(&grouping_by_card_number);
        if let Some(full_house_cards) = full_house {
            return Hand::FullHouse(full_house_cards);
        }

        // check for for flush -> grouping by by suit
        let flush = HandAnalyser::contains_flush(&grouping_by_suit);
        if let Some(flush_cards) = flush {
            return Hand::Flush(flush_cards);
        }

        // check for straight -> grouping by number
        let straight = HandAnalyser::contains_straight(cards);
        if let Some(straight_cards) = straight {
            return Hand::Straight(straight_cards);
        }

        // check for trips -> grouping by number
        let trips = HandAnalyser::contains_trips(&grouping_by_card_number, cards);
        if let Some(trips_cards) = trips {
            return Hand::Trips(trips_cards);
        }
        // check for two pair -> grouping by number
        let two_pair = HandAnalyser::contains_two_pair(&grouping_by_card_number, cards);
        if let Some(two_pair_cards) = two_pair {
            return Hand::TwoPair(two_pair_cards);
        }

        // check for pair -> grouping by number
        let pair = HandAnalyser::contains_pair(&grouping_by_card_number, cards);
        if let Some(pair_cards) = pair {
            return Hand::Pair(pair_cards);
        }

        // return high card
        return Hand::HighCard(cards[0..NUM_CARDS_IN_HAND].to_vec());
    }

    fn get_winning_hand_in_hand_type_helper(
        hands: &Vec<(usize, Vec<Card>)>,
        indicies: &[usize],
    ) -> Vec<(usize, Vec<Card>)> {
        let mut best_hands = hands.clone();
        for index in indicies {
            if best_hands.len() == 1 {
                return best_hands;
            }
            // check hand at index and get hands/hand_indicies with highest card number at index
            let mut highest_card_number: u32 = 0;
            for i in 0..best_hands.len() {
                let card_number = (&best_hands[i].1[*index].number).get_value();
                if card_number > highest_card_number {
                    highest_card_number = card_number;
                }
            }
            best_hands = best_hands
                .into_iter()
                .filter(|(_, cards)| cards[*index].number.get_value() == highest_card_number)
                .collect::<Vec<(usize, Vec<Card>)>>();
        }
        return best_hands;
    }

    fn get_winning_hand_in_hand_type(hands: &Vec<(usize, Hand)>) -> Vec<(usize, Hand)> {
        let hand_type = &hands[0].1;
        let mapped_hands = &hands
            .iter()
            .map(|(index, hand)| (*index, hand.get_cards()))
            .collect::<Vec<(usize, Vec<Card>)>>();

        match hand_type {
            Hand::RoyalFlush(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &STRAIGHT_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::RoyalFlush(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::StraightFlush(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &STRAIGHT_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::StraightFlush(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::Quads(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &QUAD_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::Quads(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::FullHouse(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &FULL_HOUSE_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::FullHouse(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::Flush(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &HIGH_CARD_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::Flush(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::Straight(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &STRAIGHT_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::Straight(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::Trips(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &TRIPS_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::Trips(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::TwoPair(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &TWO_PAIR_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::TwoPair(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::Pair(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &PAIR_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::Pair(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
            Hand::HighCard(_) => {
                let cards = HandAnalyser::get_winning_hand_in_hand_type_helper(
                    mapped_hands,
                    &HIGH_CARD_INDICIES,
                );
                return cards
                    .iter()
                    .map(|(index, cards)| (*index, Hand::HighCard(cards.clone())))
                    .collect::<Vec<(usize, Hand)>>();
            }
        };
    }

    pub fn get_winning_hands(&self, verbose: bool) -> Vec<(String, Hand)> {
        let hands = self
            .card_combinations
            .iter()
            .enumerate()
            .map(|(index, cards)| (index, HandAnalyser::get_best_hand(cards)))
            .collect::<Vec<(usize, Hand)>>();

        let mut strongest_hands_value: u32 = 0;
        for (_, hand) in &hands {
            if hand.get_value() > strongest_hands_value {
                strongest_hands_value = hand.get_value();
            }
        }

        let strongest_hands = hands
            .into_iter()
            .filter(|(_, hand)| hand.get_value() == strongest_hands_value)
            .collect::<Vec<(usize, Hand)>>();

        // for (player_num, hand) in &strongest_hands {
        //     println!("Player: {}, has a {}", player_num, hand.to_string());
        // }
        // println!("Winners:");
        let winning_hands = HandAnalyser::get_winning_hand_in_hand_type(&strongest_hands);
        // for (player_num, hand) in &winning_hands {
        //     println!("Player: {}, has a {}", player_num, hand.to_string());
        //     println!(
        //         "Player {} hand: {}",
        //         player_num,
        //         Card::cards_to_single_string(&self.player_cards[*player_num])
        //     )
        // }
        let winning_player_hands = winning_hands
            .iter()
            .map(|(player_num, hand)| {
                let cards_string = if verbose {
                    Card::cards_to_single_string(&self.player_cards[*player_num])
                } else {
                    Card::get_shorten_hand_string(&self.player_cards[*player_num])
                };
                (cards_string, hand.clone())
            })
            .collect::<Vec<(String, Hand)>>();
        return winning_player_hands;
    }
}
