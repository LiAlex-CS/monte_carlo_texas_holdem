mod args;
mod card;
mod dealer;
mod file_io;
mod hand_analyser;
mod logger;

use args::Args;
use card::Card;
use dealer::Dealer;
use file_io::FileIO;
use hand_analyser::HandAnalyser;
use logger::Logger;

use std::collections::HashMap;

use clap::Parser;

const THOUSAND: u32 = 1000;
const _NUM_THREADS: usize = 10;

const NUM_STATS: usize = 2;
const NUM_TOTAL_STATS: usize = 2;

pub enum Stats {
    NumberOfHands,
    NumberOfWins,
}

impl Stats {
    fn get_index(&self) -> usize {
        match self {
            Self::NumberOfHands => 0,
            Self::NumberOfWins => 1,
        }
    }
}

pub enum TotalStats {
    NumberOfIterations,
    NumberOfUniqueHands,
}

impl TotalStats {
    fn get_index(&self) -> usize {
        match self {
            Self::NumberOfIterations => 0,
            Self::NumberOfUniqueHands => 1,
        }
    }
}

// fn _get_combined_stats(
//     all_stats: Vec<HashMap<String, [u32; NUM_STATS]>>,
// ) -> HashMap<String, [u32; NUM_STATS]> {
//     let mut combined_stats = HashMap::new();
//     for record in all_stats {
//         combined_stats.extend(record);
//     }
//     return combined_stats;
// }

fn simulate(num_players: u32, verbose: bool, hand_stats: &mut HashMap<String, [u32; NUM_STATS]>) {
    let mut dealer = Dealer::new();
    let (community_cards, player_cards) = dealer.deal(num_players);

    let player_cards_strings = player_cards
        .iter()
        .map(|cards| match verbose {
            true => Card::cards_to_single_string(cards),
            false => Card::get_shorten_hand_string(cards),
        })
        .collect::<Vec<String>>();

    for player_cards_string in player_cards_strings {
        // hand_stats.entry(player_cards_string).or_insert([0, 0]);
        match hand_stats.entry(player_cards_string) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let counts = entry.get_mut();
                counts[Stats::NumberOfHands.get_index()] += 1;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mut counts = [0; NUM_STATS];
                counts[Stats::NumberOfHands.get_index()] = 1;
                entry.insert(counts);
            }
        }
    }

    // println!("Community Cards:");
    // println!("    {:?}", Card::cards_to_string(&community_cards));
    // println!("Player Cards:");
    // for (index, player_hand) in player_cards.iter().enumerate() {
    //     println!("    player {}:", { index });
    //     println!("        hand: {:?}", Card::cards_to_string(player_hand))
    // }

    let analyser = HandAnalyser::new(community_cards, player_cards);
    let winning_hands = analyser.get_winning_hands(verbose);
    for (hand, _hand_type) in winning_hands {
        match hand_stats.entry(hand) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let counts = entry.get_mut();
                counts[1] += 1;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mut counts = [0; NUM_STATS];
                counts[Stats::NumberOfHands.get_index()] = 1;
                counts[Stats::NumberOfWins.get_index()] = 1;
                entry.insert(counts);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let logger = Logger::new(args.debug);

    let verbose_str = if args.verbose { "_verbose" } else { "" };

    let file = FileIO::new(format!(
        "./output_{}_players{}.csv",
        args.num_players, verbose_str
    ));

    let (mut hand_stats, mut total_stats) = file.read_from_file().expect("Error reading from file");

    for _ in 0..args.num_thousand_iterations * THOUSAND {
        simulate(args.num_players, args.verbose, &mut hand_stats)
    }

    total_stats[TotalStats::NumberOfIterations.get_index()] +=
        args.num_thousand_iterations * THOUSAND;
    total_stats[TotalStats::NumberOfUniqueHands.get_index()] =
        hand_stats.keys().len().try_into().unwrap();

    file.clear_file().unwrap();

    logger.print(format!(
        "Number of unique hands: {}",
        &hand_stats.keys().len()
    ));

    file.write_to_file(format!("{}", FileIO::array_to_string(&total_stats)))
        .unwrap();

    for (hand, counts) in &hand_stats {
        logger.print(format!("{}, {:?}", hand, counts));
        file.write_to_file(format!("{},{}", hand, FileIO::array_to_string(counts)))
            .unwrap();
    }

    // let mut community_cards = vec![];
    // for i in 2..=5 {
    //     community_cards.push(Card {
    //         number: CardNumber::match_int_with_card_num(i),
    //         suit: Suit::Clubs,
    //     });
    // }
    // community_cards.push(Card {
    //     number: CardNumber::match_int_with_card_num(10),
    //     suit: Suit::Spades,
    // });
    // let player_cards = vec![vec![
    //     Card {
    //         number: CardNumber::match_int_with_card_num(2),
    //         suit: Suit::Hearts,
    //     },
    //     Card {
    //         number: CardNumber::match_int_with_card_num(8),
    //         suit: Suit::Diamonds,
    //     },
    // ]];

    // let analyser = HandAnalyser::new(community_cards, player_cards);
    // analyser.get_winning_hand();
}
