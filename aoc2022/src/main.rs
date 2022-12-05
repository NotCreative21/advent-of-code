use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_input<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    Ok(lines?.join("\n"))
}

macro_rules! problem {
    ($num:expr, $day:ident) => {
        let input = read_input(format!("inputs/day{}", $num))?;
        let ans = $day::solve(&input);
        println!(
            "day {}, part one: \t\t\t{}, \tpart two: \t\t{}",
            $num, ans.0, ans.1
        );
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    problem!(1, day1);
    problem!(2, day2);
    problem!(3, day3);
    problem!(4, day4);
    problem!(5, day5);
    Ok(())
}

mod day1 {
    pub fn solve(input: &str) -> (isize, isize) {
        let mut values: Vec<isize> = input
            .split("\n\n")
            .into_iter()
            .map(|x| x.split('\n').filter_map(|x| x.parse::<isize>().ok()).sum())
            .collect();
        values.sort();
        (part_one(&values), part_two(&values))
    }

    #[inline(always)]
    fn part_one(values: &[isize]) -> isize {
        *values.last().expect("should be longer than 1 item")
    }

    #[inline(always)]
    fn part_two(values: &[isize]) -> isize {
        values[values.len() - 3..].iter().sum()
    }
}

mod day2 {
    #[inline(always)]
    fn score(c: char) -> isize {
        match c {
            'A' | 'X' => 1, // Rock
            'B' | 'Y' => 2, // Paper
            'C' | 'Z' => 3, // Scissors
            _ => unreachable!(),
        }
    }

    pub fn solve(input: &str) -> (isize, isize) {
        let (mut one, mut two) = (0, 0);
        for line in input.lines() {
            let round: Vec<char> = line.chars().collect();
            let play = score(round[2]);
            let op = score(round[0]);
            one += if play == op {
                3 + score(round[0])
            } else {
                match play {
                    1 => {
                        if op == 2 {
                            play
                        } else {
                            play + 6
                        }
                    }
                    2 => {
                        if op == 1 {
                            play + 6
                        } else {
                            play
                        }
                    }
                    3 => {
                        if op == 2 {
                            play + 6
                        } else {
                            play
                        }
                    }
                    _ => unreachable!(),
                }
            };
            two += if play == 2 {
                op + 3
            } else {
                match play {
                    1 => {
                        if op == 3 {
                            2
                        } else if op == 2 {
                            1
                        } else {
                            3
                        }
                    }
                    3 => {
                        if op == 3 {
                            7
                        } else if op == 2 {
                            9
                        } else {
                            8
                        }
                    }
                    _ => unreachable!(),
                }
            };
        }
        (one, two)
    }
}

mod day3 {

    macro_rules! char_u8 {
        ($val:expr) => {
            $val.chars()
                .map(|x| {
                    let x = x as u8;
                    (if x >= 97 { x - 96 } else { x - 38 }) as isize
                })
                .collect()
        };
    }

    // messy but works
    fn part_one(input: &str) -> isize {
        input
            .lines()
            .map(|x| {
                let first = x.len() / 2;
                let c1: Vec<isize> = char_u8!(x[..first]);
                let c2: Vec<isize> = char_u8!(x[first..]);
                for v in c1 {
                    if c2.contains(&v) {
                        return v;
                    }
                }
                0
            })
            .sum()
    }

    fn part_two(input: &str) -> isize {
        let input: Vec<&str> = input.lines().collect();
        input
            .chunks(3)
            .map(|x| {
                let c1: Vec<isize> = char_u8!(x[0]);
                let c2: Vec<isize> = char_u8!(x[1]);
                let c3: Vec<isize> = char_u8!(x[2]);
                for v in c1 {
                    if c2.contains(&v) && c3.contains(&v) {
                        return v;
                    }
                }
                0
            })
            .sum()
    }
    pub fn solve(input: &str) -> (isize, isize) {
        (part_one(input), part_two(input))
    }
}

mod day4 {
    macro_rules! to_range {
        ($val:expr) => {
            {
                let ends: Vec<&str> = $val.split('-').collect();
                let (Some(start), Some(end)) = (ends.first(), ends.last()) else {
                    unreachable!();
                };
                let (Ok(start), Ok(end)): (Result<isize, _>, Result<isize, _>) = (start.parse(),end.parse()) else {
                    unreachable!();
                };
                (start..=end)
            }
        };
    }
    fn part_one(input: &str) -> isize {
        input
            .trim()
            .lines()
            .filter(|x| {
                let x: Vec<&str> = x.split(',').collect();
                let (Some(l), Some(r)) = (x.first(), x.last()) else {
                unreachable!();
            };
                let l = to_range!(l);
                let mut r = to_range!(r);
                l.clone().all(|x| r.clone().contains(&x)) || r.all(|x| l.contains(&x))
            })
            .count() as isize
    }
    fn part_two(input: &str) -> isize {
        input
            .trim()
            .lines()
            .filter(|x| {
                let x: Vec<&str> = x.split(',').collect();
                let (Some(l), Some(r)) = (x.first(), x.last()) else {
                unreachable!();
            };
                let mut l = to_range!(l);
                let mut r = to_range!(r);
                // WHY DOES THIS NOT WORK!!!
                l.any(|x| r.contains(&x)) || r.any(|x| l.contains(&x))
            })
            .count() as isize
    }
    pub fn solve(input: &str) -> (isize, isize) {
        (part_one(input), part_two(input))
    }
}

mod day5 {

    fn part_one(input: &str) -> String {
        let mut crate_input = 0;
        let mut crates = vec![vec![]; 9];
        let mut data = Vec::new();
        for (l, line) in input.lines().enumerate() {
            if line.contains('[') {
                data.push(line);
            } else {
                for e in data.iter_mut() {
                    for (pos, c) in e.chars().enumerate() {
                        if c == ' ' || pos == 0 || (pos - 1) % 4 != 0 {
                            continue;
                        }
                        let pos = pos - 1;
                        let stack = if pos >= 4 { pos / 4 } else { pos };
                        crates[stack].push(c);
                    }
                }
                crate_input = l;
                break;
            }
        }
        for (l, line) in input.lines().enumerate() {
            if line.trim().is_empty() || l <= crate_input {
                continue;
            }
            let line = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");
            let moves: Result<Vec<usize>, _> = line.split_whitespace().map(|x| x.parse()).collect();
            let moves = moves.expect("fail to parse");
            unsafe {
                let mut moved = crates[moves[1] - 1]
                    .drain(..moves[0])
                    .collect::<Vec<char>>();
                let original = &mut (*crates.as_mut_ptr().add(moves[2] - 1));
                original.reverse();
                original.append(&mut moved);
                original.reverse();
            }
        }
        crates.iter().map(|x| x.first().unwrap_or(&' ')).collect()
    }

    fn part_two(input: &str) -> String {
        let mut crate_input = 0;
        let mut crates = vec![vec![]; 9];
        let mut data = Vec::new();
        for (l, line) in input.lines().enumerate() {
            if line.contains('[') {
                data.push(line);
            } else {
                for e in data.iter_mut() {
                    for (pos, c) in e.chars().enumerate() {
                        if c == ' ' || pos == 0 || (pos - 1) % 4 != 0 {
                            continue;
                        }
                        let pos = pos - 1;
                        let stack = if pos >= 4 { pos / 4 } else { pos };
                        crates[stack].push(c);
                    }
                }
                crate_input = l;
                break;
            }
        }
        for (l, line) in input.lines().enumerate() {
            if line.trim().is_empty() || l <= crate_input {
                continue;
            }
            let line = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");
            let moves: Result<Vec<usize>, _> = line.split_whitespace().map(|x| x.parse()).collect();
            let moves = moves.expect("fail to parse");
            unsafe {
                let mut moved = crates[moves[1] - 1]
                    .drain(..moves[0])
                    .collect::<Vec<char>>();
                moved.reverse();
                let original = &mut (*crates.as_mut_ptr().add(moves[2] - 1));
                original.reverse();
                original.append(&mut moved);
                original.reverse();
            }
        }
        crates.iter().map(|x| x.first().unwrap_or(&' ')).collect()
    }

    pub fn solve(input: &str) -> (String, String) {
        (part_one(input), part_two(input))
    }
}
