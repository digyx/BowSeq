use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod params;

mod sequence;
use sequence::Sequence;
use sequence::AlphaBeta;

/*
  Memory Usage:
    27 Rows : 1 GB
    28 Rows : 4 GB
    29 Rows : 8 GB
    30 Rows : 16 GB 
*/


fn main() {
    let seq_params = params::get_sequence_params();

    let alpha = seq_params.alpha;
    let beta = seq_params.beta;
    let row_count = seq_params.row_count;
    
    let seq_type = seq_params.sequence_type.as_str();

    let mut base: Sequence = Vec::new();
    base.push(AlphaBeta{alpha: 1, beta: 0});
    base.push(AlphaBeta{alpha: 0, beta: 1});

    if seq_params.standalone {
        let base: i64 = 2;
        let size = (base.pow(row_count + 1) - 1 )as u32;
    
        let seq = Box::new(sequence::new_standalone(alpha, beta, size));

        if seq_params.sum {
            println!("\nSum:");
            println!("\t{}", seq.sum());
        }
        
        if seq_params.mean {
            println!("\nMean:");
            println!("\t{}", seq.mean());
        }

        if seq_params.min_max {
            seq.sum();

            println!("\nMinumum:");            
            println!("\t{}", seq.min());
                        
            println!("\nMaximum:");            
            println!("\t{}", seq.max());            
        }

        return
    }

    if seq_type == "alphabeta" {
        let s = sequence_generator(row_count, base);
        row_generator(s);
        return
    }

    println!("Generating sequence...");
    let s = sequence_generator(row_count, base);
    let seq = Box::new(s);
    println!("Done generating.");

    // Optional functions
    if seq_params.find_elem != 0.0 {
        find_elem_index(&seq, seq_params.find_elem, alpha, beta);
    }

    if seq_params.min_max {
        min_max(&seq, alpha, beta);
    }

    if seq_params.sum {
        println!("\nSum:");
        println!("\t{}", sum(&seq, alpha, beta));
    }
    
    if seq_params.mean {
        mean(&seq, alpha, beta);
    }

    if seq_params.gen_rows {
        println!("Generating rows...");
        row_generator(*seq);
        println!("Done generating");
    }
}

fn sequence_generator(row_count: u32, mut s: Vec<AlphaBeta>) -> Vec<AlphaBeta> {
    let base: i64 = 2;
    let length = base.pow(row_count + 1) - 1;

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

fn min_max(s: &Sequence, alpha: f64, beta: f64) {
    let mut min = s[0];
    let mut max = s[0];

    for num in s {
        let cur_term = num.float(alpha, beta);

        if cur_term > max.float(alpha, beta) {max = *num}
        if cur_term < min.float(alpha, beta) {min = *num}
    }

    println!("\nMaximum:\n\t{}", max);
    println!("\nMinimum:\n\t{}", min);
}

fn sum(s: &Sequence, alpha: f64, beta: f64) -> f64 {
    let mut alpha_sum: u64 = 0;
    let mut beta_sum: u64 = 0;

    for num in s {
        alpha_sum += num.alpha as u64;
        beta_sum += num.beta as u64;
        // println!("{}", sum);
    };
    
    (alpha_sum as f64 * alpha) + (beta_sum as f64 * beta)
}

fn mean(s: &Sequence, alpha: f64, beta: f64) {
    let count = s.len() as f64;

    let sum = sum(s, alpha, beta);

    println!("\nMean:");
    println!("\t{}", sum / count);
}

fn find_elem_index(s: &Sequence, n: f64, alpha: f64, beta: f64) {
    let mut results = Vec::new();
    for (index, elem) in s.iter().enumerate() {
        if elem.float(alpha, beta) == n {
            results.push(index);
        }
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

    println!("")
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
