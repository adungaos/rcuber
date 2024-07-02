//! # min2phase 
//! `min2phase` - crate for rubiks cube and solver(min2phase).

/// Module for represent moves.
pub mod utils;
/// Module containing 3x3 cube constants.
pub mod constants;
/// Module for represent a cube on the facelet level.
pub mod cubie;
/// Module for represent a cube on the cubie level(array model).
pub mod arraycube;
/// Module for represent a cube on the coordinate level.
pub mod coord;
/// Module for data tables.
pub mod tables;
/// Module for min2phase search/solver.
pub mod solver;

pub use solver::Solver as Min2PhaseSolver;

use std::str::FromStr;
use rand::random;

use crate::moves::Move::{self, *};

/// Generate a random scramble formula.
pub fn scramble() -> Vec<Move> {
    let mut r = Vec::new();
    let mut p = B;
    for _ in 0..25 {
        let m = match random::<u32>() % 6 {
            0 => U,
            1 => R,
            2 => F,
            3 => D,
            4 => L,
            _ => B,
        };
        if m == p {
            continue;
        }
        let s = match random::<u32>() % 3 {
            0 => "",
            1 => "2",
            _ => "'",
        };
        let mv = format!("{:?}{}", m, s);
        let mv = Move::from_str(mv.as_str()).unwrap();
        r.push(mv);
        p = m;
    }
    r
}


#[cfg(test)]
mod tests {
    use crate::scramble;

    #[test]
    fn test_scramble(){
        let r = scramble();
        println!("{:?}", r);
    }
}