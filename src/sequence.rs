use std::ops::Add;
use std::fmt;
use std::iter::Iterator;
// ==================== Term Enum ====================
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Term {
    Int(i32),
    Float(f64),
    AlphaBeta(AlphaBeta)
}

impl Term {
    pub fn int(self) -> i32 {
        match self {
            Term::Int(x) => x,
            Term::Float(x) => x as i32,
            Term::AlphaBeta(_) => panic!("error:  alphabeta cannot be cast as int")
        }
    }

    pub fn float(self) -> f64 {
        match self {
            Term::Int(x) => x as f64,
            Term::Float(x) => x,
            Term::AlphaBeta(_) => panic!("error:  alphabeta cannot be cast as float")
        }
    }

    pub fn alphabeta(self) -> AlphaBeta {
        match self {
            Term::Int(_) => panic!("error:  int cannot be cast as alphabeta"),
            Term::Float(_) => panic!("error:  float cannot be cast as alphabeta"),
            Term::AlphaBeta(x) => x
        }
    }
}

impl Add for Term {
    type Output = Term;

    fn add(self, other: Term) -> Term {
        match self {
            Term::Int(x) => Term::Int(x + other.int()),
            Term::Float(x) => Term::Float(x + other.float()),
            Term::AlphaBeta(x) => Term::AlphaBeta(x + other.alphabeta()),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Float(x) => write!(f, "{}", x),
            Term::Int(x) => write!(f, "{}", x),
            Term::AlphaBeta(x) => write!(f, "{}", x),
        }
    }
}

// ==================== Sequence Enum ====================
#[derive(Clone)]
pub enum Sequence {
    Int(Vec<i32>),
    Float(Vec<f64>),
    AlphaBeta(Vec<AlphaBeta>)
}

impl Sequence {
    pub fn index(&self, index: usize) -> Term {
        match self {
            Sequence::Int(x) => Term::Int(x[index]),
            Sequence::Float(x) => Term::Float(x[index]),
            Sequence::AlphaBeta(x) => Term::AlphaBeta(x[index])
        }
    }

    pub fn remove(&mut self, index: usize) -> String {
        match self {
            Sequence::Int(s) => format!("{}", s.remove(index)),
            Sequence::Float(s) => format!("{}", s.remove(index)),
            Sequence::AlphaBeta(s) => format!("{}", s.remove(index)),
        }
    }

    pub fn push(&mut self, term: Term) {
        match self {
            Sequence::Int(x) => x.push(term.int()),
            Sequence::Float(x) => x.push(term.float()),
            Sequence::AlphaBeta(x) => x.push(term.alphabeta())
        }
    }

    pub fn len(&self) -> usize{
        match self {
            Sequence::Int(x) => x.len(),
            Sequence::Float(x) => x.len(),
            Sequence::AlphaBeta(x) => x.len()
        }
    }
}

impl Iterator for Sequence {
    type Item = Term;

    fn next(&mut self) -> Option<Term> {
        match self {
            Sequence::Int(x) => {
                if x.len() == 0 {return None}
                Some(Term::Int(x.remove(0)))
            }

            Sequence::Float(x) => {
                if x.len() == 0 {return None}
                Some(Term::Float(x.remove(0)))
            }

            Sequence::AlphaBeta(x) => {
                if x.len() == 0 {return None}
                Some(Term::AlphaBeta(x.remove(0)))
            }
        }
    }
}

// ==================== AlphaBeta Struct ====================
#[derive(Copy, Clone, PartialOrd, PartialEq)]
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
