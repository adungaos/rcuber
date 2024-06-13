//! # RCuber 
//! `RCuber` - crate for rubiks cube and solvers(CFOP,LBL,Roux).

pub mod error;
/// Module containing 3x3 cube constants.
pub mod constants;
/// Module for represent a cube on the facelet level.
pub mod facelet;
/// Module for represent a cube on the cubie level.
pub mod cubie;
/// Module for represent moves.
pub mod moves;
/// Module for Solvers.
pub mod solver;
#[cfg(feature = "term")]
/// Module for print a facelet cube on terminal witch color.
pub mod printer;

use std::str::FromStr;
use rand::random;

use moves::Move::{self, *};

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