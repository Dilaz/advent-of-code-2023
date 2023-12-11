use rayon::iter::{ParallelBridge, ParallelIterator, IntoParallelIterator};

fn main() {
    let input = include_str!("../../inputs/day6.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut iter = input.lines();
    let times = iter.next().unwrap().split_whitespace().skip(1).filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let destinations = iter.next().unwrap().split_whitespace().skip(1).filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let races = times.iter().zip(destinations.iter()).collect::<Vec<_>>();

    races.into_par_iter().map(|(time, distance)| options(&time, &distance)).product::<u64>()
}

fn part2(input: &str) -> u64 {
    let mut iter = input.lines();
    let time = iter.next().unwrap().split_whitespace().skip(1).filter(|s| !s.is_empty()).collect::<String>().parse::<u64>().unwrap();
    let destination = iter.next().unwrap().split_whitespace().skip(1).filter(|s| !s.is_empty()).collect::<String>().parse::<u64>().unwrap();

    options(&time, &destination)
}

fn options(time: &u64, distance: &u64) -> u64 {
    (1..*time)
    .par_bridge()
    .map(|hold_time| (time - hold_time) * hold_time)
    .into_par_iter()
    .filter(|&race_distance| race_distance > *distance)
    .count() as u64
}


#[test]
fn test_part1() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(part1(input), 288);
}

#[test]
fn test_part2() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(part2(input), 71503);
}
