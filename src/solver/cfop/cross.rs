use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::facelet::Color;
use crate::moves::Move::{self, *};
use crate::{
    constants::ALL_EDGES,
    cubie::{
        CubieCube,
        Edge::{self, *},
    },
};

pub struct CrossSolver {
    pub cube: CubieCube,
}

impl CrossSolver {
    /// Simulate the cube rotation by updating four edges.
    pub fn _rotate(edges: Vec<(Edge, u8, u8)>, step: Move) -> Vec<(Edge, u8, u8)> {
        let mut cc = CubieCube::default();
        let mut ep = cc.ep.clone();
        let mut eo = cc.eo.clone();
        for i in 4..8 {
            ep[i] = UR;
        }
        for edge in edges.clone() {
            let ei = edge.1 as usize;
            ep[ei] = edge.0;
            eo[ei] = edge.2;
        }
        for edge in ALL_EDGES {
            let mut counts = HashMap::new();
            for _edge in ep {
                counts
                    .entry(_edge)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            if !ep.contains(&edge) {
                for _e in counts {
                    if _e.1 > 1 {
                        for i in 0..12 {
                            if ep[i] == _e.0 {
                                ep[i] = edge;
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }
        cc.ep = ep;
        cc.eo = eo;
        let cc = cc.apply_move(step);
        let d_edges = cc.get_edges_d();
        // println!("Edges: {:?}, Move: {:?}, Got: {:?}", edges, step, d_edges);
        d_edges
    }

    /// Successors function for solving the cross.
    pub fn cross_successors(
        state: ([Color; 6], Vec<(Edge, u8, u8)>),
        last_action: Option<Move>,
    ) -> Vec<(Move, ([Color; 6], Vec<(Edge, u8, u8)>))> {
        let (centres, edges) = state;
        let mut acts = HashSet::new();
        for m in [
            R, R2, R3, L, L2, L3, U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3,
        ] {
            acts.insert(m);
        }

        if last_action.is_some() {
            let la = format!("{:?}", last_action.expect("None"));
            let face = &(la.as_str())[0..1];
            for s in ["", "'", "2"] {
                acts.remove(&Move::from_str(format!("{}{}", face, s).as_str()).unwrap());
            }
        }
        let mut acts = acts.iter().collect::<Vec<&Move>>();
        acts.sort();
        let mut result = Vec::new();
        for step in acts {
            let new_edges = CrossSolver::_rotate(edges.clone(), *step);
            result.push((*step, (centres.clone(), new_edges)));
        }
        result
    }

    /// The goal function for cross solving search.
    /// MUST rotate cube to right position first.
    pub fn cross_goal(state: ([Color; 6], Vec<(Edge, u8, u8)>)) -> bool {
        let (_centres, edges) = state;
        let mut solved = 0;
        for edge in edges {
            match edge {
                (DR, 4, 0) | (DF, 5, 0) | (DL, 6, 0) | (DB, 7, 0) => solved += 1,
                _ => {}
            };
        }
        if solved == 4 {
            return true;
        }
        false
    }

    /// Compute the state value of the cross solving search.
    pub fn cross_state_value(state: ([Color; 6], Vec<(Edge, u8, u8)>)) -> u32 {
        let (centres, edges) = state;
        let mut value = 0;
        for edge in edges.clone() {
            let _edge = Edge::try_from(edge.1).unwrap();
            match _edge {
                UR | UF | UL | UB => {
                    if edge.2 == 0 {
                        value += 1;
                    } else {
                        value += 2;
                    }
                }
                DR | DF | DL | DB => {
                    if edge.2 == 1 {
                        value += 3;
                    }
                }
                _ => value += 1,
            }
        }
        let mut edgeposes = HashMap::new();
        let mut counts = HashMap::new();
        for f in [Color::L, Color::F, Color::R, Color::B] {
            counts.insert(f, 0);
        }
        let mut ngedges = Vec::new();
        for edge in edges.clone() {
            let _edge = Edge::try_from(edge.1).unwrap();
            if _edge == UR || _edge == UF || _edge == UL || _edge == UB {
                if edge.2 == 0 {
                    let (k, c) = edge_to_pos(edge).1;
                    edgeposes.insert(k, c);
                    counts.insert(k, counts[&k] + 1);
                } else {
                    ngedges.push(edge);
                }
            } else if _edge == DR || _edge == DF || _edge == DL || _edge == DB {
                if edge.2 == 0 {
                    let (k, c) = edge_to_pos(edge).1;
                    edgeposes.insert(k, c);
                    counts.insert(k, counts[&k] + 1);
                } else {
                    ngedges.push(edge);
                }
            } else {
                let (e1, e2) = edge_to_pos(edge);
                if e1.1 != centres[3] {
                    edgeposes.insert(e1.0, e1.1);
                    counts.insert(e1.0, counts[&e1.0] + 1);
                } else if e2.1 != centres[3] {
                    edgeposes.insert(e2.0, e2.1);
                    counts.insert(e2.0, counts[&e2.0] + 1);
                }
            }
        }
        for edge in ngedges {
            let color = edge_to_pos(edge);
            let idx = match color.0 .0 {
                Color::L => 0,
                Color::F => 1,
                Color::R => 2,
                _ => 3,
            };
            let mut br = false;
            for _ in [-1, 1] {
                let k = match (idx + 1) % 4 {
                    0 => Color::L,
                    1 => Color::F,
                    2 => Color::R,
                    _ => Color::B,
                };
                if !edgeposes.contains_key(&k) {
                    let k2 = color.1 .1;
                    edgeposes.insert(k, k2);
                    counts.insert(k, counts[&k] + 1);
                    br = true;
                    break;
                }
            }
            if !br {
                let k2 = match (idx + 3) % 4 {
                    0 => Color::L,
                    1 => Color::F,
                    2 => Color::R,
                    _ => Color::B,
                };
                let k3 = match (idx + 1) % 4 {
                    0 => Color::L,
                    1 => Color::F,
                    2 => Color::R,
                    _ => Color::B,
                };
                if counts[&k2] > counts[&k3] {
                    edgeposes.insert(k2, color.1 .1);
                } else {
                    edgeposes.insert(k3, color.1 .1);
                }
            }
        }

        let mut relative_pos = HashMap::new();
        relative_pos.insert(Color::R, centres[1]);
        relative_pos.insert(Color::F, centres[2]);
        relative_pos.insert(Color::L, centres[4]);
        relative_pos.insert(Color::B, centres[5]);
        if edgeposes.len() == 4 {
            let mut br = false;
            for _ in 0..4 {
                let tmp = edgeposes[&Color::L];
                edgeposes.insert(Color::L, edgeposes[&Color::F]);
                edgeposes.insert(Color::F, edgeposes[&Color::R]);
                edgeposes.insert(Color::R, edgeposes[&Color::B]);
                edgeposes.insert(Color::B, tmp);
                if edgeposes == relative_pos {
                    br = true;
                    break;
                }
            }
            if !br {
                value += 5;
            }
        } else {
            value += 3;
        }
        value
    }

    /// Solve the cross.
    pub fn solve(&mut self) -> Vec<Move> {
        let centers = self.cube.center;
        let d_edges = self.cube.get_edges_d();
        a_star_search(
            (centers, d_edges),
            CrossSolver::cross_successors,
            CrossSolver::cross_state_value,
            CrossSolver::cross_goal,
        )
    }

    pub fn is_solved(&self) -> bool {
        let centers = self.cube.center;
        let d_edges = self.cube.get_edges_d();
        CrossSolver::cross_goal((centers, d_edges))
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

pub fn edge_to_face(edge: Edge) -> (Color, Color) {
    let edge = format!("{:?}", edge);
    let edge = edge.as_bytes();
    (
        Color::try_from(char::from(edge[0])).unwrap(),
        Color::try_from(char::from(edge[1])).unwrap(),
    )
}

pub fn edge_to_pos(edge: (Edge, u8, u8)) -> ((Color, Color), (Color, Color)) {
    let _edge = Edge::try_from(edge.1).unwrap();
    let colors = edge_to_face(edge.0);
    let faces = edge_to_face(_edge);
    if edge.2 == 0 {
        ((faces.0, colors.0), (faces.1, colors.1))
    } else {
        ((faces.1, colors.0), (faces.0, colors.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cubie::CubieCube, scramble};
    #[test]
    fn test_cross_solver() {
        let cc = CubieCube::default();
        let d_edges = cc.get_edges_d();
        let solved = CrossSolver::cross_goal((cc.center, d_edges));
        assert!(solved);
        // let moves = vec![
        //     L2, B3, U, L2, F2, U, L2, D, R, B2, L3, F, U2, L2, B3, D3, F2, U3, L3, D, R2, U3, L
        // ];
        let moves = vec![L2, R, F2, L, D, U, R, L, D, F, U];
        let cc = cc.apply_moves(&moves);

        // let csv = CrossSolver::cross_state_value((cc.center, d_edges.clone()));
        // assert_eq!(csv, 7);
        // for edge in d_edges.clone() {
        //     let _edge = Edge::try_from(edge.1).unwrap();
        //     println!("{:?}, {:?}, {}, {}", edge.0, _edge, edge.1, edge.2);
        // }
        // let ne = CrossSolver::_rotate(d_edges.clone(), R);
        // for edge in ne {
        //     let _edge = Edge::try_from(edge.1).unwrap();
        //     println!("{:?}, {:?}, {}, {}", edge.0, _edge, edge.1, edge.2);
        // }
        let mut cs = CrossSolver { cube: cc };
        let result = cs.solve();
        let cc = cc.apply_moves(&result);
        let d_edges = cc.get_edges_d();
        let solved = CrossSolver::cross_goal((cc.center, d_edges.clone()));
        assert!(solved);
        // println!("{:?}", result);
        for i in 0..100 {
            let cc = CubieCube::default();
            let moves = scramble();
            let cc = cc.apply_moves(&moves);
            let mut cs = CrossSolver{cube: cc};
            let solution = cs.solve();
            let cc = cc.apply_moves(&solution);
            let d_edges = cc.get_edges_d();
            let solved = CrossSolver::cross_goal((cc.center, d_edges.clone()));
            assert!(solved);
            println!("Testing {}, Moves: {:?}, Solution: {:?}, Solved: {:?}", i, &moves, &solution, solved);
        }
    }
}
