use std::fs::File;
use std::io::Read;
fn main() {
    let mut opened = File::open(&"./input").unwrap();

    let mut contents: String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst: Vec<&str> = contents.split("\n").collect();
    println!("{}", vecfirst.len());
    let (mut i, mut output) = (0, 0);
    let mut vecfinal: Vec<u32> = Vec::new();
    while i < (vecfirst.len() - 3) as u32 {
        vecfinal.push(
            vecfirst[i as usize].parse::<u32>().unwrap()
                + vecfirst[(i + 1) as usize].parse::<u32>().unwrap()
                + vecfirst[(i + 2) as usize].parse::<u32>().unwrap(),
        );
        i += 1;
    }

    for e in 0..vecfinal.len() {
        if e > 0 && vecfinal[e - 1] < vecfinal[e] {
            output += 1;
        }
    }
    println!("larger: {}", output);
}
