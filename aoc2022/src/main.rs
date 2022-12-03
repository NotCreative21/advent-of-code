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
        println!("day {}, part one: \t{}, \tpart two: \t{}", $num, ans.0, ans.1);
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    problem!(1, day1);
    problem!(2, day2);
    problem!(3, day3);
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
