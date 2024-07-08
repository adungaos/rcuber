use crate::cubie::Edge::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// EOLLSolver for LBL(Layer by Layer) method, i.e, solve edge orientation of last layer's four edges(UR, UF, UL, UB).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::lbl::cross::CrossSolver;
/// use rcuber::solver::lbl::bottom::BottomCornerSolver;
/// use rcuber::solver::lbl::middle::MiddleEdgeSolver;
/// use rcuber::solver::lbl::eoll::EOLLSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = Formula::scramble();
///     let cc = cc.apply_formula(&moves);
///     let mut cross = CrossSolver::new(cc, true);
///     let _cs = cross.solve();
///     let mut bottom = BottomCornerSolver { cube: cross.cube };
///     let _bs = bottom.solve();
///     assert!(bottom.is_solved());
///     // let _bs = optimise_moves(&_bs);
///     let mut middle = MiddleEdgeSolver { cube: bottom.cube };
///     let _ms = middle.solve();
///     assert!(middle.is_solved());
///     // let _ms = optimise_moves(&_ms);
///     let mut eoll = EOLLSolver { cube: middle.cube };
///     let _eos = eoll.solve();
///     assert!(eoll.is_solved());
///     println!(
///         "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}",
///         moves, _cs, _bs, _ms, _eos
///     );
/// }
/// ```
pub struct EOLLSolver {
    pub cube: CubieCube,
}

impl EOLLSolver {
    /// Solve the edge orientation of last layer edges(UR, UF, UL, UB).
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let case = self.recognise();
        if case == 0 {
            return solution;
        } else if case == 4 {
            let solution = vec![F, R, U, R3, U3, R, U, R3, U3, F3, U3, F, R, U, R3, U3, F3];
            self.cube = self.cube.apply_moves(&solution);
            return solution;
        } else {
            for i in 0..4 {
                let u_cube = self.cube;
                let mut u_put = get_put_move(i, U);
                self.cube = self.cube.apply_moves(&u_put);
                for mut _s in [
                    vec![F, R, U, R3, U3, F3],
                    vec![F, R, U, R3, U3, F3, U3, F, R, U, R3, U3, F3],
                ] {
                    let s_cube = self.cube;
                    self.cube = self.cube.apply_moves(&_s);
                    if self.is_solved() {
                        solution.append(&mut u_put);
                        solution.append(&mut _s);
                        return solution;
                    }
                    self.cube = s_cube;
                }
                self.cube = u_cube;
            }
        }
        solution
    }

    fn recognise(&self) -> u8 {
        let edges_u = get_edges_u(&self.cube);
        edges_u.iter().fold(0, |acc, e| acc + e.2)
    }

    /// Check if the last layer's edge orientation is solved.
    pub fn is_solved(&self) -> bool {
        self.recognise() == 0
    }
}

pub fn get_edges_u(cc: &CubieCube) -> Vec<(Edge, u8, u8)> {
    let mut edges = Vec::new();
    for edge in [UR, UF, UL, UB] {
        for i in 0..12 {
            if cc.ep[i] == edge {
                edges.push((edge, i as u8, cc.eo[i]));
            }
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use crate::{
        cubie::CubieCube,
        moves::Formula,
        solver::lbl::{
            bottom::BottomCornerSolver, cross::CrossSolver, eoll::EOLLSolver,
            middle::MiddleEdgeSolver,
        },
    };

    #[test]
    fn test_eoll() {
        let cc = CubieCube::default();
        let moves =Formula::scramble();
        let cc = cc.apply_formula(&moves);
        let mut cross = CrossSolver::new(cc, true);
        let _cs = cross.solve();
        let mut bottom = BottomCornerSolver { cube: cross.cube };
        let _bs = bottom.solve();
        assert!(bottom.is_solved());
        // let _bs = optimise_moves(&_bs);
        let mut middle = MiddleEdgeSolver { cube: bottom.cube };
        let _ms = middle.solve();
        assert!(middle.is_solved());
        // let _ms = optimise_moves(&_ms);
        let mut eoll = EOLLSolver { cube: middle.cube };
        let _eos = eoll.solve();
        assert!(eoll.is_solved());
        println!(
            "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}",
            moves, _cs, _bs, _ms, _eos
        );
    }
}
