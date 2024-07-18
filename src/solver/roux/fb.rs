use std::collections::{HashMap, HashSet};

use crate::{
    cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE},
    moves::Move::{self, *},
};

use super::get_available_move;

/// FBSolver for solve Roux's First Block(a 1x2x3 block at left bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
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
    min_depth: i32,
    max_depth: i32,
    solution: Vec<Move>,
    pruner: FBPruner,
    moveset: Vec<Move>,
    next_moves: HashMap<Move, Vec<Move>>,
}

impl FBSolver {
    pub fn new(cube: CubieCube) -> Self {
        let moveset = vec![
            R, R2, R3, L, L2, L3, U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, M, M2, M3, Rw, Rw2,
            Rw3,
        ];
        let mut next_moves = HashMap::new();
        for m in moveset.clone() {
            next_moves.insert(m, get_available_move(m, &moveset));
        }
        let pruner = FBPruner::new();
        Self {
            cube,
            min_depth: 0,
            max_depth: 9,
            solution: Vec::new(),
            pruner,
            moveset,
            next_moves,
        }
    }

    pub fn is_solved(&self) -> bool {
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
        seen_encodings.insert(FBPruner::encode(cube));

        for m in available_moves.iter() {
            let new_cube = cube.apply_move(*m);
            let enc = FBPruner::encode(&new_cube);
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
struct FBPruner {
    max_depth: u8,
    dist: Vec<u8>,
}

impl FBPruner {
    pub fn new() -> Self {
        let size = 24usize.pow(3) * 24usize.pow(2);
        let solved_states = vec![CubieCube::default()];
        let max_depth = 4;
        let moves = vec![
            R, R2, R3, L, L2, L3, U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, M, M2, M3, Rw, Rw2,
            Rw3,
        ];

        let mut dist: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            dist.push(255);
        }
        for state in solved_states.iter() {
            dist[FBPruner::encode(state)] = 0;
        }
        let mut frontier = solved_states.clone();
        for i in 0..max_depth {
            let mut new_frontier = Vec::new();
            for state in frontier {
                for m in moves.iter() {
                    let mut new_state = state.clone();
                    new_state.multiply_moves(&vec![*m]);
                    // let new_state = state.apply_move(*m);
                    let idx = FBPruner::encode(&new_state);
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
