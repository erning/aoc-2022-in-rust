fn parse_instructions(input: &str) -> Vec<i32> {
    // -1: turn right
    // -3: turn left
    // positive: moving steps
    let mut instructions: Vec<i32> = Vec::new();
    let mut value = 0;
    for c in input.lines().last().unwrap().chars() {
        match c {
            'R' => {
                instructions.push(value);
                instructions.push(1 - 2);
                value = 0;
            }
            'L' => {
                instructions.push(value);
                instructions.push(-1 - 2);
                value = 0;
            }
            _ => {
                value *= 10;
                value += c.to_digit(10).unwrap() as i32;
            }
        }
    }
    instructions.push(value);
    instructions
}

const R: i32 = 0;
const D: i32 = 1;
const L: i32 = 2;
const U: i32 = 3;

struct Board {
    board: Vec<Vec<u8>>,
    rows: Vec<(i32, i32)>,
    cols: Vec<(i32, i32)>,
}

impl Board {
    fn new(input: &str) -> Board {
        let width = input
            .lines()
            .take_while(|v| !v.is_empty())
            .map(|v| v.len())
            .max()
            .unwrap();
        let height = input.lines().count() - 2;

        let board: Vec<Vec<u8>> = input
            .lines()
            .take_while(|v| !v.is_empty())
            .map(|v| {
                v.bytes()
                    .map(|v| match v {
                        b'.' => 1,
                        b'#' => 2,
                        _ => 0,
                    })
                    .collect::<Vec<u8>>()
            })
            .map(|v| {
                let mut v = v;
                v.extend(std::iter::repeat(0).take(width - v.len()));
                v
            })
            .collect();

        let rows: Vec<(i32, i32)> = (0..height)
            .into_iter()
            .map(|y| {
                let a = (0..width)
                    .into_iter()
                    .find(|&x| board[y][x] > 0)
                    .unwrap();
                let b = (0..width)
                    .into_iter()
                    .rev()
                    .find(|&x| board[y][x] > 0)
                    .unwrap();
                (a as i32, b as i32)
            })
            .collect();

        let cols: Vec<(i32, i32)> = (0..width)
            .into_iter()
            .map(|x| {
                let a = (0..height)
                    .into_iter()
                    .find(|&y| board[y][x] > 0)
                    .unwrap();
                let b = (0..height)
                    .into_iter()
                    .rev()
                    .find(|&y| board[y][x] > 0)
                    .unwrap();
                (a as i32, b as i32)
            })
            .collect();

        Board { board, rows, cols }
    }

    fn next(&self, x: &mut i32, y: &mut i32, d: i32) -> bool {
        let (r1, r2) = self.rows[*y as usize];
        let (c1, c2) = self.cols[*x as usize];
        let (nx, ny) = match d {
            R => (if *x == r2 { r1 } else { *x + 1 }, *y),
            D => (*x, if *y == c2 { c1 } else { *y + 1 }),
            L => (if *x == r1 { r2 } else { *x - 1 }, *y),
            U => (*x, if *y == c1 { c2 } else { *y - 1 }),
            _ => panic!(),
        };
        if self.is_open_tile(nx, ny) {
            (*x, *y) = (nx, ny);
            true
        } else {
            false
        }
    }

    fn is_open_tile(&self, x: i32, y: i32) -> bool {
        self.board[y as usize][x as usize] == 1
    }

    fn edge(&self, x: i32, y: i32, d: i32) -> i32 {
        match d {
            R => self.rows[y as usize].1,
            D => self.cols[x as usize].1,
            L => self.rows[y as usize].0,
            U => self.rows[x as usize].0,
            _ => panic!(),
        }
    }
}

const CU: i32 = 0;
const CL: i32 = 1;
const CF: i32 = 2;
const CR: i32 = 3;
const CD: i32 = 4;
const CB: i32 = 5;

struct Cube {
    surfaces: Vec<Vec<Vec<u8>>>,
    n: i32,
    config: Vec<(i32, i32, i32)>,
}

impl Cube {
    fn new(input: &str) -> Cube {
        fn parse_surface(
            x: i32,
            y: i32,
            n: i32,
            t: i32,
            board: &[Vec<u8>],
        ) -> Vec<Vec<u8>> {
            let mut surface: Vec<Vec<u8>> = Vec::new();
            match t {
                1 => {
                    for i in 0..=n {
                        let y = (y + i) as usize;
                        let mut row: Vec<u8> = Vec::new();
                        for j in 0..=n {
                            let x = (x + j) as usize;
                            row.push(board[y][x]);
                        }
                        surface.push(row);
                    }
                }
                2 => {
                    for i in 0..=n {
                        let x = (x + i) as usize;
                        let mut row: Vec<u8> = Vec::new();
                        for j in 0..=n {
                            let y = (y + n - j) as usize;
                            row.push(board[y][x]);
                        }
                        surface.push(row);
                    }
                }
                3 => {
                    for i in 0..=n {
                        let x = (x + n - i) as usize;
                        let mut row: Vec<u8> = Vec::new();
                        for j in 0..=n {
                            let y = (y + j) as usize;
                            row.push(board[y][x]);
                        }
                        surface.push(row);
                    }
                }
                4 => {
                    for i in 0..=n {
                        let y = (y + n - i) as usize;
                        let mut row: Vec<u8> = Vec::new();
                        for j in 0..=n {
                            let x = (x + n - j) as usize;
                            row.push(board[y][x]);
                        }
                        surface.push(row);
                    }
                }
                _ => panic!(),
            };

            surface
        }

        let board = Board::new(input);
        // (x, y): surface location
        //   0 1 2 3
        // 0 . . . .
        // 1 . . . .
        // 2 . . . .
        // 3 . . . .
        //
        // t: rotation type to the standard form
        //   - 1 origin
        //   - 2: clockwise
        //   - 3: anti-clockwise
        //   - 4: rotate 180 degree
        //
        // standard form:
        //   UU
        // LLFFRR
        //   DD
        //   BB
        let (n, config) = if board.board[0].len() == 16 {
            // example data
            let mut config = vec![(0, 0, 0); 6];
            config[CU as usize] = (2, 0, 1);
            config[CB as usize] = (0, 1, 4);
            config[CL as usize] = (1, 1, 1);
            config[CF as usize] = (2, 1, 1);
            config[CD as usize] = (2, 2, 1);
            config[CR as usize] = (3, 2, 3);
            (4, config)
        } else {
            // hardcode for real input data
            let mut config = vec![(0, 0, 0); 6];
            config[CU as usize] = (1, 0, 1);
            config[CR as usize] = (2, 0, 2);
            config[CF as usize] = (1, 1, 1);
            config[CL as usize] = (0, 2, 2);
            config[CD as usize] = (1, 2, 1);
            config[CB as usize] = (0, 3, 3);
            (50, config)
        };

        let mut surfaces = vec![Vec::new(); 6];
        for (i, &(x, y, t)) in config.iter().enumerate() {
            surfaces[i] = parse_surface(x * n, y * n, n - 1, t, &board.board);
        }

        Cube {
            surfaces,
            n,
            config,
        }
    }

