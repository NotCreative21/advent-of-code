use std::fs::File;
use std::io::Read;

fn main() {
    let mut opened = File::open(&"./input").unwrap();
    let mut contents: String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst: Vec<&str> = contents.split("\n").collect();
    let (mut x, mut y) = (0, 0);

    for e in vecfirst.iter() {
        let test_str: Vec<&str> = e.split(" ").collect();
        match test_str[0] {
            "forward" => x += test_str[1].parse::<i32>().unwrap(),
            "up" => y += test_str[1].parse::<i32>().unwrap(),
            "down" => y -= test_str[1].parse::<i32>().unwrap(),
            _ => continue,
        }
    }
    println!("{} * {} = {}", x, y, (x*y).abs());
}
