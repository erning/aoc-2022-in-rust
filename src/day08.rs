fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|v| v.chars().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut count = h + h + w + w - 4;
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            let v = grid[i][j];
            if (0..i).all(|p| grid[p][j] < v)
                || ((i + 1)..h).all(|p| grid[p][j] < v)
                || (0..j).all(|p| grid[i][p] < v)
                || ((j + 1)..h).all(|p| grid[i][p] < v)
            {
                count += 1;
                continue;
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut highest_scenic_score = 0;
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            let v = grid[i][j];
            let a = match (0..i).rev().find(|&p| grid[p][j] >= v) {
                None => i,
                Some(p) => i - p,
            }; // up
            let b = match ((i + 1)..h).find(|&p| grid[p][j] >= v) {
                None => h - i - 1,
                Some(p) => p - i,
            }; // down
            let c = match (0..j).rev().find(|&p| grid[i][p] >= v) {
                None => j,
                Some(p) => j - p,
            }; // left
            let d = match ((j + 1)..w).find(|&p| grid[i][p] >= v) {
                None => w - j - 1,
                Some(p) => p - j,
            }; // right
            let score = a * b * c * d;
            highest_scenic_score = score.max(highest_scenic_score);
        }
    }
    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 21);
        assert_eq!(part_two(&input), 8);
    }
}