    fn next(
        &self,
        s: &mut i32, // which surface,
        x: &mut i32, // (x, y) relative position within the surface
        y: &mut i32,
        d: &mut i32, // moving direction
    ) -> bool {
        let n = self.surfaces[0][0].len() as i32 - 1;

        let (nx, ny) = match *d {
            R => (*x + 1, *y),
            D => (*x, *y + 1),
            L => (*x - 1, *y),
            U => (*x, *y - 1),
            _ => panic!(),
        };

        if nx >= 0 && nx <= n && ny >= 0 && ny <= n {
            return if self.is_open_tile(*s, nx, ny) {
                *x = nx;
                *y = ny;
                true
            } else {
                false
            };
        }

        // moving cross surfaces, standard form:
        //   UU
        // LLFFRR
        //   DD
        //   BB

        let (ns, nd, nx, ny) = match (*s, *d) {
            (CU, R) => (CR, D, n - *y, 0),
            (CU, D) => (CF, D, *x, 0),
            (CU, L) => (CL, D, *y, 0),
            (CU, U) => (CB, U, *x, n),

            (CL, R) => (CF, R, 0, *y),
            (CL, D) => (CD, R, 0, n - *x),
            (CL, L) => (CB, R, 0, n - *y),
            (CL, U) => (CU, R, 0, *x),

            (CF, R) => (CR, R, 0, *y),
            (CF, D) => (CD, D, *x, 0),
            (CF, L) => (CL, L, n, *y),
            (CF, U) => (CU, U, *x, n),

            (CR, R) => (CB, L, n, n - *y),
            (CR, D) => (CD, L, n, *x),
            (CR, L) => (CF, L, n, *y),
            (CR, U) => (CU, L, n, n - *x),

            (CD, R) => (CR, U, *y, n),
            (CD, D) => (CB, D, *x, 0),
            (CD, L) => (CL, U, n - *y, n),
            (CD, U) => (CF, U, *x, n),

            (CB, R) => (CR, L, n, n - *y),
            (CB, D) => (CU, D, *x, 0),
            (CB, L) => (CL, R, 0, n - *y),
            (CB, U) => (CD, U, *x, n),

            _ => panic!(),
        };

        if self.is_open_tile(ns, nx, ny) {
            *s = ns;
            *x = nx;
            *y = ny;
            *d = nd;
            true
        } else {
            false
        }
    }

    fn is_open_tile(&self, s: i32, x: i32, y: i32) -> bool {
        self.surfaces[s as usize][y as usize][x as usize] == 1
    }

    // to the original coordinate
    fn absolute(&self, s: i32, x: i32, y: i32, d: i32) -> (i32, i32, i32) {
        let n = self.n;
        let (i, j, t) = self.config[s as usize];
        match t {
            1 => (i * n + x, j * n + y, d),
            2 => (i * n + y, j * n + n - x - 1, (d + 3) % 4),
            3 => (i * n + n - y - 1, j * n + x, (d + 1) % 4),
            4 => (i * n + n - x - 1, j * n + n - y - 1, (d + 2) % 4),
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let board = Board::new(input);
    let (mut x, mut y) = (board.edge(0, 0, L), 0);
    let mut d = R;

    for i in instructions.into_iter() {
        if i >= 0 {
            for _ in 0..i {
                if !board.next(&mut x, &mut y, d) {
                    break;
                }
            }
        } else {
            // turn right or left and avoid negative
            d += i + 2;
            d += 4;
            d %= 4;
        }
    }

    1000 * (y + 1) + 4 * (x + 1) + d
}

pub fn part_two(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let cube = Cube::new(input);
    let (mut s, mut x, mut y) = (CU, 0, 0);
    let mut d = R;

    for i in instructions.into_iter() {
        if i >= 0 {
            for _ in 0..i {
                if !cube.next(&mut s, &mut x, &mut y, &mut d) {
                    break;
                }
            }
        } else {
            d += i + 2;
            d += 4;
            d %= 4;
        }
    }

    let (x, y, d) = cube.absolute(s, x, y, d);
    1000 * (y + 1) + 4 * (x + 1) + d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 6032);
        assert_eq!(part_two(&input), 5031);
    }
}
