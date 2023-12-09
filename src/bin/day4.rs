use std::{collections::{HashSet, HashMap, VecDeque}};
use rayon::iter::{ParallelBridge, ParallelIterator};


fn main() {
    let input = include_str!("../../inputs/day4.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));
}

fn part1(input: &str) -> u32 {
    let sum = input.lines()
    .par_bridge()
    .map(|line| {
        let mut iter = line.split(": ");
        let _card_name = iter.next().unwrap();
        let mut all_numbers = iter.next().unwrap().split(" | ");
        let winning_numbers = extract_numbers(&all_numbers.next().unwrap());
        let card_numbers = extract_numbers(&all_numbers.next().unwrap());
        let matching_numbers = winning_numbers.intersection(&card_numbers).collect::<Vec<&u32>>();

        let matching_count = matching_numbers.len() as u32;

        if matching_count == 0 {
            return 0;
        }

        return 2u32.pow(matching_count - 1);

    })
    .sum();

    sum
}

fn extract_numbers(number_list: &str) -> HashSet<u32> {
    number_list.split(" ")
        .filter_map(|num| match num.trim().parse::<u32>() { Ok(n) => Some(n), _ => None })
        .collect::<HashSet<u32>>()
}

fn part2(input: &str) -> u32 {
    let games = input.lines()
        .par_bridge()
        .map(|line| {
            let mut iter = line.split(": ");
            let card_number = iter.next().unwrap().split(" ").filter(|s| !s.is_empty()).nth(1).unwrap().trim().parse::<u32>().unwrap();
            let mut all_numbers = iter.next().unwrap().split(" | ");
            let winning_numbers = extract_numbers(&all_numbers.next().unwrap());
            let card_numbers = extract_numbers(&all_numbers.next().unwrap());
            let matching_numbers = winning_numbers.intersection(&card_numbers).cloned().collect::<HashSet<u32>>();

            (card_number, matching_numbers)
        })
        .collect::<HashMap<u32, HashSet<u32>>>();

    let mut game_queue: VecDeque<u32> = VecDeque::new();
    for &game in games.keys() {
        game_queue.push_back(game);
    }

    let mut sum = 0;

    while let Some(card_number) = game_queue.pop_front() {
        sum += 1;
        let matching_numbers = games.get(&card_number).unwrap();
        let winning_games = matching_numbers.len() as u32;
        for i in 1..=winning_games {
            let game_num = card_number + i;
            if games.contains_key(&game_num) {
                game_queue.push_back(game_num);
            }
        }
    }

    sum
}

#[test]
fn test_part1() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part1(input), 13);
}

#[test]
fn test_part2() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part2(input), 30);
}