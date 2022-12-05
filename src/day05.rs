fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut lines = input.lines();
    'stacks: for line in lines.by_ref() {
        let mut chars = line.trim_end().chars();
        let mut n = 0;
        while let Some(c) = chars.nth(1) {
            if n >= stacks.len() {
                stacks.push(vec![]);
            }
            match c {
                'A'..='Z' => {
                    stacks[n].push(c);
                }
                '1'..='9' => {
                    break 'stacks;
                }
                ' ' => {}
                _ => {
                    panic!();
                }
            }
            n += 1;
            chars.next();
            chars.next();
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    lines.next(); // skip  empty line

    let mut moves: Vec<[usize; 3]> = vec![];
    for line in lines.by_ref() {
        let mut iter = line.split_ascii_whitespace();
        moves.push([
            iter.nth(1).unwrap().parse().unwrap(),
            iter.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            iter.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        ]);
    }
    (stacks, moves)
}

pub fn part_one(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for [n, a, b] in moves {
        for _ in 0..n {
            let c = stacks[a].pop().unwrap();
            stacks[b].push(c);
        }
    }
    let mut ans: Vec<char> = vec![];
    for stack in stacks.iter() {
        ans.push(*stack.last().unwrap());
    }
    ans.into_iter().collect::<String>()
}

pub fn part_two(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for [n, a, b] in moves {
        let mut s: Vec<char> = vec![];
        for _ in 0..n {
            let c = stacks[a].pop().unwrap();
            s.push(c);
        }
        while let Some(c) = s.pop() {
            stacks[b].push(c);
        }
    }
    let mut ans: Vec<char> = vec![];
    for stack in stacks.iter() {
        ans.push(*stack.last().unwrap());
    }
    ans.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), "CMZ");
        assert_eq!(part_two(&input), "MCD");
    }
}
