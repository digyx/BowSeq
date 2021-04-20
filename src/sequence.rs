use std::ops::Add;
use std::fmt;

pub type Sequence = Vec<AlphaBeta>;

// ==================== Standalone Sequence ====================
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

pub fn new_standalone(alpha: f64, beta: f64, size: u32) -> StandaloneSequence {
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

    pub fn mean(&self) -> f64 {
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
#[derive(Copy, Clone)]
pub struct AlphaBeta {
    pub alpha: u32,
    pub beta: u32,
}

impl AlphaBeta {
    pub fn float(&self, alpha: f64, beta: f64) -> f64 {
        (self.alpha as f64 * alpha) + (self.beta as f64 * beta)
    }
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
