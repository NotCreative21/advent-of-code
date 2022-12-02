use std::fs::File;
use std::io::Read;

fn main() {
    let mut opened = File::open(&"./input").unwrap();
    let mut contents: String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst: Vec<&str> = contents.split("\n").collect();
    let binlen = vecfirst[0].len();
    let mut result: Vec<u8> = Vec::new();
    let mut result2: Vec<u8> = Vec::new();

    for d in 0..binlen {
        let (mut x, mut y) = (0, 0);
        for i in 0..vecfirst.len() {
            let testvec: Vec<char> = vecfirst[i].chars().collect();
            if testvec.len() == 0 {
                continue;
            }
            let z = testvec[d].to_digit(2).unwrap();
            println!("{} : {:?}", z, testvec);
            if z == 0 {
                x += 1;
            }
            else {
                y += 1;
            }
        } 
        if x > y {
            result.push(0);
            result2.push(1);
        }
        else {
            result.push(1);
            result2.push(0)
        }
    }

    let binstr: String = result.iter().map( |&i| i.to_string() + "").collect();
    let strbin: String = result2.iter().map( |&i| i.to_string() + "").collect();
    let a = isize::from_str_radix(&binstr, 2).unwrap();
    let b = isize::from_str_radix(&strbin, 2).unwrap();
    println!("{:?}\n{:?}\n{} * {} = {}", binstr, strbin, a, b, a*b);
}
