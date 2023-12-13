use std::collections::HashMap;

use rayon::iter::{ParallelBridge, ParallelIterator, IntoParallelIterator};

const CARDS: &str = "AKQJT98765432";
const CARDS_WITH_JOKER: &str = "AKQT98765432J";
const JOKER: char = 'J';

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    println!("Part 1: {}", &parts(&input, false));
    println!("Part 2: {}", &parts(&input, true));
}

fn parts(input: &str, check_joker: bool) -> u32 {
    let iter = input.lines();
    let mut hands = vec![];
    for line in iter {
        let (hand, score) = line.split_once(' ').unwrap();
        hands.push((hand, check_hand(&hand, check_joker), score.parse::<u32>().unwrap()));
    }

    if check_joker {
        hands = hands.into_par_iter().map(|(hand, parsed_hand, score)| {
            if let Some(new_hand) = parse_joker(hand, &parsed_hand) {
                (hand, new_hand, score)
            } else {
                (hand, parsed_hand, score)
            }
        }).collect();
    }

    hands.sort_by(|hand1, hand2| {
        if hand1.1 < hand2.1 { return std::cmp::Ordering::Less; }
        else if hand1.1 > hand2.1 { return std::cmp::Ordering::Greater; }
        else {
            for (card1, card2) in hand1.0.chars().zip(hand2.0.chars()) {
                if card1 == card2 { continue; }
                if check_joker {
                    return CARDS_WITH_JOKER.find(card1).unwrap().cmp(&CARDS_WITH_JOKER.find(card2).unwrap());
                } else {
                    return CARDS.find(card1).unwrap().cmp(&CARDS.find(card2).unwrap());
                }
            }
        }
        return std::cmp::Ordering::Equal
    });

    let hand_len = hands.len() as u32;

    hands.into_iter().zip(0..).par_bridge().map(|(hand, score)| (hand_len - score) * hand.2).sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_joker(hand: &str, parsed_hand: &Hand) -> Option<Hand> {
    let num_jokers = hand.chars().filter(|&c| c == JOKER).count();
    if num_jokers == 0 {
        return None;
    }

    match (parsed_hand, num_jokers) {
        (_, 5) => Some(Hand::FiveOfAKind),
        (Hand::FourOfAKind, 1) => Some(Hand::FiveOfAKind),
        (Hand::ThreeOfAKind, 2) => Some(Hand::FiveOfAKind),
        (Hand::ThreeOfAKind, 1) => Some(Hand::FourOfAKind),
        (Hand::TwoPair, 1) => Some(Hand::FullHouse),
        (Hand::OnePair, 3) => Some(Hand::FiveOfAKind),
        (Hand::OnePair, 2) => Some(Hand::FourOfAKind),
        (Hand::OnePair, 1) => Some(Hand::ThreeOfAKind),
        (Hand::HighCard, 4) => Some(Hand::FiveOfAKind),
        (Hand::HighCard, 3) => Some(Hand::FourOfAKind),
        (Hand::HighCard, 2) => Some(Hand::ThreeOfAKind),
        (Hand::HighCard, 1) => Some(Hand::OnePair),
        _ => None,
    }
}


fn check_hand(hand: &str, ignore_joker: bool) -> Hand {
    let mut hand_map = HashMap::new();
    for card in hand.chars() {
        hand_map.entry(card).and_modify(|e| *e += 1).or_insert(1);
    }

    if ignore_joker {
        hand_map.remove(&JOKER);
    }

    // Check for five of a kind
    if hand_map.values().any(|&v| v == 5) {
        return Hand::FiveOfAKind;
    } else if hand_map.values().any(|&v| v == 4) {
        return Hand::FourOfAKind;
    } else if hand_map.values().any(|&v| v == 3) {
        if hand_map.values().any(|&v| v == 2) {
            return Hand::FullHouse;
        } else {
            return Hand::ThreeOfAKind;
        }
    } else if hand_map.values().filter(|&v| *v == 2).count() == 2 {
        return Hand::TwoPair;
    } else if hand_map.values().any(|&v| v == 2) {
        return Hand::OnePair;
    }
    
    Hand::HighCard
}


#[test]
fn test_part1() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(parts(input, false), 6440);
}

#[test]
fn test_part2() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(parts(input, true), 5905);
}
