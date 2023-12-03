use std::{fs, collections::HashMap};
use rayon::prelude::*;

const FILE_PATH: &str = "input.txt";

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

const RED_STR: &str = "red";
const GREEN_STR: &str = "green";
const BLUE_STR: &str = "blue";

fn main() {
    let input = fs::read_to_string(FILE_PATH).expect("Failed to read file");

    // Part 1
    println!("Part 1: {}", &parts(&input, false));
    
    // Part 2
    println!("Part 2: {}", &parts(&input, true));
}

fn parts(input: &str, minimum_values: bool) -> u32 {

    let sum: u32 = input
    .lines()
    .par_bridge()
    .map(|line| {
        let mut iter = line.split(": ");
        let game_name = iter.next().unwrap();
        let rounds = iter.next().unwrap().split("; ");
        let mut game_cube_colors = HashMap::<&str, u32>::new();
        for round in rounds {
            let mut cube_colors = HashMap::<&str, u32>::new();
            let cubes = round.split(", ");
            for cube in cubes {
                let mut iter = cube.split(" ");
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next();

                if minimum_values {
                    println!("{}: {} {}", game_name, num, color.unwrap());
                    match color.unwrap() {
                        RED_STR => game_cube_colors.entry(RED_STR).and_modify(|n| if *n < num { *n = num }).or_insert(num),
                        GREEN_STR => game_cube_colors.entry(GREEN_STR).and_modify(|n| if *n < num { *n = num }).or_insert(num),
                        BLUE_STR => game_cube_colors.entry(BLUE_STR).and_modify(|n| if *n < num { *n = num }).or_insert(num),
                        _ => panic!("Invalid color")
                    };
                } else {
                    match color.unwrap() {
                        RED_STR => cube_colors.entry(RED_STR).and_modify(|n| *n += num).or_insert(num),
                        GREEN_STR => cube_colors.entry(GREEN_STR).and_modify(|n| *n += num).or_insert(num),
                        BLUE_STR => cube_colors.entry(BLUE_STR).and_modify(|n| *n += num).or_insert(num),
                        _ => panic!("Invalid color")
                    };
                }
            }

            if !minimum_values
                && (*cube_colors.get(RED_STR).unwrap_or(&0) > RED_CUBES
                || *cube_colors.get(GREEN_STR).unwrap_or(&0) > GREEN_CUBES
                || *cube_colors.get(BLUE_STR).unwrap_or(&0) > BLUE_CUBES) {
                println!("{}: {}", game_name, round);
                return 0;
            }
        }

        // Part 2 return
        if minimum_values {
            println!("{}: {:?}", game_name, game_cube_colors);
            return game_cube_colors.values().product::<u32>();
        }

        return game_name.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
    })
    .sum();

    sum
}

#[test]
fn test_part1() {
    let test = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();
    let result = parts(&test, false);
    assert_eq!(result, 8)
}
#[test]
fn test_part2() {
    let test = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();
    let result = parts(&test, true);
    assert_eq!(result, 2286)
}