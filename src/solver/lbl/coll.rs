use crate::cubie::Corner::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// COLLSolver for LBL(Layer by Layer) method, i.e, solve corner orientation of last layer's four corners(URF, UFL, ULB, UBR).
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
///     println!(
///         "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}",
///         moves, _cs, _bs, _ms, _eos, _cos
///     );
/// }
/// ```
pub struct COLLSolver {
    pub cube: CubieCube,
}

impl COLLSolver {
    /// Solve the corner orientation of last layer corners(URF, UFL, ULB, UBR).
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        match self.recognise() {
            0 => {
                return solution;
            }
            3 => {
                for i in 0..4 {
                    let _cube = self.cube;
                    let mut u_put = get_put_move(i, U);
                    self.cube = self.cube.apply_moves(&u_put);
                    for mut _s in [vec![R, U, R3, U, R, U2, R3], vec![R, U2, R3, U3, R, U3, R3]] {
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&_s);
                        if self.is_solved() {
                            solution.append(&mut u_put);
                            solution.append(&mut _s);
                            return solution;
                        }
                        self.cube = _cube;
                    }
                    self.cube = _cube;
                }
            }
            _ => {
                for i in 0..4 {
                    let u_cube = self.cube;
                    let mut u_put = get_put_move(i, U);
                    self.cube = self.cube.apply_moves(&u_put);
                    let mut _s = vec![R, U, R3, U, R, U2, R3];
                    self.cube = self.cube.apply_moves(&_s);
                    if self.recognise() == 3 {
                        solution.append(&mut u_put);
                        solution.append(&mut _s);
                        let mut _s2 = self.solve();
                        solution.append(&mut _s2);
                        return solution;
                    }
                    self.cube = u_cube;
                }
            }
        }
        solution
    }

    fn recognise(&self) -> u8 {
        let u_corners = get_corners_u(&self.cube);
        u_corners
            .iter()
            .fold(0, |acc, c| if c.2 != 0 { acc + 1 } else { acc + 0 })
    }

    /// Check if the last layer's corner orientation is solved.
    pub fn is_solved(&self) -> bool {
        self.recognise() == 0
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
            bottom::BottomCornerSolver, coll::COLLSolver, cross::CrossSolver, eoll::EOLLSolver,
            middle::MiddleEdgeSolver,
        },
    };

    #[test]
    fn test_coll() {
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
        println!(
            "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}, {:?}, {:?}",
            moves, _cs, _bs, _ms, _eos, _cos
        );
    }
}
