use std::collections::HashMap;

// returns map of (name, job)
fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    HashMap::from_iter(input.lines().map(|v| {
        let v: Vec<&str> = v.split([' ', ':']).collect();
        (v[0], v[2..].to_vec())
    }))
}

pub fn part_one(input: &str) -> i64 {
    fn f(name: &str, jobs: &HashMap<&str, Vec<&str>>) -> i64 {
        match jobs.get(name) {
            Some(job) if job.len() == 1 => job[0].parse().unwrap(),
            Some(job) if job.len() == 3 => {
                let a = f(job[0], jobs);
                let b = f(job[2], jobs);
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

    let jobs = parse_input(input);
    f("root", &jobs)
}

pub fn part_two(input: &str) -> i64 {
    // phase 1
    // calculate correct part of the tree
    fn p1<'a>(
        name: &'a str,
        jobs: &HashMap<&'a str, Vec<&'a str>>,
        calculated: &mut HashMap<&'a str, i64>,
    ) -> Option<i64> {
        if name == "humn" {
            return None;
        }
        match jobs.get(name) {
            Some(job) if job.len() == 1 => {
                let v = job[0].parse().unwrap();
                calculated.insert(name, v);
                Some(v)
            }
            Some(job) if job.len() == 3 => {
                let v1 = p1(job[0], jobs, calculated);
                let v2 = p1(job[2], jobs, calculated);
                match (v1, v2) {
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
        jobs: &HashMap<&str, Vec<&str>>,
        calculated: &HashMap<&str, i64>,
    ) -> i64 {
        if name == "humn" {
            return value;
        }
        let job = jobs.get(name).unwrap();
        let (a, b) = (job[0], job[2]);
        match (calculated.get(a), calculated.get(b)) {
            (Some(v), None) => match job[1] {
                "+" => p2(b, value - v, jobs, calculated),
                "-" => p2(b, v - value, jobs, calculated),
                "*" => p2(b, value / v, jobs, calculated),
                "/" => p2(b, v / value, jobs, calculated),
                _ => panic!(),
            },
            (None, Some(v)) => match job[1] {
                "+" => p2(a, value - v, jobs, calculated),
                "-" => p2(a, value + v, jobs, calculated),
                "*" => p2(a, value / v, jobs, calculated),
                "/" => p2(a, value * v, jobs, calculated),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    let mut jobs = parse_input(input);
    let mut calculated: HashMap<&str, i64> = HashMap::new();
    p1("root", &jobs, &mut calculated);

    let job = jobs.get_mut("root").unwrap();
    assert_eq!(job.len(), 3);
    // if lhs matchs rhs then lhs - rhs == 0
    job[1] = "-";
    p2("root", 0, &jobs, &calculated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 152);
        assert_eq!(part_two(&input), 301);
    }
}
