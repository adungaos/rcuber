/// Module for Roux's third step, solve the corners of the last layer without considering the M-slice.
pub mod cmll;
/// Module for Roux's first step, solve First Block.
pub mod fb;
/// Module for Roux's fourth step, solve LSE(Last Six Edges).
pub mod lse;
/// Module for Roux's second step, solve Second Block.
pub mod sb;

pub use cmll::CMLLSolver;
pub use fb::FBSolver;
pub use lse::LSESolver;
pub use sb::SBSolver;

use crate::{
    cubie::{CubieCube, SOLVED_CUBIE_CUBE},
    moves::Move::{self, *},
};

/// Roux is a Rubik's cube speedsolving method invented by Gilles Roux.
/// Roux is based on Blockbuilding and Corners First methods.
/// It is notable for its low movecount, lack of rotations, heavy use of M moves in the last step, and adaptability to One-Handed Solving.
/// # Steps
/// 1. Build a 1x2x3 Block anywhere on the cube.
/// 2. Build a second 1x2x3 block opposite of the first 1x2x3 block, without disrupting the first 1x2x3 block. After this step, there should be two 1x2x3 blocks: one on the lower left side, and one lower right side, leaving the U slice and M slice free to move.
/// Steps 1 and 2 are referred to as the First Two Blocks
/// 3. Simultaneously orient and permute the remaining four corners on the top layer (U-slice). If performed in one step, there are 42 algorithms. This set of algorithms is commonly referred to as CMLL.
/// 4. LSE, also called L6E, short for Last Six Edges.
/// 4a. Orient the 6 remaining edges using only M and U moves (UF, UB, UL, UR, DF, DB need to be oriented correctly).
/// 4b. Solve the UL and UR edges, preserving edge orientation. After this step, both the left and right side layers should be complete.
/// 4c. Solve the centers and edges in the M slice. This step is sometimes also called L4E or L4EP. see Last Six Edges.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::RouxSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let f = Formula::scramble();
///     let cc = cc.apply_formula(&f);
///     let mut roux = RouxSolver::new(cc);
///     let _roux = roux.solve();
///     assert!(roux.is_solved());
///     println!("Scramble: {:?}\nRoux Solution: {:?}", f.moves, _roux);
/// }
/// ```

#[derive(Debug)]
pub struct RouxSolver {
    pub cube: CubieCube,
}

impl RouxSolver {
    pub fn new(cube: CubieCube) -> Self {
        Self { cube }
    }

    /// Check if Cube is solved.
    pub fn is_solved(&self) -> bool {
        self.cube == SOLVED_CUBIE_CUBE
    }

    pub fn solve(&mut self) -> Vec<Move> {
        let mut result = Vec::new();
        let mut fb = FBSolver::new(self.cube);
        let mut _fb = fb.solve();
        assert!(fb.is_solved());
        self.cube = fb.cube;
        result.append(&mut _fb);
        let mut sb = SBSolver::new(self.cube);
        let mut _sb = sb.solve();
        assert!(sb.is_solved());
        self.cube = sb.cube;
        result.append(&mut _sb);
        let mut cmll = CMLLSolver::new(self.cube);
        let mut _cmll = cmll.solve();
        assert!(cmll.is_solved());
        self.cube = cmll.cube;
        result.append(&mut _cmll);
        let mut lse = LSESolver::new(self.cube);
        let mut _lse = lse.solve();
        assert!(lse.is_solved());
        self.cube = lse.cube;
        result.append(&mut _lse);
        assert!(self.is_solved());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::RouxSolver;
    use crate::{cubie::CubieCube, moves::Formula};

    #[test]
    fn test_roux() {
        let cc = CubieCube::default();
        let f = Formula::scramble();
        let cc = cc.apply_formula(&f);
        let mut roux = RouxSolver::new(cc);
        let _roux = roux.solve();
        assert!(roux.is_solved());
        println!("Scramble: {:?}\nRoux Solution: {:?}", f.moves, _roux);
    }
}

fn get_available_move(m: Move, moveset: &Vec<Move>) -> Vec<Move> {
    match m {
        U | U2 | U3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "U")
            .collect::<Vec<_>>(),
        R | R2 | R3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "R")
            .collect::<Vec<_>>(),
        F | F2 | F3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "F")
            .collect::<Vec<_>>(),
        D | D2 | D3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "D")
            .collect::<Vec<_>>(),
        L | L2 | L3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "L")
            .collect::<Vec<_>>(),
        B | B2 | B3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "B")
            .collect::<Vec<_>>(),
        M | M2 | M3 => moveset
            .clone()
            .into_iter()
            .filter(|k| k.get_face() != "M")
            .collect::<Vec<_>>(),
        _ => moveset.clone(),
    }
}