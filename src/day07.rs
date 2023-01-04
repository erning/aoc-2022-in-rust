use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(Vec<&str>, &str, usize)> {
    let mut cwd: Vec<&str> = vec![];
    let mut fs: HashMap<(Vec<&str>, &str), usize> = HashMap::new();
    fs.insert((vec![], ""), 0);
    for line in input.lines() {
        match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                cwd.clear();
            }
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", dir] => {
                cwd.push(dir);
                fs.insert((cwd.clone(), ""), 0);
            }
            ["$", "ls"] => {}
            ["dir", subdir] => {
                let mut dir = cwd.clone();
                dir.push(subdir);
                fs.insert((dir, ""), 0);
            }
            [size, file] => {
                let size = size.parse().unwrap();
                fs.insert((cwd.clone(), file), size);
                for i in 0..=cwd.len() {
                    let dir = (cwd[..i].to_vec(), "");
                    if let Some(v) = fs.get_mut(&dir) {
                        *v += size;
                    }
                }
            }
            _ => {
                panic!("unknown")
            }
        }
    }
    fs.into_iter().map(|((a, b), c)| (a, b, c)).collect()
}

pub fn part_one(input: &str) -> usize {
    let fs = parse_input(input);
    fs.iter()
        .filter(|(_, file, size)| file.is_empty() && size <= &100_000)
        .map(|(_, _, size)| *size)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let fs = parse_input(input);
    let unused: usize = 70_000_000
        - fs.iter()
            .find(|(dir, file, _)| dir.is_empty() && file.is_empty())
            .map(|(_, _, size)| size)
            .unwrap();
    fs.iter()
        .filter(|(_, file, size)| {
            file.is_empty() && unused + size >= 30_000_000
        })
        .map(|(_, _, size)| *size)
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
        // let mut fs = parse_input(&input);
        // fs.sort_unstable();
        // for (dir, file, size) in fs {
        //     if dir.is_empty() {
        //         println!("/{} -- {}", file, size);
        //     } else {
        //         println!("/{}/{} -- {}", dir.join("/"), file, size);
        //     }
        // }
        assert_eq!(part_one(&input), 95437);
        assert_eq!(part_two(&input), 24933642);
    }
}
