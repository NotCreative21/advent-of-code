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
    ($num:expr, $day:ident) => {{
        let input = read_input(format!("inputs/day{}", $num))?;
        let ans = $day::solve(&input);
        format!(
            "day {}, part one: \t\t\t{}, \tpart two: \t\t{}",
            $num, ans.0, ans.1
        )
    }};
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        vec![
            problem!(1, day1),
            problem!(2, day2),
            problem!(3, day3),
            problem!(4, day4),
            problem!(5, day5),
            problem!(6, day6),
            problem!(7, day7),
            problem!(8, day8),
        ]
        .join("\n")
    );
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

mod day6 {
    fn part_one(input: &str) -> isize {
        let chars: Vec<char> = input.chars().collect();
        for (i, window) in chars.windows(4).enumerate() {
            let mut previous = Vec::with_capacity(4);
            let mut unique = true;
            for v in window {
                if previous.contains(v) {
                    unique = false;
                    continue;
                }
                previous.push(*v);
            }
            if unique {
                return i as isize + 4;
            }
        }
        unreachable!();
    }

    fn part_two(input: &str) -> isize {
        let chars: Vec<char> = input.chars().collect();
        for (i, window) in chars.windows(14).enumerate() {
            let mut previous = Vec::with_capacity(4);
            let mut unique = true;
            for v in window {
                if previous.contains(v) {
                    unique = false;
                    continue;
                }
                previous.push(*v);
            }
            if unique {
                return i as isize + 14;
            }
        }
        unreachable!();
    }

    pub fn solve(input: &str) -> (isize, isize) {
        (part_one(input), part_two(input))
    }
}

mod day7 {
    #[derive(Default)]
    struct Graph {
        nodes: Vec<NodeData>,
        edges: Vec<EdgeData>,
    }

    type NodeIndex = usize;
    type EdgeIndex = usize;

    struct NodeData {
        first_outgoing_edge: Option<EdgeIndex>,
    }

    struct EdgeData {
        target: NodeIndex,
        next_outgoing_edge: Option<EdgeIndex>
    }

    struct Successors<'graph> {
        graph: &'graph Graph,
        current_edge_index: Option<EdgeIndex>,
    }

    impl<'graph> Iterator for Successors<'graph> {
        type Item = NodeIndex;

        fn next(&mut self) -> Option<NodeIndex> {
            match self.current_edge_index {
                None => None,
                Some(edge_num) => {
                    let edge = &self.graph.edges[edge_num];
                    self.current_edge_index = edge.next_outgoing_edge;
                    Some(edge.target)
                }
            }
        }
    }

    impl Graph {
        pub fn add_node(&mut self) -> NodeIndex {
            let index = self.nodes.len();
            self.nodes.push(NodeData { first_outgoing_edge: None });
            index
        }
        pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
            let edge_index = self.edges.len();
            let node_data = &mut self.nodes[source];
            self.edges.push(EdgeData {
                target,
                next_outgoing_edge: node_data.first_outgoing_edge
            });
            node_data.first_outgoing_edge = Some(edge_index);
        }

        fn successors(&self, source: NodeIndex) -> Successors {
            let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
            Successors { graph: self, current_edge_index: first_outgoing_edge }
        }

    }
    // fn traverse_add(tree: &mut Vec<FsObject>, name: &[String], children: &[FsObject]) {
    //     for v in tree {
    //         if let FsObject::Directory(v) = v {
    //             if v.name == name && !children.is_empty() {
    //                 v.children = children.to_vec();
    //                 return;
    //             }
    //             traverse_add(&mut v.children, name, children);
    //         }
    //     }
    // }
    //
    // fn traverse(fs: &Vec<FsObject>, root: bool) -> usize {
    //     let mut total = 0;
    //     for v in fs {
    //         if let FsObject::Directory(v) = v {
    //             let d = traverse(&v.children, false);
    //             if root && d <= 100000 {
    //                 total += d;
    //             } else {
    //                 total += traverse(&v.children, false);
    //             }
    //         }
    //         if let FsObject::File(f) = v {
    //             if root {
    //                 continue;
    //             }
    //             total += f.size;
    //         }
    //     }
    //     total
    // }

    pub fn solve(input: &str) -> (usize, usize) {
        let lines: Vec<&str> = input.lines().collect();
        let mut current_dir: Vec<String> = Vec::new();
        let mut ls = false;
        let mut child = Vec::new();
        let mut fs = Graph::default();
        for (i, line) in lines.iter().enumerate() {
            let cmd: Vec<&str> = line.split_whitespace().collect();
            if line.starts_with('$') || i == lines.len() - 1 {
                let seg = &line[2..];
                if seg.starts_with("cd") {
                    let dir = cmd[2];
                    if dir == ".." {
                        current_dir.pop();
                    } else {
                        current_dir.push(dir.to_string());
                        // push
                    }
                }
                ls = seg.starts_with("ls");
                continue;
            }
            if !ls {
                continue;
            }
            let current_dir = current_dir.join("/");
            // match cmd[0] {
            //     "dir" => fs.add_edge(current_dir, current_dir + "/" + cmd[1]),
            //     _ => fs.add_edge(current_dir, line),
            // };
        }
        (0, 0)
    }
}

