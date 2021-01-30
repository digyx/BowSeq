use std::env;
use std::process::exit;

use crate::sequence;
use sequence::Term;

pub struct SequenceParams {
    pub alpha: f64,
    pub beta: f64,
    pub row_count: u32,
    pub sequence_type: String,
    pub gen_rows: bool,
    pub min_max: bool,
    pub sum: bool,
    pub mean: bool,
    pub find_elem: Term,
}

pub fn get_sequence_params() -> SequenceParams{
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("\
Bow Sequence Generator {}

    Usage:  bowseq <alpha_term> <beta_term> [options]
            
    Options:
        -count <count>  Amount of rows you would like generated (defaults to 10)
        -type <string>  Representation of the numbers in the sequence (int, float, alphabeta)
        -rowFormat      Log the sequence with row formatting in rows.txt

        -find <num>     Returns the indexes where a specific value appears
        -minmax         Returns the maximum and minimum values in the sequence
        -sum            Returns the sum of the entire sequence
        -mean           Returns the average value of the entire sequence",
    env!("CARGO_PKG_VERSION"));
        exit(1);
    }

    let mut seq_type: String = match args[1].contains(".") {
        true => String::from("float"),
        false => String::from("int")
    };
    
    let alpha: f64 =  match args.remove(1).parse() {
        Ok(num) => num,
        Err(_) => panic!("error: cannot parge alpha term")
    };

    let beta: f64 = match args.remove(1).parse() {
        Ok(num) => num,
        Err(_) => panic!("error: cannot parse beta term")
    };

    let mut row_count: u32 = 10;
    let mut gen_rows: bool = false;
    let mut find_elem: Term = Term::Int(0);
    let mut min_max: bool = false;
    let mut sum: bool = false;
    let mut mean: bool = false;

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
            "-find" => {
                find_elem = match seq_type.as_str() {
                    "int" => Term::Int(args[i+1].parse().unwrap()),
                    "float" => Term::Float(args[i+1].parse().unwrap()),
                    _ => panic!("error:  find_elem can only use int and float")
                }
            },
            "-minmax" => {
                min_max = true;
            },
            "-sum" => {
                sum = true
            },
            "-mean" => {
                mean = true;
            },
            _ => {}
        }
    }
    
    SequenceParams{
        alpha: alpha,
        beta: beta,
        row_count: row_count,
        sequence_type: seq_type,
        gen_rows: gen_rows,
        find_elem: find_elem,
        min_max: min_max,
        sum: sum,
        mean: mean,
    }
}
