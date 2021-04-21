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
}

/* 
    Okay, we'll have the Standalone conversation here.  This is something I whipped up
    in a few days based on my desire to not rent out AWS space to calculate some numbers.
    Since faking infinity computationally requires a very large number (and row formatting
    is very useful with this sequence), the amount of RAM necessary to guesstimate what
    heading off to infinity is obscene (AWS only offers up to 4TB of RAM, which would
    end up generating 42 rows).

    To combat this, what if computations such as max and min were guesstimated used
    on-the-fly indexing instead of storing each number?  Since there is no non-recursive
    formula (yet) for calculating a given value knowing just alpha, beta, and the index,
    each number has to be calculated recursively,  Because of the how the sequence works,
    this triples the computation time for each additional row, compared to double the
    current primary method uses.

    "Why keep this here?" Becuase if more rows are neded, tripling computation time can
    be mitigated using parrallel computations.  Running this in a swarm of AWS nodes that
    total out to be 1024 CPU cores would drastically reduce computation time since each
    instance could store the previous 42 rows in memory and compute the next ones on the
    fly, communicating the results to a master node.
*/
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
    }
}

impl StandaloneSequence {
    /* 
        Recursive function used for on-the-fly term computation.  Calculates what the term
        at the given index should be, as an AlphaBeta, and then returns it
    */
    pub fn index(&self, index: u32) -> AlphaBeta {
        // Base values
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

        // Index is even
        if index % 2 == 0 {
            return self.index(index / 2) + self.index(index / 2 + 1)
        }
        
        // Index is odd
        self.index((index - 1) / 2)
    }

    /* 
        Generates the next term in the sequence as a float, stopping at the limit.
        This could probably be parallelised
    */
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

    /* 

    */
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

    // TODO
    pub fn min(&self) -> f64 {
        println!("Not yet implemented");
        0.0
    }

    // TODO
    pub fn max(&self) -> f64 {
        println!("Not yet implemented");
        0.0
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

// Used for row generation
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
