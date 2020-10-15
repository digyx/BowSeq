use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod sequence;
use sequence::Sequence;
use sequence::AlphaBeta;

/*
  Memory USAGE:
    26 Rows : 2 GB
    27 Rows : 4 GB
    28 Rows : 8 GB
    29 Rows : 16 GB
    30 Rows : 32 GB 
*/

fn main() {
    let alpha = 0.0;
    let beta = 1.0;
    let row_count = 10;

    let format_as_rows = true;
    let generate_as_alpha_beta = false;

    if generate_as_alpha_beta {
        let s = alpha_beta_sequence_generator(row_count);
        row_generator(s);
        return
    }

    let s = seq_generator(row_count, alpha, beta);

    min_max(s.clone().float());

    if format_as_rows {
        row_generator(s);
    }
}

fn seq_generator(row_count: u32, alpha: f32, beta: f32) -> Sequence {
    let mut s: Vec<f32> = vec![];
    let base: i64 = 2;
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

    Sequence::Float(s)
}

fn alpha_beta_sequence_generator(row_count: u32) -> Sequence {
    let mut s: Vec<AlphaBeta> = Vec::new();
    let base: i64 = 2;
    let length = base.pow(row_count + 1) - 1;
    
    s.push(AlphaBeta{alpha: 1, beta: 0});
    s.push(AlphaBeta{alpha: 0, beta: 1});

    for num in 1..(length-1) {
        if num % 2 == 0 {
            let add = s[(num / 2) as usize] + s[(num / 2 + 1) as usize];
            s.push(add);
        } else {
            let add = s[((num - 1) / 2) as usize];
            s.push(add);
        }
    };

    Sequence::AlphaBeta(s)
}

fn min_max(s: Vec<f32>) {
    let mut max = 0.0;
    let mut min = 0.0;

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

fn row_generator(mut s: Sequence) {
    let mut file = File::create("target/sequence.txt").unwrap();
    let mut contents = format!("{}", s.remove(0));
    
    println!("Formatting data...");
    for num in s {
        contents = contents + format!(" {}", num).as_str();
    }

    println!("Transferring data to Python...");
    file.write_all(contents.as_bytes()).unwrap();

    Command::new("python3")
        .args(&["src/rows.py", "target/sequence.txt", "target/rows.txt"])
        .status().unwrap();
}