mod day8 {
    #[inline(always)]
    fn check(grid: &[Vec<u8>], row: &[u8], r: usize, c: usize, tree: u8) -> bool {
        let mut vis = true;
        for i in grid.iter().take(r) {
            if i[c] >= tree {
                vis = false;
            }
        }
        if vis {
            return true;
        }
        let mut vis = true;
        for i in row.iter().take(c) {
            if *i >= tree {
                vis = false;
            }
        }
        if vis {
            return true;
        }
        let mut vis = true;
        for i in row.iter().skip(c + 1) {
            if *i >= tree {
                vis = false;
            }
        }
        if vis {
            return true;
        }
        let mut vis = true;
        for i in grid.iter().take(row.len()).skip(r + 1) {
            if i[c] >= tree {
                vis = false;
            }
        }
        if vis {
            return true;
        }
        false
    }
    #[inline(always)]
    fn check_score(grid: &[Vec<u8>], row: &[u8], r: usize, c: usize, tree: u8) -> usize {
        if r == row.len() - 1 || r == 0 || c == grid.len() - 1 || c == 0 {
            return 0;
        }
        let mut score_up = 0;
        for i in grid.iter().take(r).rev() {
            if i[c] >= tree {
                score_up += 1;
                break;
            }
            if i[c] < tree {
                score_up += 1;
            } else {
                break;
            }
        }
        let mut score_left = 0;
        for i in row.iter().take(c).rev() {
            if *i >= tree {
                score_left += 1;
                break;
            }
            if *i < tree {
                score_left += 1;
            } else {
                break;
            }
        }
        let mut score_right = 0;
        for i in row.iter().skip(c + 1) {
            if *i >= tree {
                score_right += 1;
                break;
            }
            if *i < tree {
                score_right += 1;
            } else {
                break;
            }
        }
        let mut score_down = 0;
        for i in grid.iter().skip(r + 1).take(row.len()) {
            if i[c] >= tree {
                score_down += 1;
                break;
            }
            if i[c] < tree {
                score_down += 1;
            } else {
                break;
            }
        }
        score_up * score_left * score_right * score_down
    }
    pub fn solve(input: &str) -> (usize, usize) {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();
        let mut visible = 0;
        let mut tree_score = vec![];
        for (r, row) in grid.iter().enumerate() {
            for (c, tree) in row.iter().enumerate() {
                if check(&grid, row, r, c, *tree) {
                    visible += 1;
                }
                let score = check_score(&grid, row, r, c, *tree);
                tree_score.push(score);
            }
        }
        tree_score.sort();
        (visible, *tree_score.last().unwrap())
    }
}
