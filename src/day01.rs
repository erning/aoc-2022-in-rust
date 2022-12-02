fn parse_input(input: &str) -> Vec<u32> {
    let mut calory = 0;
    let mut calories: Vec<u32> = vec![];
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => calory += n,
            Err(_) => {
                if calory > 0 {
                    calories.push(calory)
                }
                calory = 0
            }
        }
    }
    if calory > 0 {
        calories.push(calory);
    }
    calories
}

pub fn part_one(input: &str) -> u32 {
    let calories = parse_input(input);
    calories.into_iter().max().unwrap_or(0)
}

pub fn part_two(input: &str) -> u32 {
    let mut calories = parse_input(input);
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
