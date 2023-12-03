use std::{fs, collections::HashMap};

const FILE_PATH: &str = "input.txt";

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

const RED_STR: &str = "red";
const GREEN_STR: &str = "green";
const BLUE_STR: &str = "blue";

fn main() {
    // Part 1
    parts(false);

    // Part 2
    parts(true);
}

fn parts(minimum_values: bool) {
    let content = fs::read_to_string(FILE_PATH).expect("Failed to read file");

    let sum: u32 = content
    .lines()
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

    println!("Sum: {}", sum);
}
