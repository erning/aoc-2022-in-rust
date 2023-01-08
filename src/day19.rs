use std::collections::BinaryHeap;
use std::collections::HashSet;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

type Blueprint = [Vec<(usize, i32)>; 4];
type Status = (usize, [i32; 4], [i32; 4]); // time, bots, resources

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|v| {
            v.split_ascii_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            [
                vec![(ORE, v[0])],                   // for ore
                vec![(ORE, v[1])],                   // for clay
                vec![(ORE, v[2]), (CLAY, v[3])],     // for obsidian
                vec![(ORE, v[4]), (OBSIDIAN, v[5])], // for geode
            ]
        })
        .collect()
}

fn search(bp: &Blueprint, minute: usize) -> i32 {
    let maxbots: Vec<i32> = (0..3)
        .map(|rtype| {
            bp.iter()
                .filter_map(|recipe| {
                    recipe.iter().find(|(i, _)| *i == rtype).map(|(_, v)| *v)
                })
                .max()
                .unwrap()
        })
        .collect();

    let mut geodes: Vec<i32> = vec![0; minute + 1];
    let mut visited: HashSet<Status> = HashSet::new();
    let mut queue: BinaryHeap<(i32, Status)> = BinaryHeap::new();
    queue.push((0, (minute, [1, 0, 0, 0], [0, 0, 0, 0])));

    while let Some((_, status)) = queue.pop() {
        let (time, bots, resources) = status;
        if resources[GEODE] > geodes[time] {
            geodes[time] = resources[GEODE];
        }
        if bots[GEODE] /* * time as i32 */ + resources[GEODE] < geodes[time] {
            continue;
        }
        if time == 0 {
            continue;
        }
        if !visited.insert(status) {
            continue;
        }

        for (btype, recipe) in bp.iter().enumerate() {
            if btype != GEODE && bots[btype] >= maxbots[btype] {
                continue;
            }
            if recipe.iter().any(|&(i, _)| bots[i] == 0) {
                continue;
            }
            let wait = recipe
                .iter()
                .map(|&(rtype, amount)| {
                    (amount - resources[rtype], bots[rtype])
                })
                .map(|(a, b)| {
                    (a + b - 1) / b // ceil(a/b)
                })
                .max()
                .unwrap()
                .max(0) as usize
                + 1; // add one minute for building the bot

            if wait > time {
                continue;
            }
            // collected materials
            let mut resources = resources;
            resources.iter_mut().enumerate().for_each(|(i, v)| {
                *v += bots[i] * wait as i32;
            });
            // built bots
            let mut bots = bots;
            bots[btype] += 1;
            // cost materials
            recipe.iter().for_each(|&(i, v)| {
                resources[i] -= v;
            });
            let time = time - wait;
            let status = (time, bots, resources);
            let h = bots[GEODE] + resources[GEODE];
            queue.push((h, status));
        }
    }

    geodes[0]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 33);
        assert_eq!(part_two(&input), 56 * 62);
    }
}
