use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_input(input: &str) -> HashMap<&str, (i32, Vec<&str>)> {
    input
        .lines()
        .map(|v| {
            let s = v.split(';').collect::<Vec<&str>>();
            let name = &s[0][6..8];
            let rate = s[0][23..].parse::<i32>().unwrap();
            let nexts = s[1]
                .split([' ', ','])
                .filter(|v| !v.is_empty())
                .enumerate()
                .filter(|&(i, _)| i >= 4)
                .map(|(_, v)| v)
                .collect::<Vec<&str>>();
            (name, (rate, nexts))
        })
        .collect()
}

fn explore(
    minute: i32,
    start: &str,
    valves: &HashMap<&str, (i32, Vec<&str>)>,
    max: &mut i32,
) {
    let mut paths: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for a in valves.keys() {
        let mut p: HashMap<&str, i32> = HashMap::new();
        let mut queue: VecDeque<(i32, &str)> = VecDeque::new();
        queue.push_back((0, a));
        while let Some((m, b)) = queue.pop_front() {
            p.insert(b, m);
            let (_, nexts) = valves.get(b).unwrap();
            for n in nexts {
                if p.contains_key(n) {
                    continue;
                }
                queue.push_back((m + 1, n));
            }
        }
        paths.insert(a, p);
    }

    // minute, released, valve, previous-valves, releasing-per-minute
    let mut queue: BinaryHeap<(i32, i32, &str, Vec<&str>, i32)> =
        BinaryHeap::new();
    queue.push((minute, 0, start, vec![], 0));

    let pendings: Vec<&str> = valves
        .iter()
        .filter(|(_, (v, _))| *v > 0)
        .map(|(v, _)| *v)
        .collect();

    while let Some((remaining, released, valve, prev, rpm)) = queue.pop() {
        if remaining <= 0 {
            continue;
        }
        for next in pendings
            .iter()
            .filter(|v| *v != &valve && !prev.contains(v))
        {
            let (rate, _) = valves.get(next).unwrap();
            // let key = [valve, next].join("");
            let m = 1 + paths.get(valve).unwrap().get(next).unwrap();
            let mut opened = prev.clone();
            opened.push(valve);
            queue.push((
                (remaining - m).max(0),
                released + rpm * m,
                next,
                opened,
                rpm + rate,
            ));
        }
        if *max < released + remaining * rpm {
            *max = released + remaining * rpm;
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let valves = parse_input(input);
    let mut max = i32::MIN;
    explore(30, "AA", &valves, &mut max);
    max
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
