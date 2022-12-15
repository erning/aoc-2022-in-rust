use std::env;
use std::fmt::Display;
use std::time::SystemTime;

fn main() {
    macro_rules! puzzle {
        ($mod:ident, $title:expr) => {
            (
                $title,
                |input| Box::new(aoc::$mod::part_one(input)),
                |input| Box::new(aoc::$mod::part_two(input)),
            )
        };
    }

    type SolverFn = fn(&str) -> Box<dyn Display>;

    let puzzles: Vec<(&str, SolverFn, SolverFn)> = vec![
        puzzle!(day01, "Calorie Counting"),
        puzzle!(day02, "Rock Paper Scissors"),
        puzzle!(day03, "Rucksack Reorganization"),
        puzzle!(day04, "Camp Cleanup"),
        puzzle!(day05, "Supply Stacks"),
        puzzle!(day06, "Tuning Trouble"),
        puzzle!(day07, "No Space Left On Device"),
        puzzle!(day08, "Treetop Tree House"),
        puzzle!(day09, "Rope Bridge"),
        puzzle!(day10, "Cathode-Ray Tube"),
        puzzle!(day11, "Monkey in the Middle"),
        puzzle!(day12, "Hill Climbing Algorithm"),
        puzzle!(day13, "Distress Signal"),
        puzzle!(day14, "Regolith Reservoir"),
        puzzle!(day15, "Beacon Exclusion Zone"),
    ];

    let filename = match env::args().find(|a| a == "--example") {
        None => "input",
        Some(_) => "example",
    };

    let show_time = env::args().any(|a| a == "--time");

    let mut days: Vec<usize> =
        env::args().filter_map(|a| a.parse().ok()).collect();

    if days.is_empty() {
        days = (1..=puzzles.len()).collect();
    }

    for day in days {
        let bt = SystemTime::now();

        let (title, part1, part2) = &puzzles[day - 1];
        let input = aoc::read_as_string(day as u8, filename);
        let input = input.as_str();

        println!("--- Day {}: {} ---", day, title);
        println!("Part One: {}", part1(input));
        println!("Part Two: {}", part2(input));

        if show_time {
            let et = SystemTime::now();
            let duration = et.duration_since(bt).unwrap_or_default();
            println!("Duration: {:?}", duration);
        }
        println!();
    }
}
