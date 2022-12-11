use std::collections::VecDeque;

#[derive(Debug)]
struct Note {
    items: VecDeque<i64>,
    operation: (char, Option<i64>),
    divisible_by: i64,
    throw_to: (usize, usize),
}

fn parse_input(input: &str) -> Vec<Note> {
    input.lines().into_iter().collect::<Vec<&str>>()[..]
        .chunks(7)
        .map(|v| Note {
            items: v[1][18..]
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect(),
            operation: (
                v[2][23..24].chars().next().unwrap(),
                v[2][25..].parse().ok(),
            ),
            divisible_by: v[3][21..].parse().unwrap(),
            throw_to: (
                v[4][29..].parse().unwrap(),
                v[5][30..].parse().unwrap(),
            ),
        })
        .collect()
}

fn inspect(notes: Vec<Note>, divided_by_three: bool, round: usize) -> i64 {
    let mut notes = notes;
    let mut times: Vec<i64> = vec![0; notes.len()];
    let modulo: i64 = notes.iter().map(|v| v.divisible_by).product();
    for _ in 0..round {
        for i in 0..notes.len() {
            while let Some(item) = notes[i].items.pop_front() {
                let new = match notes[i].operation {
                    ('+', Some(v)) => item + v,
                    ('*', Some(v)) => item * v,
                    ('+', None) => item + item,
                    ('*', None) => item * item,
                    _ => panic!("unknown operation"),
                } % modulo
                    / if divided_by_three { 3 } else { 1 };
                let throw_to = if new % notes[i].divisible_by == 0 {
                    notes[i].throw_to.0
                } else {
                    notes[i].throw_to.1
                };
                notes[throw_to].items.push_back(new);
                times[i] += 1;
            }
        }
    }
    times.sort_unstable_by(|a, b| b.cmp(a));
    times[0] * times[1]
}

pub fn part_one(input: &str) -> i64 {
    let notes = parse_input(input);
    inspect(notes, true, 20)
}

pub fn part_two(input: &str) -> i64 {
    let notes = parse_input(input);
    inspect(notes, false, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 10605);
        assert_eq!(part_two(&input), 2713310158);
    }
}
