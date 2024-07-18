//! # min2phase
//! ## Two-phase algorithm
//! See [Kociemba's page](http://kociemba.org/cube.htm)
//! ## `min2phase` Improvements compared with conventional two-phase algorithm
//!    Conventional two-phase algorithm only find (sub-)optimal solutions to <U,R2,F2,D,L2,B2>. However, If we are able to find more phase1 solutions within a limited depth, the probability of finding a short solution will be increased.
//!    - Try different axes: The target of phase1 can be either <U,R2,F2,D,L2,B2>, <U2,R,F2,D2,L,B2>, or <U2,R2,F,D2,L2,B>.
//!    - Try the inverse of the state: We will try to solve the inverse state simultaneously to find more phase1 solutions.
//!    - Try pre-scramble: We can also use pre-scramble technique (which is widely used in fewest-move challenge) to find more phase1 solutions. If PreMoves \* Scramble \* Phase1 \* Phase2 = Solved, then Scramble \* (Phase1 \* Phase2 \* PreMoves) = Solved, Solution = Phase1 \* Phase2 \* PreMoves.

/// Module for represent a cube on the cubie level(array model).
pub mod arraycube;
/// Module containing 3x3 cube constants.
pub mod constants;
/// Module for represent a cube on the coordinate level.
pub mod coord;
/// Module for represent a cube on the facelet level.
/// Impl `From<&ArrayCube>` for CubieCube.
pub mod cubie;
/// Module for min2phase solver.
pub mod solver;
/// Module for data tables.
pub mod tables;
/// Module for misc utils and tables.
pub mod utils;

use crate::moves::Formula;
use crate::{cubie::CubieCube, facelet::FaceCube};
use solver::Solver;

/// Min2PhaseSolver for solve a cube use min2phase method.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::min2phase::Min2PhaseSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let formula = Formula::scramble();
///     let cc = cc.apply_formula(&formula);
///     let mut solver = Min2PhaseSolver{cube: cc};
///     assert!(!solver.is_solved());
///     let solution = solver.solve();
///     assert!(solver.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}", formula, solution);
/// }
/// ```
/// For find a more optimal solution, use `min2phase::solver::solver::next`.
///
#[derive(Debug)]
pub struct Min2PhaseSolver {
    pub cube: CubieCube,
}

impl Min2PhaseSolver {
    pub fn solve(&mut self) -> Formula {
        let mut solver = Solver::default();
        let s = solver
            .solve(
                FaceCube::try_from(&self.cube).unwrap().to_string().as_str(),
                21,
                1000000,
                0,
                0x0,
            )
            .unwrap();
        self.cube = self.cube.apply_formula(&s);
        s
    }
    pub fn is_solved(&self) -> bool {
        self.cube == CubieCube::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::cubie::CubieCube;
    use crate::moves::Formula;
    use crate::solver::min2phase::Min2PhaseSolver;

    #[test]
    fn test_solver() {
        let cc = CubieCube::default();
        let formula = Formula::scramble();
        let cc = cc.apply_formula(&formula);
        let mut solver = Min2PhaseSolver { cube: cc };
        assert!(!solver.is_solved());
        let solution = solver.solve();
        assert!(solver.is_solved());
        println!("Scramble: {:?}\nSolution: {:?}", formula, solution);
    }
}
