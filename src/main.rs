use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod params;

mod sequence;
use sequence::Term;
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
    let seq_params = params::get_sequence_params();

    let alpha = seq_params.alpha;
    let beta = seq_params.beta;
    let row_count = seq_params.row_count;
    
    let seq_type = seq_params.sequence_type.as_str();

    let base = match seq_type {
        "float" => {
            let mut base: Vec<f64> = Vec::new();
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
        "int" => {
            let mut base: Vec<i32> = Vec::new();
            base.push(alpha as i32);
            base.push(beta as i32);
            Sequence::Int(base)
        }
        _ => panic!("error:  incorrect seq_type")
    };

    if seq_type == "alphabeta" {
        let s = sequence_generator(row_count, base);
        row_generator(s);
        return
    }

    println!("Generating sequence...");
    let s = sequence_generator(row_count, base);
    println!("Done generating.");

    // Optional functions
    if seq_params.gen_rows {
        println!("Generating rows...");
        row_generator(s.clone());
        println!("Done generating");
    }
    
    if seq_params.min_max {
        min_max(s.clone());
    }
    
    if seq_params.mean {
        mean(s.clone());
    }

    if seq_params.find_elem != Term::Int(0) {
        find_elem_index(s.clone(), seq_params.find_elem);
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

fn min_max(s: Sequence) {
    let mut max = match s {
        Sequence::Float(_) => Term::Float(0.0),
        Sequence::Int(_) => Term::Int(0),
        _ => panic!("error:  min_max can only be used with floats and ints"),
    };

    let mut min = match s {
        Sequence::Float(_) => Term::Float(0.0),
        Sequence::Int(_) => Term::Int(0),
        _ => panic!("error:  min_max can only be used with floats and ints"),
    };

    for num in s {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
    }

    println!("\nMinimum and Maximum:");
    println!("\tMax: {}", max);
    println!("\tMin: {}", min);
}

fn mean(s: Sequence) {
    let count = s.len() as f64;

    // Looping through the Term enum is absurdly slow
    let sum = match s {  
        Sequence::Float(x) => {
            let mut sum = 0.0;
            for num in x {sum += num};
            sum
        },
        Sequence::Int(x) => {
            let mut sum = 0;
            for num in x {sum += num};
            sum as f64
        },
        _ => panic!("error: incompatible type for mean function")
    };

    println!("\nMean:");
    println!("\t{}", sum/count);
}

fn find_elem_index(s: Sequence, n: Term) {
    let mut results = Vec::new();
    for (index, elem) in s.enumerate() {
        if elem == n {results.push(index);}
    }

    let mut i = 0;
    println!("\nIndexes where {} occurs:", n);
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
        .args(&["rows.py", "sequence.txt", "rows.txt"])
        .status().unwrap();
}
