use std::collections::HashMap;

use crate::cubie::{CubieCube, SOLVED_CUBIE_CUBE};
use crate::facelet::Color;
use crate::moves::Move::{self, *};

use super::{get_available_move, Pruner, SolverBase, SolverConfig};

/// LSE(Last Six Edges) is the fourth step of the Roux method.
/// LSE typically split to 3 substeps (called 4a, 4b, and 4c).
/// 4a: Edge orientation (EO)
/// 4b: Solve upper left and upper right edges (UL/UR)
/// 4c: Solve middle edges
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::SolverBase;
/// use rcuber::solver::roux::fb::FBSolver;
/// use rcuber::solver::roux::sb::SBSolver;
/// use rcuber::solver::roux::cmll::CMLLSolver;
/// use rcuber::solver::roux::lse::LSESolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let f = Formula::scramble();
///     let cc = cc.apply_formula(&f);
///     let mut fb = FBSolver::new(cc);
///     let _fb = fb.solve();
///     assert!(fb.is_solved());
///     let mut sb = SBSolver::new(fb.cube);
///     let _sb = sb.solve();
///     assert!(sb.is_solved());
///     let mut cmll = CMLLSolver::new(sb.cube);
///     let _cmll = cmll.solve();
///     assert!(cmll.is_solved());
///     let mut lse = LSESolver::new(cmll.cube);
///     let _lse = lse.solve();
///     assert!(lse.is_solved());
///     println!("
///         Scramble: {:?}\nFirst Block: {:?}\nSecond Block: {:?}\nCMLL: {:?}\nLSE: {:?}",
///         f.moves, _fb, _sb, _cmll, _lse
///     );
/// }
/// ```

#[derive(Debug)]
pub struct LSESolver {
    pub cube: CubieCube,
    config: SolverConfig,
    pruner: LSEPruner,
}

impl SolverBase for LSESolver {
    fn new(cube: CubieCube) -> Self {
        let pruner = LSEPruner::new();
        let moveset = pruner.moveset.clone();
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }

        let config = SolverConfig {
            min_depth: 0,
            max_depth: 20,
            moveset,
            next_moves,
        };

        Self {
            cube,
            config,
            pruner,
        }
    }

    /// Check if Cube is solved.
    fn is_solved(&self) -> bool {
        self.cube == SOLVED_CUBIE_CUBE
    }

    fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        for i in self.config.min_depth..=self.config.max_depth {
            let min_depth = self.config.min_depth;
            solution = Self::solve_depth(&self.cube, min_depth, i, &mut self.config, &self.pruner, LSEPruner::encode);
            if solution.len() > 0 {
                break;
            }
        }
        self.cube = self.cube.apply_moves(&solution);
        solution
    }
}

#[derive(Debug)]
struct LSEPruner {
    max_depth: u8,
    dist: Vec<u8>,
    moveset: Vec<Move>,
}

impl Pruner for LSEPruner {
    fn new() -> Self {
        let size = 12usize.pow(6) * 4 * 4 / 2;
        let max_depth = 6;
        let moveset = vec![U, U2, U3, M, M2, M3];
        let mut dist = Self::init(size, LSEPruner::encode, &moveset, max_depth);
        dist[LSEPruner::encode(&CubieCube::default())] = 0;
        Self {
            max_depth,
            dist,
            moveset,
        }
    }

    fn encode(cube: &CubieCube) -> usize {
        let mut enc = [0; 6];
        let edge_encode = [0, 1, 2, 3, 255, 4, 255, 5, 255, 255, 255, 255];

        for i in 0..12 {
            let idx = edge_encode[cube.ep[i] as usize];
            if idx != 255 {
                enc[idx] = edge_encode[i] * 2 + cube.eo[i] as usize;
            }
        }
        let mut enc_e = 0;
        for i in 0..6 {
            enc_e = enc_e * 12 + enc[i];
        }
        let enc_c = if cube.center[0] == Color::U { 0 } else { 1 };
        enc_e * 4 + enc_c * 4 + cube.cp[0] as usize
    }

    fn query(&self, cube: &CubieCube) -> u8 {
        let d = self.dist[Self::encode(cube)];
        if d == 255 {
            return self.max_depth + 1;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::{super::fb::FBSolver, LSESolver};
    use crate::{
        cubie::CubieCube,
        moves::Formula,
        solver::roux::{cmll::CMLLSolver, SBSolver, SolverBase},
    };

    #[test]
    fn test_lse() {
        let cc = CubieCube::default();
        let f = Formula::scramble();
        let cc = cc.apply_formula(&f);
        let mut fb = FBSolver::new(cc);
        let _fb = fb.solve();
        assert!(fb.is_solved());
        let mut sb = SBSolver::new(fb.cube);
        let _sb = sb.solve();
        assert!(sb.is_solved());
        let mut cmll = CMLLSolver::new(sb.cube);
        let _cmll = cmll.solve();
        assert!(cmll.is_solved());
        let mut lse = LSESolver::new(cmll.cube);
        let _lse = lse.solve();
        assert!(lse.is_solved());
        println!(
            "Scramble: {:?}\nFirst Block: {:?}\nSecond Block: {:?}\nCMLL: {:?}\nLSE: {:?}",
            f.moves, _fb, _sb, _cmll, _lse
        );
    }
}
