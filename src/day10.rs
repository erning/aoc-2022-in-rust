fn parse_input(input: &str) -> Vec<i32> {
    let mut dx = vec![];
    input.lines().for_each(|v| {
        if v.starts_with("noop") {
            dx.push(0);
        } else if v.starts_with("addx") {
            dx.push(0);
            dx.push(v[5..].parse().unwrap());
        }
    });
    dx
}

pub fn part_one(input: &str) -> i32 {
    let mut strengths = 0;
    let mut cycle = 0;
    let mut x = 1;
    for v in parse_input(input) {
        cycle += 1;
        if (cycle + 20) % 40 == 0 {
            strengths += cycle * x
        }
        x += v
    }
    strengths
}

pub fn part_two(input: &str) -> String {
    let mut crt: Vec<char> = vec![];
    let mut x = 1;
    for (i, v) in parse_input(input).into_iter().enumerate() {
        if (x - 1..=x + 1).contains(&(i as i32 % 40)) {
            crt.push('#');
        } else {
            crt.push(' ');
        }
        x += v;
    }

    #[cfg(test)]
    crt.iter_mut().filter(|v| *v == &' ').for_each(|v| *v = '.');

    [
        String::new(),
        crt[..]
            .chunks(40)
            .map(|v| v.iter().collect())
            .collect::<Vec<String>>()
            .join("\n"),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 13140);
        assert_eq!(
            part_two(&input),
            [
                "",
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
            .join("\n")
        );
    }
}
