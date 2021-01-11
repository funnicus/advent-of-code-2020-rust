use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/report.txt";
    // Open the file in read-only mode (ignoring errors with unwrap, which can cause a crash).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut v: Vec<i32> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => panic!("Line reading failed!"),
        };
        v.push(line.trim().parse().unwrap());
    }

    for (i, x) in v.iter().enumerate() {
        for (j, y) in v.iter().enumerate() {
            if j <= i { continue; }
            if x + y == 2020 {
                println!("Values {} + {} will result in 2020!", x, y);
                let result = x*y;
                println!("The result is: {}", result);
            }
            for (l, z) in v.iter().enumerate() {
                if l <= j || l <= i { continue; }
                if x + y + z == 2020 {
                    println!("Values {} + {} + {} will result in 2020!", x, y, z);
                    let result = x*y*z;
                    println!("The result is: {}", result);
                }
            }
        }
    }
}
