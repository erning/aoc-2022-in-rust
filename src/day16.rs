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

fn explore(input: &str, start: &[&str], minutes: i32) -> i32 {
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

    let start: Vec<(usize, i32)> = start
        .iter()
        .map(|start| {
            (
                valves
                    .iter()
                    .enumerate()
                    .find(|(_, (name, _, _))| name == start)
                    .map(|(i, _)| i)
                    .unwrap(),
                minutes,
            )
        })
        .collect();

    let mut visited: HashMap<(Vec<usize>, u64), i32> = HashMap::new();

    let mut max = 0;
    dfs(start, 0, 0, 0, &graph, &nexts, &mut visited, &mut max);

    fn dfs(
        ids: Vec<(usize, i32)>,
        idx: usize,
        opened: u64,
        estimated: i32,
        graph: &Vec<Vec<Option<i32>>>,
        nexts: &Vec<(usize, u64, i32)>,
        visited: &mut HashMap<(Vec<usize>, u64), i32>,
        max: &mut i32,
    ) {
        if estimated > *max {
            *max = estimated;
        }
        let (id, time) = ids[idx];
        if time <= 0 {
            return;
        }

        let mut k: Vec<usize> = ids.iter().map(|(v, _)| *v).collect();
        k.sort_unstable();
        let k: (Vec<usize>, u64) = (k, opened);
        if let Some(e) = visited.get_mut(&k) {
            if estimated > *e {
                *e = estimated;
            } else {
                return;
            }
        } else {
            visited.insert(k, estimated);
        }

        for &(next_id, next_mask, next_rate) in nexts.iter() {
            if opened & next_mask != 0 {
                continue;
            }
            if let Some(distance) = graph[id][next_id] {
                let opened = opened | next_mask;
                let time = (time - distance - 1).max(0);
                let estimated = estimated + next_rate * time;
                let mut ids = ids.clone();
                ids[idx] = (next_id, time);
                let idx = (idx + 1) % ids.len();
                dfs(ids, idx, opened, estimated, graph, nexts, visited, max);
            }
        }
    }

    max
}

pub fn part_one(input: &str) -> i32 {
    explore(input, &["AA"], 30)
}

pub fn part_two(input: &str) -> i32 {
    explore(input, &["AA"; 2], 26)
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
