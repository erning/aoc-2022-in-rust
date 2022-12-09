use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<((i32, i32), usize)> {
    input
        .lines()
        .map(|v| v.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|v| (v[0].chars().next().unwrap(), v[1].parse().unwrap()))
        .map(|(direction, steps)| match direction {
            'U' => ((0, -1), steps),
            'D' => ((0, 1), steps),
            'L' => ((-1, 0), steps),
            'R' => ((1, 0), steps),
            _ => panic!("unknown direction"),
        })
        .collect()
}

pub fn simulate(motions: Vec<((i32, i32), usize)>, length: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); length];
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert((0, 0));
    for ((dx, dy), steps) in motions {
        for _ in 0..steps {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let x = rope[i - 1].0 - rope[i].0;
                let y = rope[i - 1].1 - rope[i].1;
                if x.abs() > 1 || y.abs() > 1 {
                    rope[i].0 += x.signum();
                    rope[i].1 += y.signum();
                }
            }
            positions.insert(*rope.last().unwrap());
        }
    }
    positions.len()
}

pub fn part_one(input: &str) -> usize {
    let motions = parse_input(input);
    simulate(motions, 2)
}

pub fn part_two(input: &str) -> usize {
    let motions = parse_input(input);
    simulate(motions, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn larger_example() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part_two(&input), 36);
    }
}
