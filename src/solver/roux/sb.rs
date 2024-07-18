use std::collections::{HashMap, HashSet};

use crate::{
    cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE},
    moves::Move::{self, *},
};

use super::get_available_move;

/// SBSolver for solve Roux's Second Block(a 1x2x3 block at right bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
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
    min_depth: i32,
    max_depth: i32,
    solution: Vec<Move>,
    pruner: SBPruner,
    moveset: Vec<Move>,
    next_moves: HashMap<Move, Vec<Move>>,
}

impl SBSolver {
    pub fn new(cube: CubieCube) -> Self {
        let moveset = vec![R, R2, R3, U, U2, U3, M, M2, M3, Rw, Rw2, Rw3];
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }
        let pruner = SBPruner::new();
        Self {
            cube,
            min_depth: 0,
            max_depth: 14,
            solution: Vec::new(),
            pruner,
            moveset,
            next_moves,
        }
    }

    pub fn is_solved(&self) -> bool {
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
        seen_encodings.insert(SBPruner::encode(cube));

        for m in available_moves.iter() {
            let new_cube = cube.apply_move(*m);
            let enc = SBPruner::encode(&new_cube);
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
struct SBPruner {
    max_depth: u8,
    dist: Vec<u8>,
}

impl SBPruner {
    fn new() -> Self {
        let size = 24usize.pow(3) * 24usize.pow(2);
        let solved_states = vec![CubieCube::default()];
        let max_depth = 7;
        let moves = vec![R, R2, R3, U, U2, U3, M, M2, M3, Rw, Rw2, Rw3];

        let mut dist: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            dist.push(255);
        }
        for state in solved_states.iter() {
            dist[SBPruner::encode(state)] = 0;
        }
        let mut frontier = solved_states.clone();
        for i in 0..max_depth {
            let mut new_frontier = Vec::new();
            for state in frontier {
                for m in moves.iter() {
                    let mut new_state = state.clone();
                    new_state.multiply_move(*m);
                    // let new_state = state.apply_move(*m);
                    let idx = SBPruner::encode(&new_state);
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
    use super::SBSolver;
    use super::super::fb::FBSolver;
    use crate::{
        cubie::CubieCube,
        moves::{Formula, Move::*},
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
