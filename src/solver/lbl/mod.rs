
/// Module for DaisySolver of LBL(Layer by Layer) method(optional).
pub mod daisy;
/// Module for CrossSolver of LBL(Layer by Layer) method(step 1).
pub mod cross;
/// Module for BottomCornerSolver of LBL(Layer by Layer) method(step 2).
pub mod bottom;
/// Module for MiddleEdgeSolver of LBL(Layer by Layer) method(step 3).
pub mod middle;
/// Module for EOLLSolver(Edge Orientation of Last Layer) of LBL(Layer by Layer) method(step 4).
pub mod eoll;
/// Module for COLLSolver(Corner Orientation of Last Layer) of LBL(Layer by Layer) method(step 5).
pub mod coll;
/// Module for CPLLSolver(Corner Permutation of Last Layer) of LBL(Layer by Layer) method(step 6).
pub mod cpll;
/// Module for EPLLSolver(Edge Permutation of Last Layer) of LBL(Layer by Layer) method(step 7).
pub mod epll;


use crate::facelet::Color;
use crate::{cubie::CubieCube, moves::Move};

pub use daisy::DaisySolver;
pub use cross::CrossSolver;
pub use bottom::BottomCornerSolver;
pub use middle::MiddleEdgeSolver;
pub use eoll::EOLLSolver;
pub use coll::COLLSolver;
pub use cpll::CPLLSolver;
pub use epll::EPLLSolver;

/// LBLSolver for solve a cube use LBL(Layer by Layer) method.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::scramble;
/// use rcuber::solver::lbl::LBLSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = scramble();
///     let cc = cc.apply_moves(&moves);
///     let mut solver = LBLSolver{cube: cc};
///     let solution = solver.solve();
///     assert!(solver.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
/// }
/// ```
pub struct LBLSolver{
    pub cube: CubieCube,
}

impl LBLSolver {
    /// Solve the cube.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let mut cross = CrossSolver::new(self.cube, true);
        let mut cs = cross.solve();
        solution.append(&mut cs);

        solution
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