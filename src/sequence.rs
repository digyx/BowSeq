use std::ops::Add;
use std::fmt;
use std::iter::Iterator;
// ==================== Term Enum ====================
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Term {
    Float(f64),
    AlphaBeta(AlphaBeta)
}

impl Term {
    pub fn float(self) -> f64 {
        match self {
            Term::Float(x) => x,
            Term::AlphaBeta(_) => panic!("error:  alphabeta cannot be cast as float")
        }
    }

    pub fn alphabeta(self) -> AlphaBeta {
        match self {
            Term::Float(_) => panic!("error:  float cannot be cast as alphabeta"),
            Term::AlphaBeta(x) => x
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Float(x) => write!(f, "{}", x),
            Term::AlphaBeta(x) => write!(f, "{}", x),
        }
    }
}

// ==================== Sequence Enum ====================
#[derive(Clone)]
pub enum Sequence {
    Float(Vec<f64>),
    AlphaBeta(Vec<AlphaBeta>)
}

impl Sequence {
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

    pub fn len(&self) -> usize{
        match self {
            Sequence::Float(x) => x.len(),
            Sequence::AlphaBeta(x) => x.len()
        }
    }
}

impl Iterator for Sequence {
    type Item = Term;

    fn next(&mut self) -> Option<Term> {
        match self {
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

// ==================== Sequence Enum ====================
#[derive(Clone)]
pub struct StandaloneSequence {
    term: AlphaBeta,
    alpha_value: f64,
    beta_value: f64,
    cur_term: u32,
    limit: u32,
    max: f64,
    min: f64
}

pub fn new(alpha: f64, beta: f64, size: u32) -> StandaloneSequence {
    StandaloneSequence {
        term: AlphaBeta {
            alpha: 0,
            beta: 0
        },
        alpha_value: alpha,
        beta_value: beta,
        cur_term: 0,
        limit: size,
        max: alpha.max(beta),
        min: alpha.min(beta)
    }
}

impl StandaloneSequence {
    pub fn index(&self, index: u32) -> AlphaBeta {
        if index == 1  {
            return AlphaBeta{
                alpha: 1,
                beta: 0
            }
        } else if index == 2 {
            return AlphaBeta {
                alpha: 0,
                beta: 1
            }
        }

        if index % 2 == 0 {
            self.index(index / 2) + self.index(index / 2 + 1)
        } else {
            self.index((index - 1) / 2)
        }
    }

    pub fn next(&mut self) -> Option<f64> {
        if self.cur_term == self.limit {
            return None
        }

        self.cur_term += 1;
        let term = self.index(self.cur_term);

        let alpha = term.alpha as f64 * self.alpha_value;
        let beta = term.beta as f64 * self.beta_value;

        Some(alpha + beta)
    }

    pub fn sum(&self) -> f64 {
        let mut s = self.clone();
        let mut sum = 0.0;

        while self.cur_term <= self.limit {
            match s.next() {
                Some(term) => sum += term,
                None => break
            }
        }

        sum
    }

    pub fn mean(&mut self) -> f64 {
        self.sum() / self.limit as f64
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
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
