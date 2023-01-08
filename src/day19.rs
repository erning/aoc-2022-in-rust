use std::collections::BinaryHeap;
use std::collections::HashSet;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

type Blueprint = [Vec<(usize, i32)>; 4];
type Status = (i32, [i32; 4], [i32; 4]);

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|v| {
            v.split_ascii_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            // 0 - number of ore cost by ore robot
            // 1 - number of ore cost by clay robot
            // 2 - number of ore cost by obsidian robot
            // 3 - number of clay cost by obsidian robot
            // 4 - number of ore cost by geode robot
            // 5 - number of obsidian cost by geode robot
            [
                vec![(ORE, v[0])],                   // for ore
                vec![(ORE, v[1])],                   // for clay
                vec![(ORE, v[2]), (CLAY, v[3])],     // for obsidian
                vec![(ORE, v[4]), (OBSIDIAN, v[5])], // for geode
            ]
        })
        .collect()
}

fn heuristic(s: Status) -> i32 {
    s.1[GEODE] + s.2[GEODE]
    // s.1[GEODE] * 1000 + s.1[OBSIDIAN] * 100 + s.1[CLAY] * 10 + s.1[ORE]
}

fn search(bp: &Blueprint, minute: usize) -> i32 {
    let maxspend = |rtype| {
        bp.iter()
            .filter_map(|recipe| {
                recipe.iter().find(|(i, _)| *i == rtype).map(|(_, v)| *v)
            })
            .max()
            .unwrap()
    };
    let maxspend = [maxspend(ORE), maxspend(CLAY), maxspend(OBSIDIAN)];
    println!("maxspend: {:?}", maxspend);

    let mut visited: HashSet<Status> = HashSet::new();

    let initial: Status = (minute as i32, [1, 0, 0, 0], [0, 0, 0, 0]);
    let mut queue: BinaryHeap<(i32, Status)> = BinaryHeap::new();
    queue.push((0, initial));

    let mut max: Vec<i32> = vec![0; minute + 1];

    let mut c = 0;
    while let Some((_, status)) = queue.pop() {
        c += 1;
        let (time, bots, materials) = status;
        if materials[GEODE] > max[time as usize] {
            max[time as usize] = materials[GEODE];
            println!(
                "max={:?}, minute={:?}",
                max[time as usize],
                minute as i32 - time
            );
        }
        // println!("{}: {:?}, {:?}", minute as i32 - time, bots, materials);
        if max[time as usize] > 0
            && materials[GEODE] + bots[GEODE] < max[time as usize]
        {
            continue;
        }

        if time <= 0 {
            // TODO: the max
            continue;
        }

        if !visited.insert(status) {
            continue;
        }

        for (btype, recipe) in bp.iter().enumerate() {
            if btype != 3 && bots[btype] >= maxspend[btype] {
                continue;
            }
            if recipe.iter().any(|&(i, _)| bots[i] == 0) {
                continue;
            }
            let wait = recipe
                .iter()
                .map(|&(rtype, amount)| {
                    (amount - materials[rtype], bots[rtype])
                })
                .map(|(a, b)| {
                    (a + b - 1) / b // ceil(a/b)
                })
                .max()
                .unwrap()
                .max(0);
            // println!("wait={:?}", (btype, wait));

            let ntime = time - wait - 1;
            if ntime < 0 {
                // println!("ntime={}", ntime);
                continue;
            }

            let mut nbots = bots;
            nbots[btype] += 1;
            let mut nmaterials = materials;
            nmaterials.iter_mut().enumerate().for_each(|(i, v)| {
                *v += bots[i] * (wait + 1);
            });
            recipe.iter().for_each(|&(i, v)| {
                nmaterials[i] -= v;
            });

            let nstatus = (ntime, nbots, nmaterials);
            // println!("new: {:?}", nstatus);

            queue.push((heuristic(nstatus), nstatus));
        }
    }
    println!("C={}", c);

    max[0]
}

use std::sync::mpsc;
use std::thread;

pub fn part_one(input: &str) -> i32 {
    // let bps = parse_input(input);
    // let (tx, rx) = mpsc::channel();

    // for (i, bp) in bps.iter().enumerate() {
    //     let bp = bp.clone();
    //     let tx = tx.clone();
    //     thread::spawn(move || {
    //         let max = search(&bp, 24);
    //         tx.send((i, max)).unwrap();
    //     });
    // }

    // let mut wait = bps.len();
    // let mut sum = 0;
    // for (i, v) in rx {
    //     sum += (i + 1) as i32 * v;
    //     wait -= 1;
    //     if wait == 0 {
    //         break;
    //     }
    // }
    // sum

    let bps = parse_input(input);
    // for (i, bp) in bps.iter().enumerate() {
    //     let max = search(&bp, 24);
    // }
    bps.iter()
        .map(|v| search(v, 24))
        .enumerate()
        .map(|(i, v)| (i + 1) as i32 * v)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    // let bps = parse_input(input);
    // let (tx, rx) = mpsc::channel();

    // for bp in bps.iter().take(3) {
    //     let bp = bp.clone();
    //     let tx = tx.clone();
    //     thread::spawn(move || {
    //         let max = search(&bp, 32);
    //         tx.send(max).unwrap();
    //     });
    // }

    // let mut wait = 3.min(bps.len());
    // let mut ans = 1;
    // for v in rx {
    //     ans *= v;
    //     wait -= 1;
    //     if wait == 0 {
    //         break;
    //     }
    // }
    // ans
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
        // let bps = parse_input(&input);
        // assert_eq!(search(&bps[0], 32), 56);
        // assert_eq!(search(&bps[1], 32), 62);
    }
}
