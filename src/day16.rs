use std::collections::HashMap;

fn parse_input(
    input: &str,
) -> (HashMap<&str, u32>, HashMap<u32, (i32, Vec<u32>)>) {
    let mut codes: HashMap<&str, u32> = HashMap::new();
    input
        .lines()
        .filter(|v| &v[23..24] != "0")
        .map(|v| &v[6..8])
        .enumerate()
        .for_each(|(i, v)| {
            codes.insert(v, 1 << i);
        });
    let start = 1 << codes.len();
    input
        .lines()
        .filter(|v| &v[23..24] == "0")
        .map(|v| &v[6..8])
        .enumerate()
        .for_each(|(i, v)| {
            codes.insert(v, i as u32 + start);
        });

    let cave: HashMap<u32, (i32, Vec<u32>)> = input
        .lines()
        .map(|v| {
            let s = v.split(';').collect::<Vec<&str>>();
            let name = *codes.get(&s[0][6..8]).unwrap();
            let rate = s[0][23..].parse::<i32>().unwrap();
            let nexts = s[1]
                .split([' ', ','])
                .filter(|v| !v.is_empty())
                .enumerate()
                .filter(|&(i, _)| i >= 4)
                .map(|(_, v)| *codes.get(v).unwrap())
                .collect::<Vec<u32>>();
            (name, (rate, nexts))
        })
        .collect();

    (codes, cave)
}

fn explore(
    minute: usize,
    start: u32,
    cave: &HashMap<u32, (i32, Vec<u32>)>,
) -> i32 {
    // (valve, opened) -> released
    let mut visited: HashMap<(u32, u32), i32> = HashMap::new();
    // (valve, opened, pressure, released)
    let mut prev: Vec<(u32, u32, i32, i32)> = vec![(start, 0, 0, 0)];

    for _ in 0..minute {
        let mut curr: Vec<(u32, u32, i32, i32)> = Vec::new();
        fn push(
            curr: &mut Vec<(u32, u32, i32, i32)>,
            visited: &mut HashMap<(u32, u32), i32>,
            valve: u32,
            opened: u32,
            pressure: i32,
            released: i32,
        ) {
            match visited.get(&(valve, opened)) {
                Some(v) if *v >= released => {}
                _ => {
                    curr.push((valve, opened, pressure, released));
                    visited.insert((valve, opened), released);
                }
            }
        }

        for (valve, opened, pressure, released) in prev.into_iter() {
            let (rate, nexts) = cave.get(&valve).unwrap();
            let released = released + pressure;

            // open valve
            if *rate > 0 && (opened & valve == 0) {
                push(
                    &mut curr,
                    &mut visited,
                    valve,
                    opened | valve,
                    pressure + *rate,
                    released,
                );
            }

            // stay
            visited.insert((valve, opened), released);

            // move to next
            for next in nexts.into_iter() {
                push(
                    &mut curr,
                    &mut visited,
                    *next,
                    opened,
                    pressure,
                    released,
                );
            }
        }
        prev = curr;
        // let max = prev.iter().map(|(_, _, _, v)| *v).max().unwrap();
        // println!("{}: {}", i + 1, max);
    }
    prev.iter().map(|(_, _, _, v)| *v).max().unwrap()
}

pub fn part_one(input: &str) -> i32 {
    let (codes, cave) = parse_input(input);
    let start = *codes.get("AA").unwrap();
    explore(30, start, &cave)
}

pub fn part_two(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(16);
        assert_eq!(part_one(&input), 1651);
        assert_eq!(part_two(&input), 1707);
    }
}
