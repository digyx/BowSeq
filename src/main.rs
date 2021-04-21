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

    // Initialize the Sequnce acording to the definition
    let mut base: Sequence = Vec::new();
    base.push(AlphaBeta{alpha: 1, beta: 0});
    base.push(AlphaBeta{alpha: 0, beta: 1});

    /*
        Slower method of calculating large rows, but uses almost no memory.
        In the future, this can be used after a certain row count, allowing for calculating
        more rows where the current method would be impractical because of RAM limitations
    */
    if seq_params.standalone {
        let base: i64 = 2;
        let length = (base.pow(row_count + 1) - 1 )as u32;
    
        let seq = Box::new(sequence::new_standalone(alpha, beta, length));

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

    /*
        Only used for generating a Row Format txt file
        AlphaBeta notation has no other practical use as of writing this comment
    */
    if seq_type == "alphabeta" {
        let s = sequence_generator(row_count, base);
        row_generator(s);
        return
    }

    /* 
        The sequence is generated and stored in a box.
        Boxes function as storing on the Heap rather than the Stack, so the object is
        not passed around when used by a function.  This halves RAM usage by allowing
        us to use the sequence without cloning the sequence every time it needs to be used
    */
    println!("Generating sequence...");
    let seq = Box::new(sequence_generator(row_count, base));
    println!("Done generating.");

    // Analysis functions
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

/*
    The sequence is generated using the recursive definition and stored in an AlphaBeta
    Vector, renamed sequence for convenience sake.  While very fast for computing values,
    this uses a collosal amount of RAM as each row calculated doubles the amount used.

    Using the above standlone method, analysis computations could be done with
    higher row amounts by using on-the-fly computation.  Combining the two, as well as
    parallelising the number generations would add significant value.
*/
fn sequence_generator(row_count: u32, mut s: Sequence) -> Sequence {
    let base: i64 = 2;
    let length = base.pow(row_count + 1) - 1;

    for num in 1..(length-1) {
        // The current index is even
        if num % 2 == 0 {
            let cur_term = s[(num / 2) as usize] + s[(num / 2 + 1) as usize];
            s.push(cur_term);
            continue
        }
        // The current index is odd
        let cur_term = s[((num - 1) / 2) as usize];
        s.push(cur_term);
    };

    s
}

/* 
    AlphaBeta notation's primary downside is that the actual value must be calculated
    for almost all comparisons.  The time spent on converting 30 rows (over a billion terms)
    is not signifcant, especially when compared to the time spent generating rows.
*/
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

/* 
    Since AlphaBeta is stored as two u32 variables for RAM usage, the sums cannot be done
    on single term since the 32-bit unsigned integer limit is hit far too quickly.  Instead,
    we do the addition separately on two 64-bit unsigned integers.
*/
fn sum(s: &Sequence, alpha: f64, beta: f64) -> f64 {
    let mut alpha_sum: u64 = 0;
    let mut beta_sum: u64 = 0;

    for num in s {
        alpha_sum += num.alpha as u64;
        beta_sum += num.beta as u64;
    };
    
    (alpha_sum as f64 * alpha) + (beta_sum as f64 * beta)
}

/* 
    Mean is just a basic combination of the sum function and the length of the vector.
*/
fn mean(s: &Sequence, alpha: f64, beta: f64) {
    let count = s.len() as f64;
    let sum = sum(s, alpha, beta);

    println!("\nMean:");
    println!("\t{}", sum / count);
}

/* 
    Since this requries comparisons, each term must be converted to a float.  After that,
    it essentially is just rolling through the vector and logs the indexes for the given
    number to find.
*/
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

/* 
    This program was originally written in Python, ported to Rust for speed, so the
    row generator was never ported since it ran fast enough as is and my understanding
    of Rust at the time was not good enough.  One improvement would be either porting
    rows.py to Rust or using PyO3 to call the functions without being forced to pass
    the sequence data via a file.
*/
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
