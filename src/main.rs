use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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
    let format_as_rows = true;  // Will fail if alpha and beta are not integers
    let generate_as_alpha_beta = false;

    if generate_as_alpha_beta {
        let s = alpha_beta_sequence_generator(row_count);
        second_row_generator(s);
        return
    }

    let s = seq_generator(row_count, alpha, beta);

    min_max(s.clone());

    if format_as_rows {
        row_generator(s.clone());
    }
}

fn seq_generator(row_count: u32, alpha: f32, beta: f32) -> Vec<f32> {
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

    s
}

fn alpha_beta_sequence_generator(row_count: u32) -> Vec<[i32; 2]>{
    let mut s: Vec<[i32; 2]> = Vec::new();
    let base: i64 = 2;
    let length = base.pow(row_count + 1) - 1;
    
    s.push([1, 0]);
    s.push([0, 1]);

    for num in 1..(length-1) {
        if num % 2 == 0 {
            let mut add = [0, 0];
            for i in 0..2 {
                let component = s[(num / 2) as usize][i] + s[(num / 2 + 1) as usize][i];
                add[i] = component;
            }

            s.push(add);
        } else {
            let add = s[((num - 1) / 2) as usize];
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

fn row_generator(mut s: Vec<f32>) {
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

fn second_row_generator(mut s: Vec<[i32; 2]>) {
    let mut file = File::create("target/sequence.txt").unwrap();
    let init_term = s.remove(0);
    let mut contents = format!("{}a+{}b", init_term[0], init_term[1]);
    
    println!("Formatting data...");
    for num in s {
        contents = contents + format!(" {}a+{}b", num[0], num[1]).as_str();
    }

    println!("Transferring data to Python...");
    file.write_all(contents.as_bytes()).unwrap();

    Command::new("python3")
        .args(&["src/rows.py", "target/sequence.txt", "target/rows.txt"])
        .status().unwrap();
}
