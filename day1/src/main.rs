use std::fs;

const FILE_PATH: &str = "input.txt";
const FILE_PATH_DAY_2: &str = "input2.txt";
const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Something went wrong reading the file");
    let sum = contents.split("\n")
    // Only get numbers
    .map(|x| x.chars().filter(|&c| c.is_digit(10)).collect::<String>())
    .map(|x| format!("{}{}", x.chars().nth(0).unwrap(), x.chars().last().unwrap()))
    .map(|x| x.parse::<i32>().unwrap())
    .sum::<i32>();

    println!("Part 1: {}", sum);
}
fn part2() {
    let contents = fs::read_to_string(FILE_PATH_DAY_2)
        .expect("Something went wrong reading the file");
    let sum = contents.split("\n")

    .map(|x| format!("{}{}", find_first(&x), find_last(&x)))
    .map(|x| x.parse::<u32>().unwrap())
    .sum::<u32>();

    println!("Part 2: {}", sum);
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
