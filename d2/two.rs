use std::fs::File;
use std::io::Read;

fn main() {
    let mut opened = File::open(&"./input").unwrap();
    let mut contents: String = String::new();
    opened.read_to_string(&mut contents).unwrap();
    let vecfirst: Vec<&str> = contents.split("\n").collect();
    let (mut x, mut y, mut z) = (0, 0, 0);

    for e in vecfirst.iter() {
        let test_str: Vec<&str> = e.split(" ").collect();
        println!("{} {} {}", x, y, z);
        match test_str[0] {
            "forward" => { 
                let w = test_str[1].parse::<i32>().unwrap(); 
                x += w;
                y += z * w;
            },
            "up" => {
                z -= test_str[1].parse::<i32>().unwrap(); 
            },
            "down" => { 
                z += test_str[1].parse::<i32>().unwrap(); 
            },
            _ => continue,
        }
    }
    println!("{} * {} = {}", x, y, (x*y).abs());
}
