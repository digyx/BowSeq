use std::vec::Vec;

fn main() {
    let alpha = -1.0;
    let beta = 2.61803398874989484820458683436563811772030917980;
    let length = 10000000;

    let s = seq_generator(length, alpha, beta);

    min_max(s);
}

fn seq_generator(length: i32, alpha: f32, beta: f32) -> Vec<f32> {
    let mut s: Vec<f32> = vec![];

    s.push(alpha);
    s.push(beta);

    for num in 1..length {
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
