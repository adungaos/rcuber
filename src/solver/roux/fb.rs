use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::solver::cfop::edge_to_pos;
use crate::{
    cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE},
    generator::Generator,
    moves::Move::{self, *},
};

use super::utils::a_star_search;
use super::{
    square::FaceCube,
    utils::{generate_pruning_table, generate_solution},
};

/// FBSolver for solve Roux's First Block(a 1x2x3 block at left bottom).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::fb::FBSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let f = Formula::scramble();
///     println!("Scramble: {:?}", f);
///     let cc = cc.apply_formula(&f);
///     let mut fb = FBSolver{cube: cc};
///     let solution = fb.solve();
///     assert!(fb.is_solved());
///     println!("First Block Solution: {:?}", solution);
/// }
/// ```
pub struct FBSolver {
    pub cube: CubieCube,
}

impl FBSolver {
    /// Solve the fb.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut corners = Vec::new();
        for i in 0..8 {
            match self.cube.cp[i] {
                Corner::DLF | Corner::DBL => corners.push((self.cube.cp[i], i as u8, self.cube.co[i])),
                _ => {}
            }
        }
        let mut edges = Vec::new();
        for i in 0..12 {
            match self.cube.ep[i] {
                Edge::DL | Edge::FL | Edge::BL => edges.push((self.cube.ep[i], i as u8, self.cube.eo[i])),
                _ => {}
            }
        }
        let solution = a_star_search(
            (corners, edges),
            FBSolver::fb_successors,
            FBSolver::fb_state_value,
            FBSolver::fb_goal,
        );
        self.cube = self.cube.apply_moves(&solution);
        solution
    }
    // pub fn solve(&mut self) -> Vec<Move> {
    //     let stm = vec![
    //         U, U2, U3, D, D2, D3, L, L2, L3, R, R2, R3, F, F2, F3, B, B2, B3, M, M2, M3,
    //     ];
    //     // let fb_pieces = vec![21, 23, 27, 30, 33, 39, 40, 41, 42, 43, 44, 50, 53];
    //     let fb_pruning_table =
    //         generate_pruning_table(vec![FaceCube::try_from(&self.cube).unwrap()], 5, &stm);
    //     for i in 1..=10 {
    //         let s = generate_solution(&self.cube, &stm, &Vec::new(), i, &fb_pruning_table);
    //         if s.len() > 0 {
    //             self.cube = self.cube.apply_moves(&s);
    //             return s;
    //         }
    //     }
    //     Vec::new()
    // }

    pub fn is_solved(&self) -> bool {
        if self.cube.center[4] != SOLVED_CUBIE_CUBE.center[4] {
            return false;
        }
        for i in 5..=6 {
            if self.cube.cp[i] != SOLVED_CUBIE_CUBE.cp[i] || self.cube.co[i] != 0 {
                return false;
            }
        }
        for i in [6, 9, 10] {
            if self.cube.ep[i] != SOLVED_CUBIE_CUBE.ep[i] || self.cube.eo[i] != 0 {
                return false;
            }
        }
        true
    }

    fn _rotate(
        state: (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>),
        step: Move,
    ) -> (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>) {
        let (corners, edges) = state;
        let mut cp = [-1; 8];
        let mut co = [-1; 8];
        let mut ep = [-1; 12];
        let mut eo = [-1; 12];
        for c in corners {
            cp[c.1 as usize] = c.0 as i8;
            co[c.1 as usize] = c.2 as i8;
        }
        for e in edges {
            ep[e.1 as usize] = e.0 as i8;
            eo[e.1 as usize] = e.2 as i8;
        }
        let cc = Generator::gen_state(cp, co, ep, eo);
        let cc = cc.apply_move(step);
        let mut corners = Vec::new();
        for i in 0..8 {
            match cc.cp[i] {
                Corner::DLF | Corner::DBL => corners.push((cc.cp[i], i as u8, cc.co[i])),
                _ => {}
            }
        }
        let mut edges = Vec::new();
        for i in 0..12 {
            match cc.ep[i] {
                Edge::DL | Edge::FL | Edge::BL => edges.push((cc.ep[i], i as u8, cc.eo[i])),
                _ => {}
            }
        }
        (corners, edges)
    }

    /// Successors function for solving the first block.
    pub fn fb_successors(
        state: (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>),
        last_action: Option<Move>,
    ) -> Vec<(Move, (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>))> {
        let mut acts = HashSet::new();
        for m in [
            R, R2, R3, L, L2, L3, U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, M, M2, M3,
        ] {
            acts.insert(m);
        }

        if last_action.is_some() {
            let face = last_action.unwrap().get_face();
            for s in ["", "'", "2"] {
                acts.remove(&Move::from_str(format!("{}{}", face, s).as_str()).unwrap());
            }
        }
        let mut acts = acts.iter().collect::<Vec<&Move>>();
        acts.sort();
        let mut result = Vec::new();
        for step in acts {
            let new_state = FBSolver::_rotate(state.clone(), *step);
            result.push((*step, new_state));
        }
        result
    }

    /// The goal function for fb solving search.
    /// MUST rotate cube to right position first.
    pub fn fb_goal(state: (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>)) -> bool {
        let (corners, edges) = state;
        let mut solved = 0;
        for c in corners {
            match c {
                (Corner::DLF, 5, 0) | (Corner::DBL, 6, 0) => solved += 1,
                _ => {}
            };
        }
        for e in edges {
            match e {
                (Edge::FL, 9, 0) | (Edge::BL, 10, 0) | (Edge::DL, 6, 0) => solved += 1,
                _ => {}
            };
        }
        if solved == 5 {
            return true;
        }
        false
    }

    /// Compute the state value of the fb solving search.
    pub fn fb_state_value(state: (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>)) -> u32 {
        let (corners, edges) = state;
        let mut value = 0;
        for edge in edges.clone() {
            let _edge = Edge::try_from(edge.1).unwrap();
            match _edge {
                Edge::UR | Edge::UF | Edge::UL | Edge::UB => {
                    if edge.2 == 0 {
                        value += 1;
                    } else {
                        value += 2;
                    }
                }
                Edge::DR | Edge::DF | Edge::DL | Edge::DB => {
                    if edge.2 == 1 {
                        value += 3;
                    }
                }
                _ => value += 1,
            }
        }
        for c in corners {
            let _c = Corner::try_from(c.1).unwrap();
            match _c {
                Corner::DBL | Corner::DFR | Corner::DLF | Corner::DRB => match c.2 {
                    0 => value += 1,
                    _ => value += 3,
                },
                _ => match c.2 {
                    0 => value += 2,
                    _ => value += 4,
                },
            }
        }
        // let mut edgeposes = HashMap::new();
        // let mut counts: HashMap<crate::facelet::Color, u32> = HashMap::new();

        // let mut ngedges = Vec::new();
        // for edge in edges.clone() {
        //     let _edge = Edge::try_from(edge.1).unwrap();
        //     if _edge == Edge::UR || _edge == Edge::UF || _edge == Edge::UL || _edge == Edge::UB {
        //         if edge.2 == 0 {
        //             let (k, c) = edge_to_pos(edge)[1];
        //             edgeposes.insert(k, c);
        //             counts.insert(k, counts[&k] + 1);
        //         } else {
        //             ngedges.push(edge);
        //         }
        //     } else if _edge == Edge::DR || _edge == Edge::DF || _edge == Edge::DL || _edge == Edge::DB {
        //         if edge.2 == 0 {
        //             let (k, c) = edge_to_pos(edge)[1];
        //             edgeposes.insert(k, c);
        //             counts.insert(k, counts[&k] + 1);
        //         } else {
        //             ngedges.push(edge);
        //         }
        //     } else {
        //         let _ep = edge_to_pos(edge);
        //         let e1 = _ep[0];
        //         let e2 = _ep[1];
        //         // if e1.1 != centres[3] {
        //         //     edgeposes.insert(e1.0, e1.1);
        //         //     counts.insert(e1.0, counts[&e1.0] + 1);
        //         // } else if e2.1 != centres[3] {
        //         //     edgeposes.insert(e2.0, e2.1);
        //         //     counts.insert(e2.0, counts[&e2.0] + 1);
        //         // }
        //     }
        // }
        // for edge in ngedges {
        //     let color = edge_to_pos(edge);
        //     let idx = match color[0].0 {
        //         Color::L => 0,
        //         Color::F => 1,
        //         Color::R => 2,
        //         _ => 3,
        //     };
        //     let mut br = false;
        //     for _ in [-1, 1] {
        //         let k = match (idx + 1) % 4 {
        //             0 => Color::L,
        //             1 => Color::F,
        //             2 => Color::R,
        //             _ => Color::B,
        //         };
        //         if !edgeposes.contains_key(&k) {
        //             let k2 = color[1].1;
        //             edgeposes.insert(k, k2);
        //             counts.insert(k, counts[&k] + 1);
        //             br = true;
        //             break;
        //         }
        //     }
        //     if !br {
        //         let k2 = match (idx + 3) % 4 {
        //             0 => Color::L,
        //             1 => Color::F,
        //             2 => Color::R,
        //             _ => Color::B,
        //         };
        //         let k3 = match (idx + 1) % 4 {
        //             0 => Color::L,
        //             1 => Color::F,
        //             2 => Color::R,
        //             _ => Color::B,
        //         };
        //         if counts[&k2] > counts[&k3] {
        //             edgeposes.insert(k2, color[1].1);
        //         } else {
        //             edgeposes.insert(k3, color[1].1);
        //         }
        //     }
        // }

        // let mut relative_pos = HashMap::new();
        // relative_pos.insert(Color::R, centres[1]);
        // relative_pos.insert(Color::F, centres[2]);
        // relative_pos.insert(Color::L, centres[4]);
        // relative_pos.insert(Color::B, centres[5]);
        // if edgeposes.len() == 4 {
        //     let mut br = false;
        //     for _ in 0..4 {
        //         let tmp = edgeposes[&Color::L];
        //         edgeposes.insert(Color::L, edgeposes[&Color::F]);
        //         edgeposes.insert(Color::F, edgeposes[&Color::R]);
        //         edgeposes.insert(Color::R, edgeposes[&Color::B]);
        //         edgeposes.insert(Color::B, tmp);
        //         if edgeposes == relative_pos {
        //             br = true;
        //             break;
        //         }
        //     }
        //     if !br {
        //         value += 5;
        //     }
        // } else {
        //     value += 3;
        // }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::FBSolver;
    use crate::{
        cubie::CubieCube,
        moves::{Formula, Move::*},
        solver::roux::{square::FaceCube, utils::generate_pruning_table},
    };

    #[test]
    fn test_fb() {
        let cc = CubieCube::default();
        let f = Formula::scramble();
        let f = Formula {
            moves: vec![L,D],
        };
        println!("Scramble: {:?}", f);
        let cc = cc.apply_formula(&f);
        let mut fb = FBSolver { cube: cc };
        let solution = fb.solve();
        println!("First Block Solution: {:?}", solution);
        assert!(fb.is_solved());
        // println!("First Block Solution: {:?}", solution);
    }

    #[test]
    fn test_prunning() {
        let stm = vec![
            U, U2, U3, D, D2, D3, L, L2, L3, R, R2, R3, F, F2, F3, B, B2, B3, M, M2, M3,
        ];
        let formula = Formula::scramble();
        let cc = CubieCube::default().apply_formula(&formula);
        // let fb_pieces = vec![21, 23, 27, 30, 33, 39, 40, 41, 42, 43, 44, 50, 53];
        let fb_pruning_table =
            generate_pruning_table(vec![FaceCube::try_from(&cc).unwrap()], 5, &stm);
        println!(
            "len:{}, mem:{}",
            fb_pruning_table.len(),
            std::mem::size_of_val(&fb_pruning_table)
        );
    }
}
