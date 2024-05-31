pub mod error;
pub mod constants;
pub mod utils;
pub mod facelet;
pub mod cubie;
pub mod moves;
pub mod printer;
pub mod solver;

use std::str::FromStr;

use moves::Move::{self, *};
use rand::random;

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