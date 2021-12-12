use std::fs::File;
use std::io::Read;

fn main() {
    let mut opened = File::open(&"./input").unwrap();
    let mut contents: String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst: Vec<&str> = contents.split("\n").collect();
}
