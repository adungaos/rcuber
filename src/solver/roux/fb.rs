use std::collections::HashMap;

use crate::{
    cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE},
    moves::Move::{self, *},
};

use super::{get_available_move, Pruner, SolverBase, SolverConfig};

/// FBSolver for solve Roux's First Block(a 1x2x3 block at left bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::SolverBase;
/// use rcuber::solver::roux::fb::FBSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let f = Formula::scramble();
///     println!("Scramble: {:?}", f);
///     let cc = cc.apply_formula(&f);
///     let mut fb = FBSolver::new(cc);
///     let solution = fb.solve();
///     assert!(fb.is_solved());
///     println!("First Block Solution: {:?}", solution);
/// }
/// ```

#[derive(Debug)]
pub struct FBSolver {
    pub cube: CubieCube,
    config: SolverConfig,
    pruner: FBPruner,
}

impl SolverBase for FBSolver {
    fn new(cube: CubieCube) -> Self {
        let pruner = FBPruner::new();
        let moveset = pruner.moveset.clone();
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }

        let config = SolverConfig {
            min_depth: 0,
            max_depth: 9,
            moveset,
            next_moves,
        };

        Self {
            cube,
            config,
            pruner,
        }
    }

    /// Check if First Block is solved.
    fn is_solved(&self) -> bool {
        if self.cube.center[4] != SOLVED_CUBIE_CUBE.center[4] {
            return false;
        }
        let (corners, edges) = FBSolver::get_state(&self.cube);
        let mut solved = 0;
        for c in corners {
            match c {
                (Corner::DLF, 5, 0) | (Corner::DBL, 6, 0) => solved += 1,
                _ => {}
            };
        }
        for e in edges {
            match e {
                (Edge::FL, 9, 0) | (Edge::BL, 10, 0) | (Edge::DL, 6, 0) => solved += 1,
                _ => {}
            };
        }
        if solved == 5 {
            return true;
        }
        false
    }

    fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        for i in self.config.min_depth..=self.config.max_depth {
            let min_depth = self.config.min_depth;
            solution = Self::solve_depth(
                &self.cube,
                min_depth,
                i,
                &mut self.config,
                &self.pruner,
                FBPruner::encode,
            );
            if solution.len() > 0 {
                break;
            }
        }
        self.cube = self.cube.apply_moves(&solution);
        solution
    }
}

impl FBSolver {
    fn get_state(state: &CubieCube) -> (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>) {
        let mut corners = Vec::new();
        for i in 0..8 {
            match state.cp[i] {
                Corner::DLF | Corner::DBL => corners.push((state.cp[i], i as u8, state.co[i])),
                _ => {}
            }
        }
        let mut edges = Vec::new();
        for i in 0..12 {
            match state.ep[i] {
                Edge::DL | Edge::FL | Edge::BL => edges.push((state.ep[i], i as u8, state.eo[i])),
                _ => {}
            }
        }
        (corners, edges)
    }
}
#[derive(Debug)]
struct FBPruner {
    max_depth: u8,
    dist: Vec<u8>,
    moveset: Vec<Move>,
}

impl Pruner for FBPruner {
    fn new() -> Self {
        let size = 24usize.pow(3) * 24usize.pow(2);
        let max_depth = 4;
        let moveset = vec![
            R, R2, R3, L, L2, L3, U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, M, M2, M3, Rw, Rw2,
            Rw3,
        ];
        let mut dist = Self::init(size, FBPruner::encode, &moveset, max_depth);
        dist[FBPruner::encode(&CubieCube::default())] = 0;
        Self {
            max_depth,
            dist,
            moveset,
        }
    }

    fn encode(cube: &CubieCube) -> usize {
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 0..8 {
            match cube.cp[i] {
                Corner::DLF => c1 = i * 3 + cube.co[i] as usize,
                Corner::DBL => c2 = i * 3 + cube.co[i] as usize,
                _ => {}
            }
        }
        let enc_c = c1 * 24 + c2;
        let mut e1 = 0;
        let mut e2 = 0;
        let mut e3 = 0;
        for i in 0..12 {
            match cube.ep[i] {
                Edge::DL => e1 = i * 2 + cube.eo[i] as usize,
                Edge::FL => e2 = i * 2 + cube.eo[i] as usize,
                Edge::BL => e3 = i * 2 + cube.eo[i] as usize,
                _ => {}
            }
        }
        let enc_e = e1 * (24 * 24) + e2 * (24) + e3;
        enc_e * (24 * 24) + enc_c
    }

    fn query(&self, cube: &CubieCube) -> u8 {
        let d = self.dist[FBPruner::encode(cube)];
        if d == 255 {
            return self.max_depth + 1;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::FBSolver;
    use crate::{
        cubie::CubieCube,
        moves::{Formula, Move::*},
        solver::roux::SolverBase,
    };

    #[test]
    fn test_fb() {
        let cc = CubieCube::default();
        let _f = Formula { moves: vec![L2] };
        let _f = Formula::scramble();
        println!("Scramble: {:?}", _f);
        let cc = cc.apply_formula(&_f);
        let mut solver = FBSolver::new(cc);
        let _s = solver.solve();
        assert!(solver.is_solved());
        println!("First Block Solution: {:?}", _s);
    }
}
