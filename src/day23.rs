use std::collections::HashMap;
use std::collections::HashSet;

// N, S, W, E
const MOVING_DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

const ADJACENT_POS: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // NW, N, NE
    [(-1, 1), (0, 1), (1, 1)],    // SW, S, SE
    [(-1, -1), (-1, 0), (-1, 1)], // NW, W, SW
    [(1, -1), (1, 0), (1, 1)],    // NE, E, SE
];

const AROUNDS: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|(x, y)| (x as i32, y as i32))
        .collect()
}

fn rectangle(elves: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    let x0 = elves.iter().min_by_key(|(x, _)| x).unwrap().0;
    let y0 = elves.iter().min_by_key(|(_, y)| y).unwrap().1;
    let x1 = elves.iter().max_by_key(|(x, _)| x).unwrap().0;
    let y1 = elves.iter().max_by_key(|(_, y)| y).unwrap().1;
    (x0, y0, x1, y1)
}

#[allow(dead_code)]
fn show(elves: &[(i32, i32)]) {
    let (x0, y0, x1, y1) = rectangle(elves);
    let (w, h) = (x1 - x0 + 1, y1 - y0 + 1);
    let mut display: Vec<Vec<char>> = vec![vec!['.'; w as usize]; h as usize];
    elves
        .iter()
        .map(|(x, y)| (x - x0, y - y0))
        .for_each(|(x, y)| {
            display[y as usize][x as usize] = '#';
        });
    display.into_iter().for_each(|row| {
        println!("{}", String::from_iter(row));
    })
}

fn propose_moves(elves: &[(i32, i32)], dir: i32) -> Vec<(usize, i32, i32)> {
    let occupied: HashSet<(i32, i32)> =
        elves.iter().map(|(x, y)| (*x, *y)).collect();

    let will_move = |x: i32, y: i32| -> bool {
        AROUNDS
            .iter()
            .any(|(dx, dy)| occupied.contains(&(x + dx, y + dy)))
    };

    let is_valid_direction = |x: i32, y: i32, d: i32| -> bool {
        ADJACENT_POS[d as usize]
            .iter()
            .all(|(dx, dy)| !occupied.contains(&(x + dx, y + dy)))
    };

    elves
        .iter()
        .enumerate()
        .filter_map(|(i, &(x, y))| {
            if will_move(x, y) {
                let next = (0..4).into_iter().find_map(|d| {
                    let nd = (dir + d) % 4;
                    if is_valid_direction(x, y, nd) {
                        let (dx, dy) = MOVING_DIRS[nd as usize];
                        let (nx, ny) = (x + dx, y + dy);
                        Some((i, nx, ny))
                    } else {
                        None
                    }
                });
                match next {
                    Some(_) => next,
                    None => Some((i, x, y)),
                }
            } else {
                // (i, x, y)
                None
            }
        })
        .collect()
}

fn do_moves(elves: &mut [(i32, i32)], dir: i32) -> usize {
    let nexts = propose_moves(elves, dir);
    let mut occupied: HashMap<(i32, i32), i32> = HashMap::new();

    nexts.iter().for_each(|&(_, x, y)| {
        if let Some(v) = occupied.get_mut(&(x, y)) {
            *v += 1;
        } else {
            occupied.insert((x, y), 1);
        }
    });

    nexts.iter().for_each(|&(i, x, y)| {
        match occupied.get(&(x, y)) {
            Some(v) if *v == 1 => elves[i] = (x, y),
            _ => {}
        };
    });

    nexts.len()
}

pub fn part_one(input: &str) -> i32 {
    let mut elves: Vec<(i32, i32)> = parse_input(input);
    (0..10).for_each(|i| {
        do_moves(&mut elves, i % 4);
    });

    let (x0, y0, x1, y1) = rectangle(&elves);
    (x1 - x0 + 1) * (y1 - y0 + 1) - elves.len() as i32
}

pub fn part_two(input: &str) -> i32 {
    let mut elves: Vec<(i32, i32)> = parse_input(input);
    let mut i = 0;
    while do_moves(&mut elves, i % 4) > 0 {
        i += 1;
    }
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(23);
        assert_eq!(part_one(&input), 110);
        assert_eq!(part_two(&input), 20);
    }
}
