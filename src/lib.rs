use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn read_as_string(day: u8, filename: &str) -> String {
    let filename = format!("inputs/{:02}-{}.txt", day, filename);
    fs::read_to_string(filename).unwrap()
}

pub fn read_input(day: u8) -> String {
    read_as_string(day, "input")
}
pub fn read_example(day: u8) -> String {
    read_as_string(day, "example")
}
