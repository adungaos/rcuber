use std::collections::HashMap;

use crate::{
    cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE},
    moves::Move::{self, *},
};

use super::{get_available_move, Pruner, SolverBase, SolverConfig};

/// SBSolver for solve Roux's Second Block(a 1x2x3 block at right bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::SolverBase;
/// use rcuber::solver::roux::fb::FBSolver;
/// use rcuber::solver::roux::sb::SBSolver;
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
///     let mut sb = SBSolver::new(fb.cube);
///     let solution = sb.solve();
///     assert!(sb.is_solved());
///     println!("Second Block Solution: {:?}", solution);
/// }
/// ```

#[derive(Debug)]
pub struct SBSolver {
    pub cube: CubieCube,
    config: SolverConfig,
    pruner: SBPruner,
}

impl SolverBase for SBSolver {
    fn new(cube: CubieCube) -> Self {
        let pruner = SBPruner::new();
        let moveset = pruner.moveset.clone();
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }

        let config = SolverConfig {
            min_depth: 0,
            max_depth: 14,
            moveset,
            next_moves,
        };

        Self {
            cube,
            config,
            pruner,
        }
    }

    /// Check if Second Block is solved.
    fn is_solved(&self) -> bool {
        if self.cube.center[1] != SOLVED_CUBIE_CUBE.center[1] {
            return false;
        }
        let (corners, edges) = SBSolver::get_state(&self.cube);
        let mut solved = 0;
        for c in corners {
            match c {
                (Corner::DFR, 4, 0) | (Corner::DRB, 7, 0) => solved += 1,
                _ => {}
            };
        }
        for e in edges {
            match e {
                (Edge::DR, 4, 0) | (Edge::FR, 8, 0) | (Edge::BR, 11, 0) => solved += 1,
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
                SBPruner::encode,
            );
            if solution.len() > 0 {
                break;
            }
        }
        self.cube = self.cube.apply_moves(&solution);
        solution
    }
}

impl SBSolver {
    fn get_state(state: &CubieCube) -> (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>) {
        let mut corners = Vec::new();
        for i in 0..8 {
            match state.cp[i] {
                Corner::DFR | Corner::DRB => corners.push((state.cp[i], i as u8, state.co[i])),
                _ => {}
            }
        }
        let mut edges = Vec::new();
        for i in 0..12 {
            match state.ep[i] {
                Edge::DR | Edge::FR | Edge::BR => edges.push((state.ep[i], i as u8, state.eo[i])),
                _ => {}
            }
        }
        (corners, edges)
    }
}

#[derive(Debug)]
struct SBPruner {
    max_depth: u8,
    dist: Vec<u8>,
    moveset: Vec<Move>,
}

impl Pruner for SBPruner {
    fn new() -> Self {
        let size = 24usize.pow(3) * 24usize.pow(2);
        let max_depth = 7;
        let moveset = vec![R, R2, R3, U, U2, U3, M, M2, M3, Rw, Rw2, Rw3];
        let mut dist = Self::init(size, Self::encode, &moveset, max_depth);
        dist[Self::encode(&CubieCube::default())] = 0;
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
                Corner::DFR => c1 = i * 3 + cube.co[i] as usize,
                Corner::DRB => c2 = i * 3 + cube.co[i] as usize,
                _ => {}
            }
        }
        let enc_c = c1 * 24 + c2;
        let mut e1 = 0;
        let mut e2 = 0;
        let mut e3 = 0;
        for i in 0..12 {
            match cube.ep[i] {
                Edge::DR => e1 = i * 2 + cube.eo[i] as usize,
                Edge::FR => e2 = i * 2 + cube.eo[i] as usize,
                Edge::BR => e3 = i * 2 + cube.eo[i] as usize,
                _ => {}
            }
        }
        let enc_e = e1 * (24 * 24) + e2 * (24) + e3;
        enc_e * (24 * 24) + enc_c
    }

    fn query(&self, cube: &CubieCube) -> u8 {
        let d = self.dist[SBPruner::encode(cube)];
        if d == 255 {
            return self.max_depth + 1;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::super::fb::FBSolver;
    use super::SBSolver;
    use crate::{
        cubie::CubieCube,
        moves::{Formula, Move::*},
        solver::roux::SolverBase,
    };

    #[test]
    fn test_sb() {
        let cc = CubieCube::default();
        let _f = Formula { moves: vec![L2] };
        let _f = Formula::scramble();
        println!("Scramble: {:?}", _f);
        let cc = cc.apply_formula(&_f);
        let mut fb = FBSolver::new(cc);
        let _f = fb.solve();
        assert!(fb.is_solved());
        println!("First Block Solution: {:?}", _f);
        let mut sb = SBSolver::new(fb.cube);
        let _s = sb.solve();
        println!("Second Block Solution: {:?}", _s);
        assert!(sb.is_solved());
    }
}
