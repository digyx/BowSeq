use std::env;
use std::process::exit;

pub struct SequenceParams {
    pub alpha: f32,
    pub beta: f32,
    pub row_count: u32,
    pub sequence_type: String,
    pub gen_rows: bool,
    pub min_max: bool,
    pub mean: bool,
    pub find_elem: f32,
}

pub fn get_sequence_params() -> SequenceParams{
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
    let mut mean: bool = false;
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
        min_max: min_max,
        mean: mean,
        find_elem: find_elem,
    }
}
