use std::collections::{HashMap, HashSet};

use crate::cubie::{CubieCube, SOLVED_CUBIE_CUBE};
use crate::facelet::Color;
use crate::moves::Move::{self, *};

use super::get_available_move;

/// LSE(Last Six Edges) is the fourth step of the Roux method.
/// LSE typically split to 3 substeps (called 4a, 4b, and 4c).
/// 4a: Edge orientation (EO)
/// 4b: Solve upper left and upper right edges (UL/UR)
/// 4c: Solve middle edges
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
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
    min_depth: i32,
    max_depth: i32,
    solution: Vec<Move>,
    pruner: LSEPruner,
    moveset: Vec<Move>,
    next_moves: HashMap<Move, Vec<Move>>,
}

impl LSESolver {
    pub fn new(cube: CubieCube) -> Self {
        let moveset = vec![U, U2, U3, M, M2, M3];
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }
        let pruner = LSEPruner::new();
        Self {
            cube,
            min_depth: 0,
            max_depth: 20,
            solution: Vec::new(),
            pruner,
            moveset,
            next_moves,
        }
    }

    /// Check if Cube is solved.
    pub fn is_solved(&self) -> bool {
        self.cube == SOLVED_CUBIE_CUBE
    }

    pub fn solve(&mut self) -> Vec<Move> {
        for i in self.min_depth..=self.max_depth {
            self.solution = self.solve_depth(self.min_depth, i);
            if self.solution.len() > 0 {
                break;
            }
        }
        self.cube = self.cube.apply_moves(&self.solution);
        self.solution.clone()
    }

    fn solve_depth(&mut self, min_depth: i32, max_depth: i32) -> Vec<Move> {
        self.min_depth = min_depth;
        self.max_depth = max_depth;
        let cube = self.cube.clone();
        let _r = self.search(&cube, 0, &Vec::new());
        self.solution.clone()
    }

    fn cube_is_solved(&self, cube: &CubieCube) -> bool {
        self.pruner.query(cube) == 0
    }

    fn search(&mut self, cube: &CubieCube, depth: i32, solution: &Vec<Move>) -> bool {
        if self.cube_is_solved(cube) {
            self.solution = solution.clone();
            return true;
        } else {
            if depth >= self.max_depth {
                return false;
            };

            let d = self.pruner.query(cube);
            if d as i32 + depth > self.max_depth {
                return false;
            } else {
                return self.expand(cube, depth, solution);
            }
        }
    }

    fn expand(&mut self, cube: &CubieCube, depth: i32, solution: &Vec<Move>) -> bool {
        let mut solution = solution.clone();
        let available_moves = match solution.len() > 0 {
            true => self
                .next_moves
                .get(&solution[solution.len() - 1])
                .unwrap()
                .clone(),
            false => self.moveset.clone(),
        };
        let mut seen_encodings = HashSet::new();
        seen_encodings.insert(LSEPruner::encode(cube));

        for m in available_moves.iter() {
            let new_cube = cube.apply_move(*m);
            let enc = LSEPruner::encode(&new_cube);
            if seen_encodings.len() == 0 || !seen_encodings.contains(&enc) {
                seen_encodings.insert(enc);
                solution.push(*m);
                let st = self.search(&new_cube, depth + 1, &solution);
                solution.pop();
                if st {
                    return st;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct LSEPruner {
    max_depth: u8,
    dist: Vec<u8>,
}

impl LSEPruner {
    pub fn new() -> Self {
        let size = 12usize.pow(6) * 4 * 4 / 2;
        let solved_states = vec![CubieCube::default()];
        let max_depth = 6;
        let moves = vec![U, U2, U3, M, M2, M3];

        let mut dist: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            dist.push(255);
        }
        for state in solved_states.iter() {
            dist[LSEPruner::encode(state)] = 0;
        }
        let mut frontier = solved_states.clone();
        for i in 0..max_depth {
            let mut new_frontier = Vec::new();
            for state in frontier {
                for m in moves.iter() {
                    let new_state = state.apply_move(*m);
                    let idx = LSEPruner::encode(&new_state);
                    if dist[idx] == 255 {
                        dist[idx] = i as u8 + 1;
                        new_frontier.push(new_state);
                    }
                }
            }
            frontier = new_frontier;
        }
        Self { max_depth, dist }
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
        let enc_c = if cube.center[0] == Color::U {0} else {1};
        enc_e * 4 + enc_c * 4 + cube.cp[0] as usize
    }

    fn query(&self, cube: &CubieCube) -> u8 {
        let d = self.dist[LSEPruner::encode(cube)];
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
        solver::roux::{cmll::CMLLSolver, SBSolver},
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
