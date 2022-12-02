fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| (v[0], v[2]))
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let guide = parse_input(input);
    guide
        .into_iter()
        .map(|(a, b)| match (a, b) {
            (b'A', b'X') => 1 + 3,
            (b'A', b'Y') => 2 + 6,
            (b'A', b'Z') => 3 + 0,
            (b'B', b'X') => 1 + 0,
            (b'B', b'Y') => 2 + 3,
            (b'B', b'Z') => 3 + 6,
            (b'C', b'X') => 1 + 6,
            (b'C', b'Y') => 2 + 0,
            (b'C', b'Z') => 3 + 3,
            _ => 0,
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let guide = parse_input(input);
    guide
        .into_iter()
        .map(|(a, b)| match (a, b) {
            (b'A', b'X') => 3 + 0,
            (b'A', b'Y') => 1 + 3,
            (b'A', b'Z') => 2 + 6,
            (b'B', b'X') => 1 + 0,
            (b'B', b'Y') => 2 + 3,
            (b'B', b'Z') => 3 + 6,
            (b'C', b'X') => 2 + 0,
            (b'C', b'Y') => 3 + 3,
            (b'C', b'Z') => 1 + 6,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 15);
        assert_eq!(part_two(&input), 12);
    }
}
