use std::fs;
use clap::Parser;

const FILE_PATH: &str = "input.txt";
const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(long, short = '1', default_value = FILE_PATH)]
    pub input_part1: String,

    #[clap(long, short = '2', default_value = FILE_PATH)]
    pub input_part2: String,
}

fn main() {
    let args = Opts::parse();
    let input_part1 = fs::read_to_string(args.input_part1)
        .expect("Something went wrong reading the file");
    println!("Part 1: {}", &part1(&input_part1));
    let input_part2 = fs::read_to_string(args.input_part2)
        .expect("Something went wrong reading the file");
    println!("Part 2: {}", &part2(&input_part2));
}

fn part1(input: &str) -> u32 {
    let sum = input.lines()
    // Only get numbers
    .map(|x| x.chars().filter(|&c| c.is_digit(10)).collect::<String>())
    .map(|x| format!("{}{}", x.chars().nth(0).unwrap(), x.chars().last().unwrap()))
    .map(|x| x.parse::<u32>().unwrap())
    .sum::<u32>();

    sum
}
fn part2(input: &str) -> u32 {
    let sum = input.lines()
    .map(|x| format!("{}{}", find_first(&x), find_last(&x)))
    .map(|x| x.parse::<u32>().unwrap())
    .sum::<u32>();

    sum
}

fn find_first(s: &str) -> u32 {
    let mut first_num: u32 = 0;
    let mut first_index: Option<usize> = None;
    for (i, &num) in NUMBERS.iter().enumerate() {
        if let Some(index) = s.find(num) {
            if first_index.is_none() || index < first_index.unwrap() {
                first_index = Some(index);
                first_num = i as u32;
            }
        }
        if let Some(index) = s.find(format!("{}", i).as_str()) {
            if first_index.is_none() || index < first_index.unwrap() {
                first_index = Some(index);
                first_num = i as u32;
            }
        }
    }

    first_num
}

fn find_last(s: &str) -> u32 {
    let mut last_num = 0;
    let mut last_index: Option<usize> = None;
    for (i, &num) in NUMBERS.iter().enumerate() {
        if let Some(index) = s.rfind(num) {
            if last_index.is_none() || index > last_index.unwrap() {
                last_index = Some(index);
                last_num = i as u32;
            }
        }
        if let Some(index) = s.rfind(format!("{}", i).as_str()) {
            if last_index.is_none() || index > last_index.unwrap() {
                last_index = Some(index);
                last_num = i as u32;
            }
        }
    }

    last_num
}


#[test]
fn test_part1() {
    let test = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#.to_string();
    let result = part1(&test);
    assert_eq!(result, 142)

}

#[test]
fn test_part2() {
    let test = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#.to_string();
    let result = part2(&test);
    assert_eq!(result, 281)

}