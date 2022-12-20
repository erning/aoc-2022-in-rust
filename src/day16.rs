use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<(&str, i32, Vec<&str>)> {
    input
        .lines()
        .map(|v| {
            let s: Vec<&str> = v
                .split([' ', '=', ';', ','])
                .filter(|v| !v.is_empty())
                .collect();
            (s[1], s[5].parse().unwrap(), s[10..].to_vec())
        })
        .collect()
}

fn build_graph(valves: &[(&str, i32, Vec<&str>)]) -> Vec<Vec<Option<i32>>> {
    // name to index
    let map: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, (v, _, _))| (*v, i))
        .collect();

    let n = valves.len();
    let mut graph: Vec<Vec<Option<i32>>> = vec![vec![None; n]; n];
    for (a, _, _) in valves.iter() {
        let a = *(map.get(a).unwrap());
        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
        queue.push_back((0, a));
        while let Some((distance, b)) = queue.pop_front() {
            if graph[a][b].is_some() {
                continue;
            }
            graph[a][b] = Some(distance);
            let (_, _, neighbors) = &valves[b];
            for neighbor in neighbors {
                let neighbor = *(map.get(neighbor).unwrap());
                queue.push_back((distance + 1, neighbor));
            }
        }
    }
    graph
}

pub fn part_one(input: &str) -> i32 {
    const START: &str = "AA";
    const MINUTES: i32 = 30;

    // name, rate, neighbors
    let valves = parse_input(input);

    // graph - the distance from a valve to another
    let graph = build_graph(&valves);

    // valves has rate (valve_id, valve_mask, rate)
    let nexts: Vec<(usize, u64, i32)> = valves
        .iter()
        .enumerate()
        .filter(|(_, (_, rate, _))| *rate > 0)
        .map(|(i, (_, rate, _))| (i, *rate))
        .enumerate()
        .map(|(i, (id, rate))| (id, 1 << i, rate))
        .collect();

    // (estimated, valve, opened, time)
    let mut queue: Vec<(i32, usize, u64, i32)> = Vec::new();
    let start = valves
        .iter()
        .enumerate()
        .find(|(_, (name, _, _))| name == &START)
        .map(|(i, _)| i)
        .unwrap();

    queue.push((0, start, 0, 0));

    let mut max = 0;
    while let Some((estimated, valve, opened, time)) = queue.pop() {
        if estimated > max {
            max = estimated;
        }

        if time >= 30 {
            continue;
        }

        for &(next_id, next_mask, next_rate) in
            nexts.iter().filter(|(_, v, _)| opened & v == 0)
        {
            if let Some(distance) = graph[valve][next_id] {
                let time = (time + distance + 1).min(MINUTES);
                let estimated = estimated + next_rate * (MINUTES - time);
                queue.push((estimated, next_id, opened | next_mask, time));
            }
        }
    }
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
