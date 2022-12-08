use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(Vec<&str>, &str, usize)> {
    let mut currdir: Vec<&str> = vec![];
    let mut files = vec![];
    for line in input.lines() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                currdir.clear();
            }
            ["$", "cd", ".."] => {
                currdir.pop();
            }
            ["$", "cd", dir] => {
                currdir.push(dir);
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, file] => {
                files.push((currdir.clone(), file, size.parse().unwrap()))
            }
            _ => {
                panic!("unknown")
            }
        }
    }
    files
}

pub fn part_one(input: &str) -> usize {
    let files = parse_input(input);
    let mut dirs: HashSet<Vec<&str>> = HashSet::new();
    files.iter().map(|(v, _, _)| v).for_each(|v| {
        for i in 0..v.len() {
            dirs.insert(v[..i + 1].to_vec());
        }
    });
    dirs.iter()
        .map(|dir| {
            files
                .iter()
                .filter(|(v, _, _)| v.starts_with(dir))
                .map(|(_, _, v)| v)
                .sum::<usize>()
        })
        .filter(|&v| v <= 100000)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let files = parse_input(input);
    let unused: usize =
        70000000 - files.iter().map(|(_, _, v)| v).sum::<usize>();

    let dirs: HashSet<Vec<&str>> =
        files.iter().map(|v| v.0.clone()).collect();

    dirs.iter()
        .map(|dir| {
            files
                .iter()
                .filter(|(v, _, _)| v.starts_with(dir))
                .map(|(_, _, v)| v)
                .sum::<usize>()
        })
        .filter(|&v| unused + v >= 30000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 95437);
        assert_eq!(part_two(&input), 24933642);
    }
}
