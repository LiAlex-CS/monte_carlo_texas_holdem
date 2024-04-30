pub const NUM_SUITS: u32 = 4;
pub const NUM_CARD_NUMBERS: u32 = 14;
pub const TWO: u32 = 2;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Suit {
    Diamonds,
    Spades,
    Hearts,
    Clubs,
}

impl Suit {
    pub fn match_int_with_suit(number: u32) -> Suit {
        match number {
            0 => Self::Diamonds,
            1 => Self::Spades,
            2 => Self::Hearts,
            3 => Self::Clubs,
            _ => Self::Clubs,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Diamonds => "Diamonds".to_string(),
            Self::Clubs => "Clubs".to_string(),
            Self::Hearts => "Hearts".to_string(),
            Self::Spades => "Spades".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum CardNumber {
    BottomAce,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardNumber {
    pub fn match_int_with_card_num(number: u32) -> CardNumber {
        match number {
            1 => Self::BottomAce,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            14 => Self::Ace,
            _ => Self::Ace,
        }
    }

    pub fn get_value(&self) -> u32 {
        match self {
            Self::BottomAce => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::BottomAce => "Ace".to_string(),
            Self::Two => "Two".to_string(),
            Self::Three => "Three".to_string(),
            Self::Four => "Four".to_string(),
            Self::Five => "Five".to_string(),
            Self::Six => "Six".to_string(),
            Self::Seven => "Seven".to_string(),
            Self::Eight => "Eight".to_string(),
            Self::Nine => "Nine".to_string(),
            Self::Ten => "Ten".to_string(),
            Self::Jack => "Jack".to_string(),
            Self::Queen => "Queen".to_string(),
            Self::King => "King".to_string(),
            Self::Ace => "Ace".to_string(),
        }
    }

    pub fn to_short_string(&self) -> String {
        match self {
            Self::BottomAce => "A".to_string(),
            Self::Two => "2".to_string(),
            Self::Three => "3".to_string(),
            Self::Four => "4".to_string(),
            Self::Five => "5".to_string(),
            Self::Six => "6".to_string(),
            Self::Seven => "7".to_string(),
            Self::Eight => "8".to_string(),
            Self::Nine => "9".to_string(),
            Self::Ten => "10".to_string(),
            Self::Jack => "J".to_string(),
            Self::Queen => "Q".to_string(),
            Self::King => "K".to_string(),
            Self::Ace => "A".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNumber,
}

impl Card {
    pub fn to_string(&self) -> String {
        return vec![self.number.to_string(), self.suit.to_string()].join(" of ");
    }

    pub fn cards_to_string(cards: &Vec<Card>) -> Vec<String> {
        return cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>();
    }

    pub fn cards_to_single_string(cards: &Vec<Card>) -> String {
        return Card::cards_to_string(cards).join(" | ");
    }

    pub fn get_shorten_hand_string(cards: &Vec<Card>) -> String {
        let card_1 = &cards[0];
        let card_2 = &cards[1];

        let shorten_card_1_num = card_1.number.to_short_string();
        let shorten_card_2_num = card_2.number.to_short_string();

        let suited_str = match card_1.suit == card_2.suit {
            true => "Suited",
            false => "Off Suit",
        };

        return format!(
            "{} | {} {}",
            shorten_card_1_num, shorten_card_2_num, suited_str
        );
    }
}
