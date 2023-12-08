use std::{ops::Range, sync::RwLock};
use rayon::iter::{ParallelBridge, ParallelIterator, IntoParallelRefIterator, IntoParallelIterator};


fn main() {
    let input = include_str!("../../inputs/day5.txt");
    println!("Part 1: {}", &parts(&input, false));
    println!("Part 2: {}", &parts(&input, true));
}

fn parts(input: &str, use_range_seeds: bool) -> u32 {
    let mut lines = input.split("\n\n");
    let mut seeds = lines.next().unwrap().split(": ").nth(1).unwrap().split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let range_seeds = RwLock::new(vec![]);
    if use_range_seeds {
        seeds.chunks(2).par_bridge().for_each(|nums| {
            let [start, end] = nums else { panic!("Invalid seed range") };
            (*start..*start+*end).into_par_iter().for_each(|n| {
                range_seeds.write().unwrap().push(n);
            });
        });

        seeds = range_seeds.read().unwrap().clone();
    }

    let mut maps = vec![];
    while let Some(data) = lines.next() {
        let mut lines = data.lines();
        let _ = lines.next();
        let mut map_list = vec![];
        while let Some(nums_str) = lines.next() {
            let mut nums = nums_str.split_whitespace();
            let destination = nums.next().unwrap().parse::<u32>().unwrap();
            let source = nums.next().unwrap().parse::<u32>().unwrap();
            let length = nums.next().unwrap().parse::<u32>().unwrap();
            let map = ElfMap::new(source, destination, length);
            map_list.push(map);
        }
        maps.push(map_list);
    }

    // Map each seed to destination
    let min = seeds.par_iter().map(|seed| {
        let mut seed = *seed as u32;
        for map_list in &maps {
            for map in map_list {
                 if map.map(&mut seed) {
                    break;
                 }
            }
        }
        seed as u32
    }).min();
    
    min.unwrap()
}

struct ElfMap {
    source: Range<u32>,
    diff: u32,
    neg: bool,
}

impl ElfMap {
    fn new(source: u32, destination: u32, length: u32) -> Self {
        Self {
            source: source..source+length,
            diff: if destination < source { source - destination } else { destination - source },
            neg: destination < source,
        }
    }

    fn map(&self, input: &mut u32) -> bool {
        if self.source.contains(&input) {
            if self.neg {
                *input -= self.diff;
            } else {
                *input += self.diff;
            }
            true
        } else {
            false
        }
    }

}

#[test]
fn test_part1() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(parts(input, false), 35);
}

#[test]
fn test_part2() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(parts(input, true), 46);
}