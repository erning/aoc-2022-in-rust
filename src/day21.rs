use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    HashMap::from_iter(input.lines().map(|v| {
        let v: Vec<&str> = v.split([' ', ':']).collect();
        (v[0], v[2..].to_vec())
    }))
}

pub fn part_one(input: &str) -> i64 {
    fn f(name: &str, monkeys: &HashMap<&str, Vec<&str>>) -> i64 {
        match monkeys.get(name) {
            Some(job) if job.len() == 1 => job[0].parse().unwrap(),
            Some(job) if job.len() == 3 => {
                let a = f(job[0], monkeys);
                let b = f(job[2], monkeys);
                match job[1] {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    let monkeys = parse_input(input);
    f("root", &monkeys)
}

pub fn part_two(input: &str) -> i64 {
    // phase 1
    // calculate correct part of the tree
    fn p1<'a>(
        name: &'a str,
        monkeys: &HashMap<&'a str, Vec<&'a str>>,
        calculated: &mut HashMap<&'a str, i64>,
    ) -> Option<i64> {
        if name == "humn" {
            return None;
        }
        match monkeys.get(name) {
            Some(job) if job.len() == 1 => {
                let v = job[0].parse().unwrap();
                calculated.insert(name, v);
                Some(v)
            }
            Some(job) if job.len() == 3 => {
                match (
                    p1(job[0], monkeys, calculated),
                    p1(job[2], monkeys, calculated),
                ) {
                    (Some(a), Some(b)) => {
                        let v = match job[1] {
                            "+" => a + b,
                            "-" => a - b,
                            "*" => a * b,
                            "/" => a / b,
                            _ => panic!(),
                        };
                        calculated.insert(name, v);
                        Some(v)
                    }
                    _ => None,
                }
            }
            _ => panic!(),
        }
    }

    // phase 2
    // calculate the humn value
    fn p2(
        name: &str,
        value: i64,
        monkeys: &HashMap<&str, Vec<&str>>,
        calculated: &HashMap<&str, i64>,
    ) -> i64 {
        if name == "humn" {
            return value;
        }
        let job = monkeys.get(name).unwrap();
        let (a, b) = (job[0], job[2]);
        match (calculated.get(a), calculated.get(b)) {
            (Some(v), None) => match job[1] {
                "+" => p2(b, value - v, monkeys, calculated),
                "-" => p2(b, v - value, monkeys, calculated),
                "*" => p2(b, value / v, monkeys, calculated),
                "/" => p2(b, v / value, monkeys, calculated),
                _ => panic!(),
            },
            (None, Some(v)) => match job[1] {
                "+" => p2(a, value - v, monkeys, calculated),
                "-" => p2(a, value + v, monkeys, calculated),
                "*" => p2(a, value / v, monkeys, calculated),
                "/" => p2(a, value * v, monkeys, calculated),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    let mut monkeys = parse_input(input);
    let mut calculated: HashMap<&str, i64> = HashMap::new();
    p1("root", &monkeys, &mut calculated);

    let job = monkeys.get_mut("root").unwrap();
    // if lhs matchs rhs then lhs - rhs == 0
    job[1] = "-";
    p2("root", 0, &monkeys, &calculated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 0);
        assert_eq!(part_two(&input), 0);
    }
}
