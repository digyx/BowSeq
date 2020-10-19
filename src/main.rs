use std::env;
use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, exit};

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

struct SequenceParams {
    alpha: f32,
    beta: f32,
    row_count: u32,
    sequence_type: String,
    gen_rows: bool,
    min_max: bool,
    find_elem: f32,
}


fn main() {
    let seq_params = get_sequence_params();

    let alpha = seq_params.alpha;
    let beta = seq_params.beta;
    let row_count = seq_params.row_count;
    
    let seq_type = seq_params.sequence_type.as_str();

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

    // Optional functions
    if seq_params.gen_rows {
        row_generator(s.clone());
    }
    
    if seq_params.min_max {
        min_max(s_float.clone());
    }
    
    if seq_params.find_elem != 0.0 {
        find_elem_index(s_float.clone(), seq_params.find_elem);
    }
}

fn get_sequence_params() -> SequenceParams{
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("error:  Must supply at least one alpha and beta value");
        exit(1);
    }
    
    let alpha: f32 = args.remove(1).parse().unwrap();
    let beta: f32 = args.remove(1).parse().unwrap();

    let mut row_count: u32 = 10;
    let mut seq_type = String::from("float");
    let mut gen_rows: bool = false;
    let mut min_max: bool = false;
    let mut find_elem: f32 = 0.0;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-count" => {
                row_count = args[i+1].parse().unwrap();
            },
            "-type" => {
                seq_type = args[i+1].clone();
            },
            "-rowFormat" => {
                gen_rows = true;
            },
            "-minmax" => {
                min_max = true;
            },
            "-find" => {
                find_elem = args[i+1].parse().unwrap();
            }
            _ => {}
        }
    }
    
    SequenceParams{
        alpha: alpha,
        beta: beta,
        row_count: row_count,
        sequence_type: seq_type,
        gen_rows: gen_rows,
        min_max: min_max,
        find_elem: find_elem,
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

    println!("Minimum and Maximum:");
    println!("\tMax: {}", max);
    println!("\tMin: {}", min);
}

fn find_elem_index(s: Vec<f32>, n: f32) {
    let mut results = Vec::new();
    for (index, elem) in s.iter().enumerate() {
        if *elem == n {results.push(index);}
    }

    let mut i = 0;
    println!("Indexes where {} occurs:", n);
    for result in results {
        if i == 5 {
            println!("");
            i = 0;
        }
        print!("\t{}", result);
        i += 1;
    }
}

fn row_generator(mut s: Sequence) {
    let mut file = File::create("sequence.txt").unwrap();
    let mut contents = format!("{}", s.remove(0));
    
    println!("Formatting data...");
    for num in s {
        contents = contents + format!(" {}", num).as_str();
    }

    println!("Transferring data to Python...");
    file.write_all(contents.as_bytes()).unwrap();

    Command::new("python3")
        .args(&["src/rows.py", "sequence.txt", "rows.txt"])
        .status().unwrap();
}
