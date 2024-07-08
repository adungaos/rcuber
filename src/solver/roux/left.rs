use crate::{cubie::CubieCube, moves::Move};


/// LBSolver for solve Roux's Left Block(a 1x2x3 block at left bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::left::LBSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves =Formula::scramble();
///     println!("Scramble: {:?}", moves);
///     let cc = cc.apply_formula(&moves);
///     let mut lb = LBSolver{cube: cc};
///     let solution = lb.solve();
///     assert!(lb.is_solved());
///     println!("Left Block Solution: {:?}", solution);
/// }
/// ```
pub struct LBSolver{
    pub cube: CubieCube,
}

impl LBSolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        if self.is_solved() {
            return solution;
        }
        let mut dle = DLEdgeSolver{cube: self.cube};
        let mut dles = dle.solve();
        assert!(dle.is_solved());
        self.cube = dle.cube;
        solution.append(&mut dles);
        let mut bls = BLSlotSolver{cube: self.cube};
        let mut blss = bls.solve();
        assert!(bls.is_solved());
        self.cube = bls.cube;
        solution.append(&mut blss);
        let mut fls = FLSlotSolver{cube: self.cube};
        let mut flss = fls.solve();
        assert!(fls.is_solved());
        self.cube = fls.cube;
        solution.append(&mut flss);
        solution
    }
    pub fn is_solved(&self) -> bool {
        true
    }
}

struct DLEdgeSolver{
    pub cube: CubieCube,
}

impl DLEdgeSolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let solution = Vec::new();
        if self.is_solved() {
            return solution;
        }
        solution
    }
    pub fn is_solved(&self) -> bool {
        true
    }
}


struct BLSlotSolver{
    pub cube: CubieCube,
}

impl BLSlotSolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let solution = Vec::new();
        if self.is_solved() {
            return solution;
        }
        solution
    }
    pub fn is_solved(&self) -> bool {
        true
    }
}

struct FLSlotSolver{
    pub cube: CubieCube,
}

impl FLSlotSolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let solution = Vec::new();
        if self.is_solved() {
            return solution;
        }
        solution
    }
    pub fn is_solved(&self) -> bool {
        true
    }
}