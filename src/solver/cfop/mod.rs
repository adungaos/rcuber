use std::collections::HashMap;

use cross::CrossSolver;
use f2l::F2LSolver;
use oll::OLLSolver;
use pll::PLLSolver;

use crate::{cubie::{Corner, CubieCube, Edge}, facelet::Color, moves::Move};

pub mod cross;
pub mod f2l;
pub mod oll;
pub mod pll;

pub struct CFOPSolver {
    pub cube: CubieCube,
}

impl CFOPSolver {
    pub fn solve(&mut self) ->Vec<Move>{
        let mut solution = Vec::new();
        
        let mut cross = CrossSolver{cube: self.cube};
        let mut cs = cross.solve();
        assert!(cross.is_solved());
        solution.append(&mut cs);

        let mut f2l = F2LSolver{cube: cross.cube};
        let mut fs = f2l.solve();
        assert!(f2l.is_solved());
        solution.append(&mut fs);

        let mut oll = OLLSolver::new(f2l.cube);
        let mut os = oll.solve();
        assert!(oll.is_solved());
        solution.append(&mut os);

        let mut pll = PLLSolver::new(oll.cube);
        let mut ps = pll.solve();
        assert!(pll.is_solved());
        solution.append(&mut ps);

        self.cube = pll.cube;
        solution
    }

    pub fn is_solved(&self) -> bool {
        let cc = CubieCube::default();
        if cc == self.cube {
            return true;
        }
        false
    }
}


/// This is a searching function of A*
pub fn a_star_search<'a, S, V, G>(
    start: ([Color; 6], Vec<(Edge, u8, u8)>),
    successors: S,
    state_value: V,
    is_goal: G,
) -> Vec<Move>
where
    S: Fn(
        ([Color; 6], Vec<(Edge, u8, u8)>),
        Option<Move>,
    ) -> Vec<(Move, ([Color; 6], Vec<(Edge, u8, u8)>))>,
    V: Fn(([Color; 6], Vec<(Edge, u8, u8)>)) -> u32,
    G: Fn(([Color; 6], Vec<(Edge, u8, u8)>)) -> bool,
{
    if is_goal(start.clone()) {
        return Vec::new();
    }
    let mut explored = Vec::new();
    let h = state_value(start.clone());
    let g = 1;
    let f = g + h;
    let p = vec![(None, start)];
    let mut frontier = Vec::new();
    frontier.push((f, g, h, p));
    while frontier.len() > 0 {
        let (_f, g, _h, path) = frontier.remove(0);
        let s = path.last().unwrap();
        let la = s.0;
        for (action, state) in successors(s.1.clone(), la.clone()) {
            if !explored.contains(&state) {
                explored.push(state.clone());
                let mut path2 = path.clone();
                path2.push((Some(action), state.clone()));
                if is_goal(state.clone()) {
                    let mut r = Vec::new();
                    for (a, _s) in path2 {
                        if a.is_some() {
                            r.push(a.expect("Move Error!"));
                        }
                    }
                    return r;
                } else {
                    let h2 = state_value(state.clone());
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
pub fn edge_to_face(edge: Edge) -> (Color, Color) {
    let edge = format!("{:?}", edge);
    let edge = edge.as_bytes();
    (
        Color::try_from(char::from(edge[0])).unwrap(),
        Color::try_from(char::from(edge[1])).unwrap(),
    )
}

/// Get the colors of Edge's faces. Ex, (Original Edge, current Position, orientation) -> (Current Face, Original Color)x2.  
pub fn edge_to_pos(edge: (Edge, u8, u8)) -> [(Color, Color);2] {
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
pub fn correct_slot(slot: [Color; 2]) -> [Color;2] {
    match slot {
        [Color::R, Color::F] => [Color::F, Color::R],
        [Color::R, Color::B] => [Color::B, Color::R],
        [Color::L, Color::B] => [Color::B, Color::L],
        [Color::L, Color::F] => [Color::F, Color::L],
        _ => slot,
    }
}
/// Split Corner expression (ex URF) to three faces(ex U, R, F).  
pub fn corner_to_face(corner: Corner) -> (Color, Color, Color) {
    let corner = format!("{:?}", corner);
    let corner = corner.as_bytes();
    (
        Color::try_from(char::from(corner[0])).unwrap(),
        Color::try_from(char::from(corner[1])).unwrap(),
        Color::try_from(char::from(corner[2])).unwrap(),
    )
}

/// Get the colors of Corner's faces. Ex, (Original Corner, current Position, orientation) -> (Current Face, Original Color)x3.  
pub fn corner_to_pos(corner: (Corner, u8, u8)) -> HashMap<Color, Color> {
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