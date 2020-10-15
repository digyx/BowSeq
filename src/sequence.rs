use std::ops::Add;
use std::fmt;
use std::iter::Iterator;

// ==================== Sequence Enum ====================
#[derive(Clone)]
pub enum Sequence {
    Float(Vec<f32>),
    AlphaBeta(Vec<AlphaBeta>)
}

impl Sequence {
    pub fn float(self) -> Vec<f32> {
        match self {
            Sequence::Float(x) => x,
            Sequence::AlphaBeta(_) => panic!("error:  alphabeta where float expected")
        }
    }

    #[allow(dead_code)]  // Could be used in the future, so exists now
    pub fn alphabeta(self) -> Vec<AlphaBeta> {
        match self {
            Sequence::Float(_) => panic!("error:  float where alphabeta expected"),
            Sequence::AlphaBeta(x) => x
        }
    }

    pub fn remove(&mut self, index: usize) -> String {
        match self {
            Sequence::Float(s) => format!("{}", s.remove(index)),
            Sequence::AlphaBeta(s) => format!("{}", s.remove(index)),
        }
    }
}

impl Iterator for Sequence {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self {
            Sequence::Float(x) => {
                if x.len() == 0 {return None}
                Some(format!("{}", x.remove(0)))
            }
            Sequence::AlphaBeta(x) => {
                if x.len() == 0 {return None}
                Some(format!("{}", x.remove(0)))
            }
        }
    }
}

// ==================== AlphaBeta Struct ====================
#[derive(Copy, Clone)]
pub struct AlphaBeta {
    pub alpha: i32,
    pub beta: i32,
}

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}a+{}b", self.alpha, self.beta)
    }
}

impl Add for AlphaBeta {
    type Output = AlphaBeta;

    fn add(self, other: AlphaBeta) -> AlphaBeta {
        AlphaBeta{
            alpha: self.alpha + other.beta,
            beta: self.beta + other.beta
        }
    }
}
