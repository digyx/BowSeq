use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let alpha = 0;
    let beta = 1;
    let row_count = 10;

    let s = seq_generator(row_count, alpha, beta);

    min_max(s.clone());
    row_generator(s.clone());
}

fn seq_generator(row_count: u32, alpha: i32, beta: i32) -> Vec<i32> {
    let mut s: Vec<i32> = vec![];
    let base: i32 = 2;
    let length = base.pow(row_count + 1) - 1;

    s.push(alpha);
    s.push(beta);

    for num in 1..(length-1) {
        if num % 2 == 0 {
            let add = s[(num / 2) as usize] + s[(num / 2 + 1) as usize];
            s.push(add);
        } else {
            let add = s[((num - 1) / 2) as usize];
            s.push(add);
        }
    };

    s
}

fn min_max(s: Vec<i32>) {
    let mut max = 0;
    let mut min = 0;

    for num in s {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
    }

    println!("Max: {}", max);
    println!("Min: {}", min);
}

fn row_generator(mut s: Vec<i32>) {
    let mut file = File::create("target/sequence.txt").unwrap();
    let mut contents = format!("{}", s.remove(0));
    
    for num in s {
        contents = format!("{} {}", contents, num);
    }

    file.write_all(contents.as_bytes()).unwrap();

    Command::new("python3").arg("src/rows.py").status().unwrap();
}
