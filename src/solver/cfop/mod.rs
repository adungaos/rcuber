//! # CFOP
//! `CFOP` (Cross, F2L, OLL, PLL, pronounced C-F-O-P or C-fop) is a 3x3 speedsolving method proposed by several cubers around 1981.
//! It is also known as the Fridrich Method after its popularizer, Jessica Fridrich.
//! In part due to Fridrich's publication of the method on her website in 1995, CFOP has been the most dominant 3x3 speedcubing method since around 2000.
//! # Steps
//! `CFOP` can be viewed as an advanced version of a Layer-By-Layer method. It takes the same steps, but combines some of them, solving more of the cube at once.
//! ## Cross
//! The first step is to make a cross on the bottom face by solving four edge pieces that share one color (white in this example).
//! Virtually all top CFOP solvers nowadays solve the cross on bottom to avoid doing a z2 or x2 cube rotation. Previously in the 2000s it was also popular to solve on a different face, for example Cross on left. Many top solvers are also color neutral, meaning they are able to solve the cross on any color. This allows them to find better solutions in many cases.
//! ## F2L (First Two Layers)
//! In between the solved cross edges and their corresponding centers are four slots that contains a corner and an edge piece. The goal of this step is to fill in these slots with the right pieces to solve the first two layers at the same time. This is accomplished by pairing up a corner that shares a color with the cross, and an edge that shares its colors with said corner, then inserting them together. The completion of this step leaves one with just the last layer, typically placed on top.
//! ## OLL (Orientation of the Last Layer)
//! In this step, the goal is to make the top face one color. There are 57 nontrivial cases, and therefore 57 algorithms to learn for this step
//! Those new to OLL break up the step into two. This greatly reduces the number of cases; 2-look OLL has 9 cases. However, note that this is a few seconds slower
//! ## PLL (Permutation of the Last Layer)
//! Finally, the cube is solved by permuting the pieces of the last layer, in other words putting them in the correct position. There are 21 nontrivial cases for this step.
//! Those new to PLL break up the step into two. This greatly reduces the number of cases; 2-look PLL has 6 cases. However, note that this is a few seconds slower

use std::collections::HashMap;

pub use cross::CrossSolver;
pub use f2l::F2LSolver;
pub use oll::OLLSolver;
pub use pll::PLLSolver;

use crate::{
    cubie::{Corner, CubieCube, Edge},
    facelet::Color,
    moves::Move,
};

/// Module for CFOP's first step, solving Rubik's Cube Cross.
pub mod cross;
/// Module for CFOP's step 2, solving Rubik's Cube F2L.
pub mod f2l;
/// Module for CFOP's step 3, solving Rubik's Cube OLL.
pub mod oll;
/// Module for CFOP's last step, solving Rubik's Cube PLL.
pub mod pll;

/// CFOPSolver for solve a cube use CFOP method.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::cfop::CFOPSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = Formula::scramble();
///     let cc = cc.apply_formula(&moves);
///     let mut solver = CFOPSolver{cube: cc};
///     assert!(!solver.is_solved());
///     let solution = solver.solve();
///     assert!(solver.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
/// }
/// ```
pub struct CFOPSolver {
    pub cube: CubieCube,
}

impl CFOPSolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();

        let mut cross = CrossSolver { cube: self.cube };
        let mut cs = cross.solve();
        assert!(cross.is_solved());
        self.cube = cross.cube;
        solution.append(&mut cs);

        let mut f2l = F2LSolver { cube: self.cube };
        let mut fs = f2l.solve();
        assert!(f2l.is_solved());
        self.cube = f2l.cube;
        solution.append(&mut fs);

        let mut oll = OLLSolver::new(self.cube);
        let mut os = oll.solve();
        assert!(oll.is_solved());
        self.cube = oll.cube;
        solution.append(&mut os);

        let mut pll = PLLSolver::new(self.cube);
        let mut ps = pll.solve();
        assert!(pll.is_solved());
        self.cube = pll.cube;
        solution.append(&mut ps);

        solution
    }

    pub fn is_solved(&self) -> bool {
        self.cube == CubieCube::default()
    }
}

