use std::collections::BinaryHeap;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

const BOT: usize = 0;
const RES: usize = 1;

type Blueprint = [i32; 6];
type Status = [[i32; 4]; 2];

fn parse_input(input: &str) -> Vec<Blueprint> {
    // 0 - number of ore cost by ore robot
    // 1 - number of ore cost by clay robot
    // 2 - number of ore cost by obsidian robot
    // 3 - number of clay cost by obsidian robot
    // 4 - number of ore cost by geode robot
    // 5 - number of obsidian cost by geode robot
    input
        .lines()
        .map(|v| {
            v.split_ascii_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|v| v.try_into().unwrap())
        .collect()
}

fn heuristic(s: Status) -> i32 {
    s[BOT][GEODE] * 1000
        + s[BOT][OBSIDIAN] * 100
        + s[BOT][CLAY] * 10
        + s[BOT][ORE]
}

fn search(bp: &Blueprint, minute: usize) -> i32 {
    let effect_robots =
        [bp[0].max(bp[1]).max(bp[2]).max(bp[4]), bp[3], bp[5]];
    // println!("Blueprint: {:?}", bp);
    // println!("effect_robots: {:?}", effect_robots);

    let initial: Status = [[1, 0, 0, 0], [0, 0, 0, 0]];
    let mut queue: BinaryHeap<(i32, usize, Status)> = BinaryHeap::new();
    queue.push((0, minute, initial));

    let mut max: Vec<i32> = vec![0; minute + 1];

    while let Some((_, time, s)) = queue.pop() {
        if s[RES][GEODE] > max[time] {
            max[time] = s[RES][GEODE];
            // println!("max={:?}, minute={} {:?}", max[time], minute - time, s);
        }

        if max[time] > 0 && s[RES][GEODE] + s[BOT][GEODE] < max[time] {
            continue;
        }

        if time == 0 {
            continue;
        }

        let mut ss: Vec<Status> = Vec::with_capacity(5);
        // building ore robot
        if s[BOT][ORE] < effect_robots[ORE] && s[RES][ORE] >= bp[0] {
            let mut ns = s;
            ns[RES][ORE] -= bp[0];
            ns[BOT][ORE] += 1;
            (0..4).into_iter().for_each(|i| ns[RES][i] += s[BOT][i]);
            ss.push(ns);
        }
        // building clay robot
        if s[BOT][CLAY] < effect_robots[CLAY] && s[RES][ORE] >= bp[1] {
            let mut ns = s;
            ns[RES][ORE] -= bp[1];
            ns[BOT][CLAY] += 1;
            (0..4).into_iter().for_each(|i| ns[RES][i] += s[BOT][i]);
            ss.push(ns);
        }
        // building obsidian robot
        if s[BOT][OBSIDIAN] < effect_robots[OBSIDIAN]
            && s[RES][ORE] >= bp[2]
            && s[RES][CLAY] >= bp[3]
        {
            let mut ns = s;
            ns[RES][ORE] -= bp[2];
            ns[RES][CLAY] -= bp[3];
            ns[BOT][OBSIDIAN] += 1;
            (0..4).into_iter().for_each(|i| ns[RES][i] += s[BOT][i]);
            ss.push(ns);
        }
        // building geode robot
        if s[RES][ORE] >= bp[4] && s[RES][OBSIDIAN] >= bp[5] {
            let mut ns = s;
            ns[RES][ORE] -= bp[4];
            ns[RES][OBSIDIAN] -= bp[5];
            ns[BOT][GEODE] += 1;
            (0..4).into_iter().for_each(|i| ns[RES][i] += s[BOT][i]);
            ss.push(ns);
        }
        // nothing to build
        if (0..3)
            .into_iter()
            .any(|i| s[BOT][i] < effect_robots[i])
        {
            let mut ns = s;
            (0..4).into_iter().for_each(|i| ns[RES][i] += s[BOT][i]);
            ss.push(ns);
        }
        // enqueue
        ss.into_iter().for_each(|v| {
            queue.push((heuristic(v), time - 1, v));
        })
    }

    max[0]
}

pub fn part_one(input: &str) -> i32 {
    let bps = parse_input(input);
    bps.iter()
        .map(|v| search(v, 24))
        .enumerate()
        .map(|(i, v)| (i + 1) as i32 * v)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let bps = parse_input(input);
    bps.iter().take(3).map(|v| search(v, 32)).product()
    // let a = search(&bps[0], 32);
    // let b = search(&bps[1], 32);
    // let c = search(&bps[2], 32);
    // a * b * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 33);
        assert_eq!(part_one(&input), 56 * 62);
        // let bps = parse_input(&input);
        // assert_eq!(search(&bps[0], 32), 56);
        // assert_eq!(search(&bps[1], 32), 62);
    }
}
