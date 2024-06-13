use crate::cubie::Corner::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// CPLLSolver for LBL(Layer by Layer) method, i.e, solve corner permutation of last layer's four corners(URF, UFL, ULB, UBR).
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
///     assert!(coll.is_solved());
///     let mut cpll = CPLLSolver { cube: coll.cube };
///     let _cps = cpll.solve();
///     assert!(cpll.is_solved());
///     println!(
///         "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
///         moves, _cs, _bs, _ms, _eos, _cos, _cps
///     );
/// }
/// ```
pub struct CPLLSolver {
    pub cube: CubieCube,
}

impl CPLLSolver {
    /// Solve the corner permutation of last layer corners(URF, UFL, ULB, UBR).
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        for i in 0..4 {
            let u_cube = self.cube;
            let u_put = get_put_move(i, U);
            self.cube = self.cube.apply_moves(&u_put);
            if self.is_solved() {
                return u_put;
            }
            self.cube = u_cube;
        }

        for i in 0..4 {
            let u_cube = self.cube;
            let mut u_put = get_put_move(i, U);
            self.cube = self.cube.apply_moves(&u_put);
            let mut _s = vec![R, U, R3, F3, R, U, R3, U3, R3, F, R2, U3, R3];
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
                }
                self.cube = u2_cube;
            }
            for j in 0..4 {
                let uu_cube = self.cube;
                let mut uu_put = get_put_move(j, U);
                self.cube = self.cube.apply_moves(&uu_put);
                let mut _s2 = vec![R, U, R3, F3, R, U, R3, U3, R3, F, R2, U3, R3];
                self.cube = self.cube.apply_moves(&_s2);
                for k in 0..4 {
                    let u2_cube = self.cube;
                    let mut u2_put = get_put_move(k, U);
                    self.cube = self.cube.apply_moves(&u2_put);
                    if self.is_solved() {
                        solution.append(&mut u_put);
                        solution.append(&mut _s);
                        solution.append(&mut uu_put);
                        solution.append(&mut _s2);
                        solution.append(&mut u2_put);
                        return solution;
                    }
                    self.cube = u2_cube;
                }
                self.cube = uu_cube;
            }
            self.cube = u_cube;
        }
        solution
    }

    /// Check if the last layer's corner permutation is solved.
    pub fn is_solved(&self) -> bool {
        let corners_u = get_corners_u(&self.cube);
        let mut solved = 0;
        for corner in corners_u {
            match corner {
                (URF, 0, 0) | (UFL, 1, 0) | (ULB, 2, 0) | (UBR, 3, 0) => solved += 1,
                _ => {}
            }
        }
        solved == 4
    }
}

pub fn get_corners_u(cc: &CubieCube) -> Vec<(Corner, u8, u8)> {
    let mut corners = Vec::new();
    for corner in [URF, UFL, ULB, UBR] {
        for i in 0..8 {
            if cc.cp[i] == corner {
                corners.push((corner, i as u8, cc.co[i]));
            }
        }
    }
    corners
}

#[cfg(test)]
mod tests {
    use crate::{
        cubie::CubieCube,
        moves::optimise_moves,
        scramble,
        solver::lbl::{
            bottom::BottomCornerSolver, coll::COLLSolver, cpll::CPLLSolver, cross::CrossSolver,
            eoll::EOLLSolver, middle::MiddleEdgeSolver,
        },
    };

    #[test]
    fn test_cpll() {
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
        assert!(coll.is_solved());
        let mut cpll = CPLLSolver { cube: coll.cube };
        let _cps = cpll.solve();
        assert!(cpll.is_solved());
        println!(
            "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            moves, _cs, _bs, _ms, _eos, _cos, _cps
        );
    }
}
