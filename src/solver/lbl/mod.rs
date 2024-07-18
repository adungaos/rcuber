//! # LBL(Layer-By-Layer)
//! `LBL` is a group of methods that solves the cube in layers.
//! In the basic, beginner LBL method, the solver finishes the layers one at a time: 
//! the first layer edges, then corners, then the second layer edges, and finally the last layer. 
//! This is a common method for new cubers to discover on their own.
//! In more advanced LBL methods, you solve layers more efficiently or solve two layers at once. 
//! For example, in the CFOP method, one solves the first two layers simultaneously by forming a cross of the first layer edges, and then filling in four pairs of a corner and an edge into the so-called slots.
//! With this method, the novice cuber truly completes each layer one after the other, using few algorithms (but taking perhaps over 100 moves). This is one of the most popular beginners' methods in existence.
//! # Steps
//! 0. ( Daisy )
//! 1. Bottom layer: Corners XG (Cross plus Corners)
//! 2. Middle Layer: Insert the 4 middle layer edges (each edge is inserted individually)
//! 3. 4-Look Last Layer, sometimes actually more like 8-looks by re-using algorithms
//! * orientation of edges
//! * orientation of corners
//! * permutation of corners
//! * permutation of edges

/// Module for BottomCornerSolver of LBL(Layer by Layer) method(step 2).
pub mod bottom;
/// Module for COLLSolver(Corner Orientation of Last Layer) of LBL(Layer by Layer) method(step 5).
pub mod coll;
/// Module for CPLLSolver(Corner Permutation of Last Layer) of LBL(Layer by Layer) method(step 6).
pub mod cpll;
/// Module for CrossSolver of LBL(Layer by Layer) method(step 1).
pub mod cross;
/// Module for DaisySolver of LBL(Layer by Layer) method(optional).
pub mod daisy;
/// Module for EOLLSolver(Edge Orientation of Last Layer) of LBL(Layer by Layer) method(step 4).
pub mod eoll;
/// Module for EPLLSolver(Edge Permutation of Last Layer) of LBL(Layer by Layer) method(step 7).
pub mod epll;
/// Module for MiddleEdgeSolver of LBL(Layer by Layer) method(step 3).
pub mod middle;

use crate::facelet::Color;
use crate::moves::Formula;
use crate::{cubie::CubieCube, moves::Move};

pub use bottom::BottomCornerSolver;
pub use coll::COLLSolver;
pub use cpll::CPLLSolver;
pub use cross::CrossSolver;
pub use daisy::DaisySolver;
pub use eoll::EOLLSolver;
pub use epll::EPLLSolver;
pub use middle::MiddleEdgeSolver;

/// LBLSolver for solve a cube use LBL(Layer by Layer) method.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::lbl::LBLSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = Formula::scramble();
///     let cc = cc.apply_formula(&moves);
///     let mut solver = LBLSolver{cube: cc};
///     let solution = solver.solve();
///     assert!(solver.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
/// }
/// ```
pub struct LBLSolver {
    pub cube: CubieCube,
}

impl LBLSolver {
    /// Solve the cube.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let mut cross = CrossSolver::new(self.cube, true);
        let mut _cs = cross.solve();
        assert!(cross.is_solved());
        self.cube = cross.cube;
        solution.append(&mut _cs);
        let mut bottom = BottomCornerSolver { cube: self.cube };
        let mut _bs = bottom.solve();
        assert!(bottom.is_solved());
        self.cube = bottom.cube;
        solution.append(&mut _bs);
        let mut middle = MiddleEdgeSolver { cube: self.cube };
        let mut _ms = middle.solve();
        assert!(middle.is_solved());
        self.cube = middle.cube;
        solution.append(&mut _ms);
        let mut eoll = EOLLSolver { cube: self.cube };
        let mut _eos = eoll.solve();
        assert!(eoll.is_solved());
        self.cube = eoll.cube;
        solution.append(&mut _eos);
        let mut coll = COLLSolver { cube: self.cube };
        let mut _cos = coll.solve();
        assert!(coll.is_solved());
        self.cube = coll.cube;
        solution.append(&mut _cos);
        let mut cpll = CPLLSolver { cube: self.cube };
        let mut _cps = cpll.solve();
        assert!(cpll.is_solved());
        self.cube = cpll.cube;
        solution.append(&mut _cps);
        let mut epll = EPLLSolver { cube: self.cube };
        let mut _eps = epll.solve();
        assert!(epll.is_solved());
        self.cube = epll.cube;
        solution.append(&mut _eps);
        Formula { moves: solution }.optimise().moves
    }

    pub fn is_solved(&self) -> bool {
        let cc = CubieCube::default();
        self.cube == cc
    }
}

pub fn get_move_face(step: Move) -> Color {
    let face = format!("{:?}", step);
    let face = face.as_bytes()[0];
    let face = char::from(face);
    let face = Color::try_from(face).unwrap();
    face
}

pub fn get_put_move(i: usize, step: Move) -> Vec<Move> {
    match i {
        1 => vec![step],
        2 => vec![step * 2],
        3 => vec![step * 3],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::{cubie::CubieCube, moves::Formula, solver::LBLSolver};

    #[test]
    fn test_lbl() {
        let cc = CubieCube::default();
        let moves = Formula::scramble();
        let cc = cc.apply_formula(&moves);
        let cc2 = cc.clone();
        let mut solver = LBLSolver { cube: cc };
        let solution = solver.solve();
        assert!(solver.is_solved());

        let cc2 = cc2.apply_moves(&solution);
        assert_eq!(cc2, CubieCube::default());
        println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
    }
}
