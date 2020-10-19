use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod sequence;
use sequence::Sequence;
use sequence::AlphaBeta;

/*
  Memory Usage:
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
    let num_to_find = 7.0;
    
    let seq_type = "float";
    let format_as_rows = true;

    let base = match seq_type {
        "float" => {
            let mut base: Vec<f32> = Vec::new();
            base.push(alpha);
            base.push(beta);
            Sequence::Float(base)
        },
        "alphabeta" => {
            let mut base: Vec<AlphaBeta> = Vec::new();
            base.push(AlphaBeta{alpha: 1, beta: 0});
            base.push(AlphaBeta{alpha: 0, beta: 1});
            Sequence::AlphaBeta(base)
        },
        _ => panic!("error:  incorrect seq_type")
    };

    if seq_type == "alphabeta" {
        let s = sequence_generator(row_count, base);
        row_generator(s);
        return
    }

    let s = sequence_generator(row_count, base);
    let s_float = s.clone().float();

    min_max(s_float.clone());
    find_elem_index(s_float.clone(), num_to_find);

    if format_as_rows {
        row_generator(s);
    }
}

fn sequence_generator(row_count: u32, mut s: Sequence) -> Sequence {
    let base: i64 = 2;
    let length = base.pow(row_count + 1) - 1;

    for num in 1..(length-1) {
        if num % 2 == 0 {
            let add = s.index((num / 2) as usize) + s.index((num / 2 + 1) as usize);
            s.push(add);
        } else {
            let add = s.index(((num - 1) / 2) as usize);
            s.push(add);
        }
    };

    s
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

fn find_elem_index(s: Vec<f32>, n: f32) {
    let mut results = Vec::new();
    for (index, elem) in s.iter().enumerate() {
        if *elem == n {results.push(index);}
    }

    println!("Indexes where {} occurs:", n);
    for i in results {
        println!("\t{}", i);
    }
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
