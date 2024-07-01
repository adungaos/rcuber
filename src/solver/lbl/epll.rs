use crate::cubie::Edge::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// EPLLSolver for LBL(Layer by Layer) method, i.e, solve edge permutation of last layer's four edges(UR, UF, UL, UB).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::scramble;
/// use rcuber::moves::optimise_moves;
/// use rcuber::solver::lbl::cross::CrossSolver;
/// use rcuber::solver::lbl::bottom::BottomCornerSolver;
/// use rcuber::solver::lbl::middle::MiddleEdgeSolver;
/// use rcuber::solver::lbl::eoll::EOLLSolver;
/// use rcuber::solver::lbl::coll::COLLSolver;
/// use rcuber::solver::lbl::cpll::CPLLSolver;
/// use rcuber::solver::lbl::epll::EPLLSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = scramble();
///     let cc = cc.apply_moves(&moves);
///     let mut cross = CrossSolver::new(cc, true);
///     let _cs = cross.solve();
///     let mut bottom = BottomCornerSolver { cube: cross.cube };
///     let _bs = bottom.solve();
///     let _bs = optimise_moves(&_bs);
///     let mut middle = MiddleEdgeSolver { cube: bottom.cube };
///     let _ms = middle.solve();
///     let _ms = optimise_moves(&_ms);
///     let mut eoll = EOLLSolver { cube: middle.cube };
///     let _eos = eoll.solve();
///     let mut coll = COLLSolver { cube: eoll.cube };
///     let _cos = coll.solve();
///     let mut cpll = CPLLSolver { cube: coll.cube };
///     let _cps = cpll.solve();
///     assert!(cpll.is_solved());
///     let mut epll = EPLLSolver { cube: cpll.cube };
///     let _eps = epll.solve();
///     assert!(epll.is_solved());
///     println!(
///         "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
///         moves, _cs, _bs, _ms, _eos, _cos, _cps, _eps
///     );
/// }
/// ```
pub struct EPLLSolver {
    pub cube: CubieCube,
}

impl EPLLSolver {
    /// Solve the edge permutation of last layer edges(UR, UF, UL, UB).
    pub fn solve(&mut self) -> Vec<Move> {
        for i in 0..4 {
            let u_put = get_put_move(i, U);
            let u_cube = self.cube;
            self.cube = self.cube.apply_moves(&u_put);
            if self.is_solved() {
                return u_put;
            }
            self.cube = u_cube;
        }
        let mut solution = Vec::new();
        for i in 0..4 {
            let u_cube = self.cube;
            let mut u_put = get_put_move(i, U);
            self.cube = self.cube.apply_moves(&u_put);
            for mut _s in [
                vec![R, U2, R3, U3, R, U3, R3, U3, R3, U2, R, U, R3, U, R],
                vec![R3, U2, R, U, R3, U, R, U, R, U2, R3, U3, R, U3, R3],
            ] {
                let s_cube = self.cube;
                self.cube = self.cube.apply_moves(&_s);
                for j in 0..4 {
                    let u2_cube = self.cube;
                    let mut u2_put = get_put_move(j, U);
                    self.cube = self.cube.apply_moves(&u2_put);
                    if self.is_solved() {
                        solution.append(&mut u_put);
                        solution.append(&mut _s);
                        solution.append(&mut u2_put);
                        return solution;
                    } else {
                        for ii in 0..4 {
                            let uu_cube = self.cube;
                            let mut uu_put = get_put_move(ii, U);
                            self.cube = self.cube.apply_moves(&uu_put);
                            for mut _s2 in [
                                vec![R, U2, R3, U3, R, U3, R3, U3, R3, U2, R, U, R3, U, R],
                                vec![R3, U2, R, U, R3, U, R, U, R, U2, R3, U3, R, U3, R3],
                            ] {
                                let s2_cube = self.cube;
                                self.cube = self.cube.apply_moves(&_s2);
                                for jj in 0..4 {
                                    let uu2_cube = self.cube;
                                    let mut uu2_put = get_put_move(jj, U);
                                    if self.is_solved() {
                                        solution.append(&mut u_put);
                                        solution.append(&mut _s);
                                        solution.append(&mut u2_put);
                                        solution.append(&mut uu_put);
                                        solution.append(&mut _s2);
                                        solution.append(&mut uu2_put);
                                        return solution;
                                    }
                                    self.cube = uu2_cube;
                                }
                                self.cube = s2_cube;
                            }
                            self.cube = uu_cube;
                        }
                    }
                    self.cube = u2_cube;
                }
                self.cube = s_cube;
            }
            self.cube = u_cube;
        }
        solution
    }

    /// Check if the last layer's edge permutation is solved.
    pub fn is_solved(&self) -> bool {
        let mut cc = CubieCube::default();
        for i in 0..4 {
            let y_put = get_put_move(i, y);
            cc = cc.apply_moves(&y_put);
            if cc == self.cube {
                return true;
            }
        }
        false
    }
}

pub fn get_edges_u(cc: &CubieCube) -> Vec<(Edge, u8, u8)> {
    let mut edges = Vec::new();
    for edge in [UR, UF, UL, UB] {
        for i in 0..8 {
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
        moves::optimise_moves,
        scramble,
        solver::lbl::{
            bottom::BottomCornerSolver, coll::COLLSolver, cpll::CPLLSolver, cross::CrossSolver,
            eoll::EOLLSolver, epll::EPLLSolver, middle::MiddleEdgeSolver,
        },
    };

    #[test]
    fn test_epll() {
        let cc = CubieCube::default();
        let moves = scramble();
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver::new(cc, true);
        let _cs = cross.solve();
        let mut bottom = BottomCornerSolver { cube: cross.cube };
        let _bs = bottom.solve();
        let _bs = optimise_moves(&_bs);
        let mut middle = MiddleEdgeSolver { cube: bottom.cube };
        let _ms = middle.solve();
        let _ms = optimise_moves(&_ms);
        let mut eoll = EOLLSolver { cube: middle.cube };
        let _eos = eoll.solve();
        let mut coll = COLLSolver { cube: eoll.cube };
        let _cos = coll.solve();
        let mut cpll = CPLLSolver { cube: coll.cube };
        let _cps = cpll.solve();
        assert!(cpll.is_solved());
        let mut epll = EPLLSolver { cube: cpll.cube };
        let _eps = epll.solve();
        assert!(epll.is_solved());
        println!(
            "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            moves, _cs, _bs, _ms, _eos, _cos, _cps, _eps
        );
    }
}
