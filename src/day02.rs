pub fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 4, // 1 + 3
            (b'A', b'Y') => 8, // 2 + 6
            (b'A', b'Z') => 3, // 3 + 0
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 7, // 1 + 6
            (b'C', b'Y') => 2, // 2 + 0
            (b'C', b'Z') => 6, // 3 + 3
            _ => panic!(),
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 3, // 3 + 0
            (b'A', b'Y') => 4, // 1 + 3
            (b'A', b'Z') => 8, // 2 + 6
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 2, // 2 + 0
            (b'C', b'Y') => 6, // 3 + 3
            (b'C', b'Z') => 7, // 1 + 6
            _ => panic!(),
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
