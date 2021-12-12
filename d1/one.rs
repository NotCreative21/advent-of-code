use std::fs::File;
use std::io::Read;
fn main() {
    let mut opened = File::open(&"./input").unwrap();
         
    let mut contents:String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst:Vec<&str> = contents.split("\n").collect();
    let mut output = 0;
    println!("{}", vecfirst.len());
    for e in 0..vecfirst.len() {
        if e > 0 && vecfirst[e - 1] < vecfirst[e] {
            output += 1;
        }
    }
    println!("larger: {}", output);
}
