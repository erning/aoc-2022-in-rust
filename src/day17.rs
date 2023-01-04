use std::collections::HashSet;

struct Patterns {
    data: Vec<i8>,
    index: usize,
}

impl Patterns {
    fn new(input: &str) -> Patterns {
        Patterns {
            data: input
                .chars()
                .filter_map(|v| match v {
                    '<' => Some(1),
                    '>' => Some(-1),
                    _ => None,
                })
                .collect(),
            index: 0,
        }
    }

    fn next(&mut self) -> i8 {
        let v = self.data[self.index];
        self.index = (self.index + 1) % self.data.len();
        v
    }
}

struct Rocks {
    data: Vec<Vec<u16>>,
    index: usize,
}

impl Rocks {
    fn new() -> Rocks {
        Rocks {
            data: vec![
                vec![0b00111100],
                vec![0b00010000, 0b00111000, 0b00010000],
                vec![0b00111000, 0b00001000, 0b00001000],
                vec![0b00100000; 4],
                vec![0b00110000; 2],
            ],
            index: 0,
        }
    }

    fn next(&mut self) -> Vec<u16> {
        let rock = &self.data[self.index];
        self.index = (self.index + 1) % self.data.len();
        rock.clone()
    }
}

const GROUND: u16 = 0b111111111;
const WALL: u16 = 0b100000001;

fn merge_rock(chamber: &mut [u16], floor: usize, rock: &[u16]) -> bool {
    let mut merged: Vec<u16> = Vec::new();
    for (a, b) in chamber[floor - rock.len()..floor].iter().zip(rock.iter()) {
        let v = a | b;
        if v != a ^ b {
            return false;
        }
        merged.push(v);
    }
    for (i, v) in merged.into_iter().rev().enumerate() {
        chamber[floor - i - 1] = v;
    }
    true
}

fn clear_rock(chamber: &mut [u16], floor: usize, rock: &[u16]) -> bool {
    let mut cleared: Vec<u16> = Vec::new();
    for (a, b) in chamber[floor - rock.len()..floor].iter().zip(rock.iter()) {
        let v = a & !b;
        if v != a ^ b {
            return false;
        }
        cleared.push(v);
    }
    for (i, v) in cleared.into_iter().rev().enumerate() {
        chamber[floor - i - 1] = v;
    }
    true
}

fn push_rock(
    chamber: &mut [u16],
    floor: usize,
    rock: &[u16],
    step: i8,
) -> Option<Vec<u16>> {
    let mut ws = chamber[floor - rock.len()..floor].to_vec();
    if !clear_rock(&mut ws, rock.len(), rock) {
        return None;
    }
    let pushed: Vec<u16> = rock
        .iter()
        .map(|v| match step.cmp(&0) {
            std::cmp::Ordering::Greater => *v << step,
            std::cmp::Ordering::Less => *v >> -step,
            std::cmp::Ordering::Equal => *v,
        })
        .collect();
    if !merge_rock(&mut ws, rock.len(), &pushed) {
        return None;
    }
    for (i, v) in ws.into_iter().rev().enumerate() {
        chamber[floor - i - 1] = v;
    }
    Some(pushed)
}

fn fall_rock(chamber: &mut [u16], floor: usize, rock: &[u16]) -> bool {
    let mut ws = chamber[floor - (rock.len() + 1)..floor].to_vec();
    if !clear_rock(&mut ws, rock.len() + 1, rock) {
        return false;
    }
    if !merge_rock(&mut ws, rock.len(), rock) {
        return false;
    }
    for (i, v) in ws.into_iter().rev().enumerate() {
        chamber[floor - i - 1] = v;
    }
    true
}

#[allow(dead_code)]
fn print_rock(chamber: &[u16]) {
    chamber.iter().rev().for_each(|v| {
        println!("{:09b}", v);
    })
}

fn simulate<F>(
    rocks: &mut Rocks,
    patterns: &mut Patterns,
    chamber: &mut Vec<u16>,
    n: usize,
    mut is_stop: F,
) where
    F: FnMut(usize, usize) -> bool,
{
    for _ in 0..n {
        // initial
        let mut rock = rocks.next();
        chamber.append(&mut vec![WALL; rock.len() + 3]);
        let mut floor = chamber.len();
        for (i, v) in rock.iter().enumerate() {
            chamber[floor - rock.len() + i] |= *v;
        }
        loop {
            // push
            let pattern = patterns.next();
            let pushed = push_rock(chamber, floor, &rock, pattern);
            if let Some(v) = pushed {
                rock = v;
            }
            // full
            if !fall_rock(chamber, floor, &rock) {
                break;
            }
            floor -= 1;
        }
        while chamber[chamber.len() - 1] == WALL {
            chamber.pop();
        }
        if is_stop(rocks.index, patterns.index) {
            break;
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut rocks = Rocks::new();
    let mut patterns = Patterns::new(input);
    let mut chamber = vec![GROUND];
    simulate(&mut rocks, &mut patterns, &mut chamber, 2022, |_, _| false);
    chamber.len() - 1
}

pub fn part_two(input: &str) -> usize {
    let x: usize = 1_000_000_000_000;
    let mut rocks = Rocks::new();
    let mut patterns = Patterns::new(input);
    let mut chamber = vec![GROUND];
    let mut n = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    simulate(&mut rocks, &mut patterns, &mut chamber, x - n, |ri, pi| {
        n += 1;
        !visited.insert((ri, pi))
    });
    let f1 = (n, chamber.len() - 1);

    visited.clear();
    visited.insert((rocks.index, patterns.index));
    simulate(&mut rocks, &mut patterns, &mut chamber, x - n, |ri, pi| {
        n += 1;
        !visited.insert((ri, pi))
    });
    let f2 = (n, chamber.len() - 1);

    let m = (x - f1.0) % (f2.0 - f1.0);
    simulate(&mut rocks, &mut patterns, &mut chamber, m, |_, _| false);
    let f3 = (n, chamber.len() - 1);

    // a ... [b,b,b] ... c
    let a = f1.1;
    let b = (x - f1.0) / (f2.0 - f1.0) * (f2.1 - f1.1);
    let c = f3.1 - f2.1;

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(17);
        assert_eq!(part_one(&input), 3068);
        assert_eq!(part_two(&input), 1514285714288);
    }
}
