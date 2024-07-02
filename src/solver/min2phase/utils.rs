use std::fmt::Display;

use super::{
    constants::{ALL_MOVES, APPEND_LENGTH, INVERSE_SOLUTION, USE_SEPARATOR},
    tables::MT,
};
use crate::moves::Move::{self, *};
use static_init::dynamic;

#[derive(Debug)]
pub struct Solution {
    pub length: usize,
    pub depth1: usize,
    pub verbose: usize,
    pub urf_idx: usize,
    pub moves: [i32; 31],
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        let urf = match (self.verbose & INVERSE_SOLUTION) != 0 {
            true => (self.urf_idx + 3) % 6,
            false => self.urf_idx,
        };
        if urf < 3 {
            for s in 0..self.length {
                if (self.verbose & USE_SEPARATOR) != 0 && s == self.depth1 {
                    res.push_str(". ");
                }
                res.push_str(
                    format!(
                        "{}",
                        ALL_MOVES[MT.urf_move[urf][self.moves[s] as usize] as usize]
                    )
                    .as_str(),
                );
                res.push_str(" ");
            }
        } else {
            for s in (0..self.length).rev() {
                res.push_str(
                    format!(
                        "{}",
                        ALL_MOVES[MT.urf_move[urf][self.moves[s] as usize] as usize]
                    )
                    .as_str(),
                );
                res.push_str(" ");
                if (self.verbose & USE_SEPARATOR) != 0 && s == self.depth1 {
                    res.push_str(". ");
                }
            }
        }
        if self.verbose & APPEND_LENGTH != 0 {
            res.push_str(format!("({}f)", self.length).as_str());
        }
        write!(f, "{}", res)
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            length: 0,
            depth1: 0,
            verbose: 0,
            urf_idx: 0,
            moves: [0; 31],
        }
    }

    pub fn set_args(&mut self, verbose: usize, urf_idx: usize, depth1: usize) {
        self.verbose = verbose;
        self.urf_idx = urf_idx;
        self.depth1 = depth1;
    }

    pub fn append_sol_move(&mut self, cur_move: i32) {
        if self.length == 0 {
            self.length += 1;
            self.moves[0] = cur_move;
            return;
        }
        let axis_cur = cur_move / 3;
        let axis_last = self.moves[self.length - 1] / 3;
        if axis_cur == axis_last {
            let pow = (cur_move % 3 + self.moves[self.length - 1] % 3 + 1) % 4;
            if pow == 3 {
                self.length -= 1;
            } else {
                self.moves[self.length - 1] = axis_cur * 3 + pow;
            }
            return;
        }
        if self.length > 1
            && axis_cur % 3 == axis_last % 3
            && axis_cur == self.moves[self.length - 2] / 3
        {
            let pow = (cur_move % 3 + self.moves[self.length - 2] % 3 + 1) % 4;
            if pow == 3 {
                self.moves[self.length - 2] = self.moves[self.length - 1];
                self.length -= 1;
            } else {
                self.moves[self.length - 2] = axis_cur * 3 + pow;
            }
            return;
        }
        self.moves[self.length] = cur_move;
        self.length += 1;
    }
}

#[derive(Debug)]
pub struct UtilTables {
    pub cnk: [[u16; 13]; 13],
    pub ud2std: [Move; 18],
    pub std2ud: [usize; 18],
    pub ckmv2bit: [i32; 11],
    pub fact: [usize; 14],
}

impl UtilTables {
    pub fn new() -> Self {
        let mut cnk = [[0; 13]; 13];
        let ud2std = [
            U, U2, U3, R2, F2, D, D2, D3, L2, B2, R, R3, F, F3, L, L3, B, B3,
        ];
        let mut std2ud = [0; 18];
        let mut ckmv2bit = [0; 11];
        let mut fact = [1; 14];
        for i in 0..18 {
            std2ud[ud2std[i] as usize] = i;
        }
        for i in 0..10 {
            let ix = ud2std[i] as usize / 3;
            ckmv2bit[i] = 0;
            for j in 0..10 {
                let jx = ud2std[j] as usize / 3;
                let xx = match (ix == jx) || ((ix % 3 == jx % 3) && (ix >= jx)) {
                    true => 1,
                    false => 0,
                };
                ckmv2bit[i] |= xx << j;
            }
        }
        ckmv2bit[10] = 0;

        for i in 0..13 {
            cnk[i][0] = 1;
            cnk[i][i] = 1;
            fact[i + 1] = fact[i] * (i + 1);
            for j in 1..i {
                cnk[i][j] = cnk[i - 1][j - 1] + cnk[i - 1][j];
            }
        }
        Self {
            cnk,
            ud2std,
            std2ud,
            ckmv2bit,
            fact,
        }
    }
}

#[dynamic(lazy)]
pub static UT: UtilTables = UtilTables::new();

#[cfg(test)]
mod tests {

    use super::UtilTables;

    #[test]
    fn test_utils() {
        let utils: UtilTables = UtilTables::new();
        // println!("{:?}", utils);
        assert_eq!(utils.cnk[12][6], 924);
        assert_eq!(utils.cnk.iter().flatten().max().unwrap(), &924);
    }
}
