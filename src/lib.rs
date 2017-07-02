//! Problems
#[macro_use]
extern crate itertools;

use std::error;
use std::fmt;

/// Solve problems with inputs & outputs
trait Solvable {
    type I; // Input, Premise
    type O; // Output, Solution

    fn input(&self) -> &Self::I;
    fn solve(&self) -> Result<Self::O, UnsolvableError>;
}

/// Error when a problem is unsolvable
#[derive(Debug)]
enum UnsolvableError {
    NoSolution
}

impl fmt::Display for UnsolvableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsolved")
    }
}

impl error::Error for UnsolvableError {
    fn description(&self) -> &str { "Cannot solve" }
}

/// template
#[allow(dead_code)]
mod _stub;

/// 1
#[allow(dead_code)]
mod triangle;
#[allow(dead_code)]
mod ants;
#[allow(dead_code)]
mod lottery;

/// 2-1
mod partial_sum;
mod lake_counting;
