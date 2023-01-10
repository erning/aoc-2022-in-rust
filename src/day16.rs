use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

// adjacency list node
type Valve<'a> = (&'a str, i32, Vec<&'a str>); // name, rate, neighbors

// adjacency matrix
//   - vertice is the id of valve
//   - edge is the distance of the two valve
type Graph = Vec<Vec<Option<i32>>>;

fn parse_input(input: &str) -> Vec<Valve> {
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

fn build_graph(valves: &[Valve]) -> Graph {
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

fn search(input: &str, n: usize, minutes: i32) -> i32 {
    let valves = parse_input(input);
    let graph = build_graph(&valves);
    let rated_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, (_, rate, _))| *rate > 0)
        .map(|(i, _)| i)
        .collect();

    let mut max_released = 0;

    // (valve, time, estimated, opened)
    type Status = (usize, i32, i32, u64);

    let start: Status = (
        valves
            .iter()
            .enumerate()
            .find(|(_, (name, _, _))| *name == "AA")
            .map(|(i, _)| i)
            .unwrap(),
        minutes,
        0,
        0,
    );

    let mut visited: HashMap<(Vec<usize>, u64), i32> = HashMap::new();
    let mut queue: BinaryHeap<(i32, Vec<Status>, usize)> = BinaryHeap::new();
    (0..n).for_each(|i| queue.push((0, vec![start; n], i)));

    while let Some((_, status, m)) = queue.pop() {
        let (valve, time, estimated, _) = status[m];
        let released: i32 = status.iter().map(|(_, _, v, _)| v).sum();
        if released > max_released {
            max_released = released;
        }
        if time <= 0 {
            continue;
        }

        let opened =
            status.iter().map(|&(_, _, _, v)| v).fold(0, |a, b| a | b);

        let key = {
            let mut valves: Vec<usize> =
                status.iter().map(|(v, _, _, _)| *v).collect();
            valves.sort_unstable();
            (valves, opened)
        };
        if let Some(v) = visited.get_mut(&key) {
            if released <= *v {
                continue;
            }
            *v = released;
        } else {
            visited.insert(key, released);
        }

        rated_valves
            .iter()
            .filter(|&v| 1 << v & opened == 0)
            .filter_map(|&next| match graph[valve][next] {
                Some(wait) => Some((next, wait + 1)),
                _ => None,
            })
            .filter(|&(_, wait)| time >= wait)
            .for_each(|(next, wait)| {
                let time = time - wait;
                let estimated = estimated + valves[next].1 * time;
                let opened = opened | 1 << next;
                let mut status = status.clone();
                status[m] = (next, time, estimated, opened);
                let h = released
                    + estimated
                    + status.iter().map(|(_, v, _, _)| v).sum::<i32>();
                queue.push((h, status, (m + 1) % n));
            });
    }

    max_released
}

pub fn part_one(input: &str) -> i32 {
    search(input, 1, 30)
}

pub fn part_two(input: &str) -> i32 {
    search(input, 2, 26)
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
