use std::str::FromStr;

use crate::{
    cubie::{
        CubieCube,
        Edge::{self, *},
    },
    moves::Move::{self, *},
    solver::lbl::{daisy::DaisySolver, get_put_move},
};

/// CrossSolver for LBL(Layer by Layer) method, i.e, solve bottom cross(four edges: DR, DF, DL, DB).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::scramble;
/// use rcuber::solver::lbl::cross::CrossSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = scramble();
///     let cc = cc.apply_moves(&moves);
///     let mut solver = CrossSolver::new(cc, true);
///     let solution = solver.solve();
///     assert!(solver.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
/// }
/// ```
#[derive(Default)]
pub struct CrossSolver {
    pub cube: CubieCube,
    pub daisy: bool,
}

impl CrossSolver {
    pub fn new(cube: CubieCube, daisy: bool) -> Self {
        Self {
            cube: cube,
            daisy: daisy,
        }
    }

    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        if self.daisy {
            let mut daisy = DaisySolver { cube: self.cube };
            let mut daisy_solution = daisy.solve();
            assert!(daisy.is_solved());
            self.cube = daisy.cube;
            solution.append(&mut daisy_solution);
            for edge in [DR, DF, DL, DB] {
                for i in 0..4 {
                    if self.cube.ep[i] == edge {
                        let uedge = format!("{:?}", edge);
                        let face = &uedge[1..2];
                        let uedge = format!("U{}", face);
                        let uedge = Edge::try_from(uedge.as_str()).unwrap();
                        let u = (4 + uedge as usize - i) % 4;
                        let mut u_put = get_put_move(u, U);
                        self.cube = self.cube.apply_moves(&u_put);
                        let face_move = Move::from_str(&format!("{}2", face)).unwrap();
                        self.cube = self.cube.apply_move(face_move);
                        solution.append(&mut u_put);
                        solution.push(face_move);
                        break;
                    }
                }
            }
        }
        solution
    }

    /// Check if the cross is solved.
    pub fn is_solved(&self) -> bool {
        let d_edges = self.cube.get_edges_d();
        let mut solved = 0;
        for edge in d_edges {
            match edge {
                (DR, 4, 0) | (DF, 5, 0) | (DL, 6, 0) | (DB, 7, 0) => solved += 1,
                _ => {}
            };
        }
        solved == 4
    }
}

#[cfg(test)]
mod test {
    use crate::{cubie::CubieCube, scramble, solver::lbl::cross::CrossSolver};

    #[test]
    fn test_cross() {
        let cc = CubieCube::default();
        let moves = scramble();
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver::new(cc, true);
        let _cs = cross.solve();
        assert!(cross.is_solved());
    }
}
