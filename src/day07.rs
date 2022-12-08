use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(Vec<&str>, &str, usize)> {
    let mut currdir: Vec<&str> = vec![];
    let mut fs: HashMap<(Vec<&str>, &str), usize> = HashMap::new();
    fs.insert((vec![], ""), 0);
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
                fs.insert((currdir.clone(), ""), 0);
            }
            ["$", "ls"] => {}
            ["dir", dir] => {
                let mut newdir = currdir.clone();
                newdir.push(dir);
                fs.insert((newdir, ""), 0);
            }
            [size, file] => {
                let size = size.parse().unwrap();
                fs.insert((currdir.clone(), file), size);
                for i in 0..=currdir.len() {
                    if let Some(v) = fs.get_mut(&(currdir[..i].to_vec(), ""))
                    {
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
        .filter(|(_, file, size)| file.is_empty() && size <= &100000)
        .map(|(_, _, size)| *size)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let fs = parse_input(input);
    let unused: usize = 70000000
        - fs.iter()
            .find(|(dir, file, _)| dir.is_empty() && file.is_empty())
            .map(|(_, _, size)| size)
            .unwrap();
    fs.iter()
        .filter(|(_, file, size)| {
            file.is_empty() && unused + size >= 30000000
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
