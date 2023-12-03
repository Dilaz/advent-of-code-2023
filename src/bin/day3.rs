use std::{cmp, collections::HashMap, fs, sync::RwLock};
use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const FILE_PATH: &str = "input2.txt";

const EMPTY_CHAR: char = '.';

#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(long, short = '1', default_value = FILE_PATH)]
    pub input_part1: String,

    #[clap(long, short = '2', default_value = FILE_PATH)]
    pub input_part2: String,
}
fn main() {
    let args: Opts = Opts::parse();
    let input_part1 = fs::read_to_string(args.input_part1).expect("Failed to read file");
    println!("Part 1: {}", parts(&input_part1, None));
    let input_part2 = fs::read_to_string(args.input_part2).expect("Failed to read file");
    println!("Part 2: {}", parts(&input_part2, Some('*')));
}

#[derive(Debug, Copy, Clone)]
struct NumberLocation {
    number: u32,
    start: (usize, usize),
}

fn parts(input: &str, multiply_symbol: Option<char>) -> u32 {
    
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut symbol_locations: Vec<(usize, usize, bool)> = vec![];
    let mut number_locations: HashMap<(usize, usize), NumberLocation> = HashMap::new();
    
    for (y, line) in map.iter().enumerate() {
        let mut current_num: Option<u32> = None;
        let mut num_start: (usize, usize) = (0, 0);
        for (x, c) in line.iter().enumerate() {
            // Check if this is a symbol or a number
            if c.is_digit(10) {
                if current_num.is_none() {
                    current_num = Some(c.to_digit(10).unwrap());
                    num_start = (x, y);
                } else {
                    current_num = Some(current_num.unwrap() * 10 + c.to_digit(10).unwrap());
                }
            } else {
                if c != &EMPTY_CHAR {
                    symbol_locations.push((x, y, Some(*c) == multiply_symbol));
                }
                if let Some(num) = current_num {
                    for current_x in num_start.0..x {
                        number_locations.insert((current_x, y), NumberLocation { number: num, start: num_start});
                    }
                    current_num = None;
                }
            }
        }

        // Save the line-ending number
        if let Some(num) = current_num {
            for current_x in num_start.0..line.len() {
                number_locations.insert((current_x, y), NumberLocation { number: num, start: num_start});
            }
        }
    }

    println!("Numbers and symbols mapped!");

    // Go through each symbol and find the adjanced numbers
    let adjanced_numbers = RwLock::new(HashMap::<(usize, usize), u32>::new());
    let sum = RwLock::new(0);
    (0..symbol_locations.len()).into_par_iter().for_each(|index| {
        let (symbol_x, symbol_y, gear) = symbol_locations[index];
        let mut current_adjanced = HashMap::<(usize, usize), u32>::new();
        // Travel around the coordinate
        for current_x in cmp::max(symbol_x, 1) - 1..=symbol_x + 1 {
            for current_y in cmp::max(symbol_y, 1) - 1..=symbol_y + 1 {
                if let Some(num) = number_locations.get(&(current_x, current_y)) {
                    if !current_adjanced.contains_key(&num.start) { 
                        current_adjanced.insert(num.start, num.number);
                    } else if gear && adjanced_numbers.read().unwrap().contains_key(&num.start) {
                        adjanced_numbers.write().unwrap().remove(&num.start);
                        *sum.write().unwrap() -= num.number;
                    }
                }
            }
        }

        // Special gear ratio for part 2
        if gear && current_adjanced.len() == 2 {
            *sum.write().unwrap() += current_adjanced.values().product::<u32>();
        } else if multiply_symbol.is_none() { // Only count if this is part 1
            *sum.write().unwrap() += current_adjanced.values().sum::<u32>();
        }

        current_adjanced.iter().for_each(|(key, value)| {
            adjanced_numbers.write().unwrap().insert(*key, *value);
        });
    });

    let total_sum = sum.read().unwrap();
    *total_sum
}

#[test]
fn test_part1() {
    let test = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.to_string();
    let result = parts(&test, None);
    assert_eq!(result, 4361)
}

#[test]
fn test_part2() {
    let test = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.to_string();
    let result = parts(&test, Some('*'));
    assert_eq!(result, 467835)
}