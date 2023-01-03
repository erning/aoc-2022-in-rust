fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|g| g.trim().lines().map(|s| s.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    parse_input(input)
        .into_iter()
        .map(|g| g.into_iter().sum())
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let mut calories: Vec<i32> = parse_input(input)
        .into_iter()
        .map(|g| g.into_iter().sum())
        .collect();
    calories.sort_unstable();
    calories.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 24000);
        assert_eq!(part_two(&input), 45000);
    }
}
