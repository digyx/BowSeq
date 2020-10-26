use std::ops::Add;
use std::fmt;
use std::iter::Iterator;

// ==================== Term Enum ====================
#[derive(Copy, Clone)]
pub enum Term {
    Float(f32),
    AlphaBeta(AlphaBeta)
}

impl Term {
    pub fn float(self) -> f32 {
        match self {
            Term::Float(x) => x,
            Term::AlphaBeta(_) => panic!("error:  alphabeta where float expected")
        }
    }

    pub fn alphabeta(self) -> AlphaBeta {
        match self {
            Term::Float(_) => panic!("error:  float where alphabeta expected"),
            Term::AlphaBeta(x) => x
        }
    }
}

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
            _ => panic!("error:  float expected")
        }
    }

    #[allow(dead_code)]  // Could be used in the future, so exists now
    pub fn alphabeta(self) -> Vec<AlphaBeta> {
        match self {
            Sequence::AlphaBeta(x) => x,
            _ => panic!("error:  alphabeta expected")
        }
    }

    pub fn index(&self, index: usize) -> Term {
        match self {
            Sequence::Float(x) => Term::Float(x[index]),
            Sequence::AlphaBeta(x) => Term::AlphaBeta(x[index])
        }
    }

    pub fn remove(&mut self, index: usize) -> String {
        match self {
            Sequence::Float(s) => format!("{}", s.remove(index)),
            Sequence::AlphaBeta(s) => format!("{}", s.remove(index)),
        }
    }

    pub fn push(&mut self, term: Term) {
        match self {
            Sequence::Float(x) => x.push(term.float()),
            Sequence::AlphaBeta(x) => x.push(term.alphabeta())
        }
    }
}

impl Add for Term {
    type Output = Term;

    fn add(self, other: Term) -> Term {
        match self {
            Term::Float(x) => Term::Float(x + other.float()),
            Term::AlphaBeta(x) => Term::AlphaBeta(x + other.alphabeta()),
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
            alpha: self.alpha + other.alpha,
            beta: self.beta + other.beta
        }
    }
}