/// This is a searching function of A*
pub fn a_star_search<S, V, G>(
    start: &CubieCube,
    successors: S,
    state_value: V,
    is_goal: G,
) -> Vec<Move>
where
    S: Fn(&CubieCube, Option<Move>) -> Vec<(Move, CubieCube)>,
    V: Fn(&CubieCube) -> u32,
    G: Fn(&CubieCube) -> bool,
{
    if is_goal(&start) {
        return Vec::new();
    }
    let mut explored = Vec::new();
    let h = state_value(&start);
    let g = 1;
    let f = g + h;
    let p = vec![(None, *start)];
    let mut frontier = Vec::new();
    frontier.push((f, g, h, p));
    while frontier.len() > 0 {
        let (_f, g, _h, path) = frontier.remove(0);
        let s = path.last().unwrap();
        let la = s.0;
        for (action, state) in successors(&s.1, la.clone()) {
            if !explored.contains(&state) {
                explored.push(state);
                let mut path2 = path.clone();
                path2.push((Some(action), state));
                if is_goal(&state) {
                    let mut r = Vec::new();
                    for (a, _s) in path2 {
                        if a.is_some() {
                            r.push(a.expect("Move Error!"));
                        }
                    }
                    return r;
                } else {
                    let h2 = state_value(&state);
                    let g2 = g + 1;
                    let f2 = h2 + g2;
                    frontier.push((f2, g2, h2, path2));
                    frontier.sort_by_key(|k| (k.0, k.1, k.2));
                }
            }
        }
    }
    Vec::new()
}

/// Split Edge expression (ex UR) to two faces(ex U & R).  
fn edge_to_face(edge: Edge) -> (Color, Color) {
    let edge = format!("{:?}", edge);
    let edge = edge.as_bytes();
    (
        Color::try_from(char::from(edge[0])).unwrap(),
        Color::try_from(char::from(edge[1])).unwrap(),
    )
}

/// Get the colors of Edge's faces. Ex, (Original Edge, current Position, orientation) -> (Current Face, Original Color)x2.  
fn edge_to_pos(edge: (Edge, u8, u8)) -> [(Color, Color); 2] {
    let _edge = Edge::try_from(edge.1).unwrap();
    let colors = edge_to_face(edge.0);
    let faces = edge_to_face(_edge);
    if edge.2 == 0 {
        [(faces.0, colors.0), (faces.1, colors.1)]
    } else {
        [(faces.1, colors.0), (faces.0, colors.1)]
    }
}

/// Correct slot vector's order.
fn correct_slot(slot: [Color; 2]) -> [Color; 2] {
    match slot {
        [Color::R, Color::F] => [Color::F, Color::R],
        [Color::R, Color::B] => [Color::B, Color::R],
        [Color::L, Color::B] => [Color::B, Color::L],
        [Color::L, Color::F] => [Color::F, Color::L],
        _ => slot,
    }
}

/// Split Corner expression (ex URF) to three faces(ex U, R, F).  
fn corner_to_face(corner: Corner) -> (Color, Color, Color) {
    let corner = format!("{:?}", corner);
    let corner = corner.as_bytes();
    (
        Color::try_from(char::from(corner[0])).unwrap(),
        Color::try_from(char::from(corner[1])).unwrap(),
        Color::try_from(char::from(corner[2])).unwrap(),
    )
}

/// Get the colors of Corner's faces.
/// Ex, (Original Corner, current Position, orientation) -> (Current Face: Original Color)x3.  
fn corner_to_pos(corner: (Corner, u8, u8)) -> HashMap<Color, Color> {
    let _corner = Corner::try_from(corner.1).unwrap();
    let colors = corner_to_face(corner.0);
    let faces = corner_to_face(_corner);
    let mut r = HashMap::new();
    if corner.2 == 0 {
        r.insert(faces.0, colors.0);
        r.insert(faces.1, colors.1);
        r.insert(faces.2, colors.2);
    } else if corner.2 == 1 {
        r.insert(faces.0, colors.2);
        r.insert(faces.1, colors.0);
        r.insert(faces.2, colors.1);
    } else {
        r.insert(faces.0, colors.1);
        r.insert(faces.1, colors.2);
        r.insert(faces.2, colors.0);
    }
    r
}

#[cfg(test)]
mod tests {
    use crate::{cubie::CubieCube, moves::Formula, solver::CFOPSolver};

    #[test]
    fn test_cfop() {
        let cc = CubieCube::default();
        let moves = Formula::scramble();
        let cc = cc.apply_formula(&moves);
        let cc2 = cc.clone();
        let mut solver = CFOPSolver { cube: cc };
        let solution = solver.solve();
        assert!(solver.is_solved());

        let cc2 = cc2.apply_moves(&solution);
        assert_eq!(cc2, CubieCube::default());
        println!("Scramble: {:?}\nSolution: {:?}", moves, solution);
    }
}
