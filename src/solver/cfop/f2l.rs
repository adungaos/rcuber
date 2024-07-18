use crate::{
    cubie::{Corner, CubieCube, Edge},
    facelet::Color,
    moves::Formula,
    moves::Move::{self, *},
};
use std::collections::HashMap;

use super::{corner_to_pos, correct_slot, edge_to_pos};

/// CrossSolver for solve CFOP's F2L. MUST SOLVE CROSS FIRST!!
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::cfop::cross::CrossSolver;
/// use rcuber::solver::cfop::f2l::F2LSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = Formula::scramble();
///     println!("Scramble: {:?}", moves);
///     let cc = cc.apply_formula(&moves);
///     let mut cross = CrossSolver{cube: cc};
///     assert!(!cross.is_solved());
///     let solution = cross.solve();
///     assert!(cross.is_solved());
///     println!("Cross Solution: {:?}", solution);
///     let mut f2l = F2LSolver{cube: cross.cube};
///     assert!(!f2l.is_solved());
///     let solution = f2l.solve();
///     assert!(f2l.is_solved());
///     println!("F2L Solution: {:?}", solution);
/// }
/// ```
pub struct F2LSolver {
    pub cube: CubieCube,
}

impl F2LSolver {
    /// Solve the entire F2L.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let mut slots_type = Vec::new();
        for slot in [
            [Color::F, Color::R],
            [Color::F, Color::L],
            [Color::B, Color::R],
            [Color::B, Color::L],
        ] {
            let solver = F2LPairSolver {
                cube: self.cube,
                pair: slot,
            };
            slots_type.push((solver.pair.clone(), solver.get_slot_type()));
        }
        slots_type.sort_by_key(|p| p.1);
        for p in slots_type {
            let mut solver = F2LPairSolver {
                cube: self.cube,
                pair: p.0,
            };
            if !solver.is_solved() {
                let mut pair_result = solver.solve();
                self.cube = self.cube.apply_moves(&pair_result);
                solution.append(&mut pair_result);
            }
        }
        solution
    }

    /// Check if Cube's F2L is solved.
    pub fn is_solved(&self) -> bool {
        let cc = CubieCube::default();
        let cp = self.cube.cp[4..8].to_vec();
        let co = self.cube.co[4..8].to_vec();
        let ep = self.cube.ep[4..12].to_vec();
        let eo = self.cube.eo[4..12].to_vec();
        let s_cp = cc.cp[4..8].to_vec();
        let s_co = cc.co[4..8].to_vec();
        let s_ep = cc.ep[4..12].to_vec();
        let s_eo = cc.eo[4..12].to_vec();
        if self.cube.center == cc.center && cp == s_cp && co == s_co && ep == s_ep && eo == s_eo {
            true
        } else {
            false
        }
    }
}

/// Slot Type by a pair's location.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum SlotType {
    SOLVED,
    SLOTFREE,
    CSLOTFREE,
    WRONGSLOT,
    ESLOTFREE,
    DIFFSLOT,
}

/// Solver for F2L's one pair (corner & edge) / slot.
struct F2LPairSolver {
    pub cube: CubieCube,
    pub pair: [Color; 2],
}

impl F2LPairSolver {
    /// Get the F2L pair (corner, edge).
    fn get_pair(&self) -> ((Corner, u8, u8), (Edge, u8, u8)) {
        let edge = format!("{:?}{:?}", self.pair[0], self.pair[1]);
        let edge = Edge::try_from(edge.as_str()).unwrap();
        let corner = format!("{:?}{:?}{:?}", Color::D, self.pair[0], self.pair[1]);
        let corner = match corner.as_str() {
            "DBR" => "DRB".to_string(),
            "DLB" => "DBL".to_string(),
            "DFL" => "DLF".to_string(),
            "DRF" => "DFR".to_string(),
            _ => corner,
        };
        let corner = Corner::try_from(corner.as_str()).unwrap();
        let mut co = 0;
        let mut cp = 0;
        for i in 0..8 {
            if self.cube.cp[i] == corner {
                cp = i;
                co = self.cube.co[i];
                break;
            }
        }
        let mut eo = 0;
        let mut ep = 0;
        for i in 0..12 {
            if self.cube.ep[i] == edge {
                ep = i;
                eo = self.cube.eo[i];
                break;
            }
        }
        ((corner, cp as u8, co), (edge, ep as u8, eo))
    }

    /// Get the estimated cubie of solved pair.
    fn estimated_position(&self) -> ((Corner, u8, u8), (Edge, u8, u8)) {
        let pair = self.get_pair();
        (
            (pair.0 .0, pair.0 .0 as u8, 0),
            (pair.1 .0, pair.1 .0 as u8, 0),
        )
    }

    /// Get the slot type of the pair.
    fn get_slot_type(&self) -> SlotType {
        let (corner, edge) = self.get_pair();
        let cc = CubieCube::default();
        let cp = cc.cp[corner.1 as usize];
        let cp = format!("{:?}", cp).replace("D", "");
        let cp = cp.as_bytes();
        let cp0 = Color::try_from(char::from(cp[0])).unwrap();
        let cp1 = Color::try_from(char::from(cp[1])).unwrap();
        let cp = [cp0, cp1];
        let ep = cc.ep[edge.1 as usize];
        let ep = format!("{:?}", ep);
        let ep = ep.as_bytes();
        let ep0 = Color::try_from(char::from(ep[0])).unwrap();
        let ep1 = Color::try_from(char::from(ep[1])).unwrap();
        let ep = [ep0, ep1];

        if cp.contains(&Color::U) && ep.contains(&Color::U) {
            return SlotType::SLOTFREE;
        }
        if cp.contains(&Color::U) {
            return SlotType::CSLOTFREE;
        }
        if ep.contains(&Color::U) {
            return SlotType::ESLOTFREE;
        }
        let mut ep_r = ep.clone();
        ep_r.reverse();
        let eps = [ep, ep_r];
        if !eps.contains(&cp) {
            return SlotType::DIFFSLOT;
        }
        if (corner, edge) == self.estimated_position() {
            return SlotType::SOLVED;
        }
        SlotType::WRONGSLOT
    }

    /// Move the paired corner & edge to U face.
    fn pair_to_uface(&mut self) -> Vec<Move> {
        let mut result = Vec::new();
        let mut put_acts = HashMap::new();
        put_acts.insert(
            [Color::F, Color::R],
            [
                vec![R, U, R3],
                vec![R, U3, R3],
                vec![R, U2, R3],
                vec![F3, U, F],
                vec![F3, U3, F],
                vec![F3, U2, F],
            ],
        );
        put_acts.insert(
            [Color::B, Color::R],
            [
                vec![R3, U, R],
                vec![R3, U3, R],
                vec![R3, U2, R],
                vec![B, U, B3],
                vec![B, U3, B3],
                vec![B, U2, B3],
            ],
        );
        put_acts.insert(
            [Color::B, Color::L],
            [
                vec![B3, U, B],
                vec![B3, U3, B],
                vec![B3, U2, B],
                vec![L, U, L3],
                vec![L, U3, L3],
                vec![L, U2, L3],
            ],
        );
        put_acts.insert(
            [Color::F, Color::L],
            [
                vec![F, U, F3],
                vec![F, U3, F3],
                vec![F, U2, F3],
                vec![L3, U, L],
                vec![L3, U3, L],
                vec![L3, U2, L],
            ],
        );
        let slot_type = self.get_slot_type();
        let (corner, edge) = self.get_pair();
        if slot_type == SlotType::SOLVED || slot_type == SlotType::SLOTFREE {
            return Vec::new();
        } else if slot_type == SlotType::CSLOTFREE || slot_type == SlotType::WRONGSLOT {
            let _edge = edge_to_pos(edge);

            let slot = [_edge[0].0, _edge[1].0];
            let slot = correct_slot(slot);
            for i in 0..4 {
                match i {
                    1 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    3 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let slot_type = self.get_slot_type();
                    if slot_type == SlotType::SLOTFREE {
                        match i {
                            1 => {
                                result.push(U);
                            }
                            2 => {
                                result.push(U2);
                            }
                            3 => {
                                result.push(U3);
                            }
                            _ => {}
                        }
                        result.append(&mut act);
                        return result;
                    }
                    let act_rev = Formula { moves: act }.inverse().moves;
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    1 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
            }
        } else if slot_type == SlotType::ESLOTFREE {
            let mut _corner = corner_to_pos(corner);
            let mut slot = Vec::new();
            for (k, _v) in _corner {
                if k == Color::D {
                    continue;
                } else {
                    slot.push(k);
                }
            }
            let slot = [slot[0], slot[1]];
            let slot = correct_slot(slot);
            for i in 0..4 {
                match i {
                    1 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    3 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let slot_type = self.get_slot_type();
                    if slot_type == SlotType::SLOTFREE {
                        match i {
                            1 => {
                                result.push(U);
                            }
                            2 => {
                                result.push(U2);
                            }
                            3 => {
                                result.push(U3);
                            }
                            _ => {}
                        }
                        result.append(&mut act);
                        return result;
                    }
                    let act_rev = Formula { moves: act }.inverse().moves;
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    1 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
            }
        } else {
            let _edge = edge_to_pos(edge);
            let slot = [_edge[0].0, _edge[1].0];
            let slot = correct_slot(slot);
            for i in 0..4 {
                match i {
                    1 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    3 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let slot_type = self.get_slot_type();
                    if slot_type == SlotType::SLOTFREE || slot_type == SlotType::ESLOTFREE {
                        match i {
                            1 => {
                                result.push(U);
                            }
                            2 => {
                                result.push(U2);
                            }
                            3 => {
                                result.push(U3);
                            }
                            _ => {}
                        }
                        result.append(&mut act);
                        let mut _combine = self.pair_to_uface();
                        result.append(&mut _combine);
                        return result;
                    }
                    let act_rev = Formula { moves: act }.inverse().moves;
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {
                        self.cube = self.cube.apply_move(U);
                    }
                    2 => {
                        self.cube = self.cube.apply_move(U2);
                    }
                    1 => {
                        self.cube = self.cube.apply_move(U3);
                    }
                    _ => {}
                }
            }
        }
        Vec::new()
    }

    /// Solve the pair.
    fn solve(&mut self) -> Vec<Move> {
        let mut combine = self.pair_to_uface();
        let estimated = self.estimated_position();
        for i in 0..4 {
            match i {
                1 => {
                    self.cube = self.cube.apply_move(U);
                }
                2 => {
                    self.cube = self.cube.apply_move(U2);
                }
                3 => {
                    self.cube = self.cube.apply_move(U3);
                }
                _ => {}
            }
            let mut put_acts = HashMap::new();
            put_acts.insert(
                [Color::F, Color::R],
                vec![
                    vec![R, U, R3],
                    vec![R, U3, R3],
                    vec![R, U2, R3],
                    vec![R, U, R3, U, R, U, R3],
                    vec![R, U, R3, U, R, U3, R3],
                    vec![R, U, R3, U2, R, U3, R3],
                    vec![R, U3, R3, U, R, U, R3],
                    vec![R, U3, R3, U2, F3, U3, F],
                    vec![R, U2, R2, F, R, F3],
                    vec![R, U2, R3, U, F3, U3, F],
                    vec![R, U2, R3, U2, R, U3, R3],
                    vec![R, U2, R3, U3, R, U, R3],
                    vec![R2, U2, R3, U3, R, U3, R2],
                    vec![R, U, R3, U, F3, U, F, U3, F3, U, F],
                    vec![F3, U, F],
                    vec![F3, U3, F],
                    vec![F3, U2, F],
                    vec![F3, U, F, U3, F3, U3, F],
                    vec![F3, U, F, U2, R, U, R3],
                    vec![F3, U3, F, U3, F3, U, F],
                    vec![F3, U3, F, U2, F3, U, F],
                    vec![F3, U3, F, U3, F3, U3, F],
                    vec![F3, U2, F, U2, F3, U, F],
                    vec![F3, U2, F, U3, R, U, R3],
                    vec![F3, U2, F, U3, F3, U, F],
                    vec![F3, U2, F, U, F3, U3, F],
                ],
            );
            put_acts.insert(
                [Color::B, Color::R],
                vec![
                    vec![R3, U3, R],
                    vec![R3, U, R],
                    vec![R3, U2, R],
                    vec![R3, U3, R, U3, R3, U3, R],
                    vec![R3, U3, R, U3, R3, U, R],
                    vec![R3, U3, R, U2, R3, U, R],
                    vec![R3, U, R, U3, R3, U3, R],
                    vec![R3, U, R, U2, B, U, B3],
                    vec![R3, U2, R2, B3, R3, B],
                    vec![R3, U2, R, U3, B, U, B3],
                    vec![R3, U2, R, U2, R3, U, R],
                    vec![R3, U2, R, U, R3, U3, R],
                    vec![R2, U2, R, U, R3, U, R2],
                    vec![R3, U3, R, U3, B, U3, B3, U, B, U3, B3],
                    vec![B, U3, B3],
                    vec![B, U, B3],
                    vec![B, U2, B3],
                    vec![B, U3, B3, U, B, U, B3],
                    vec![B, U3, B3, U2, R3, U3, R],
                    vec![B, U, B3, U, B, U3, B3],
                    vec![B, U, B3, U2, B, U3, B3],
                    vec![B, U, B3, U, B, U, B3],
                    vec![B, U2, B3, U2, B, U3, B3],
                    vec![B, U2, B3, U, R3, U3, R],
                    vec![B, U2, B3, U, B, U3, B3],
                    vec![B, U2, B3, U3, B, U, B3],
                ],
            );
            put_acts.insert(
                [Color::B, Color::L],
                vec![
                    vec![L, U, L3],
                    vec![L, U3, L3],
                    vec![L, U2, L3],
                    vec![L, U, L3, U, L, U, L3],
                    vec![L, U, L3, U, L, U3, L3],
                    vec![L, U, L3, U2, L, U3, L3],
                    vec![L, U3, L3, U, L, U, L3],
                    vec![L, U3, L3, U2, B3, U3, B],
                    vec![L, U2, L2, B, L, B3],
                    vec![L, U2, L3, U, B3, U3, B],
                    vec![L, U2, L3, U2, L, U3, L3],
                    vec![L, U2, L3, U3, L, U, L3],
                    vec![L2, U2, L3, U3, L, U3, L2],
                    vec![L, U, L3, U, B3, U, B, U3, B3, U, B],
                    vec![B3, U, B],
                    vec![B3, U3, B],
                    vec![B3, U2, B],
                    vec![B3, U, B, U3, B3, U3, B],
                    vec![B3, U, B, U2, L, U, L3],
                    vec![B3, U3, B, U3, B3, U, B],
                    vec![B3, U3, B, U2, B3, U, B],
                    vec![B3, U3, B, U3, B3, U3, B],
                    vec![B3, U2, B, U2, B3, U, B],
                    vec![B3, U2, B, U3, L, U, L3],
                    vec![B3, U2, B, U3, B3, U, B],
                    vec![B3, U2, B, U, B3, U3, B],
                ],
            );
            put_acts.insert(
                [Color::F, Color::L],
                vec![
                    vec![F, U3, F3],
                    vec![F, U, F3],
                    vec![F, U2, F3],
                    vec![F, U3, F3, U, F, U, F3],
                    vec![F, U3, F3, U2, L3, U3, L],
                    vec![F, U, F3, U, F, U3, F3],
                    vec![F, U, F3, U2, F, U3, F3],
                    vec![F, U, F3, U, F, U, F3],
                    vec![F, U2, F3, U2, F, U3, F3],
                    vec![F, U2, F3, U, L3, U3, L],
                    vec![F, U2, F3, U, F, U3, F3],
                    vec![F, U2, F3, U3, F, U, F3],
                    vec![L3, U3, L],
                    vec![L3, U, L],
                    vec![L3, U2, L],
                    vec![L3, U3, L, U3, L3, U3, L],
                    vec![L3, U3, L, U3, L3, U, L],
                    vec![L3, U3, L, U2, L3, U, L],
                    vec![L3, U, L, U3, L3, U3, L],
                    vec![L3, U, L, U2, F, U, F3],
                    vec![L3, U2, L2, F3, L3, F],
                    vec![L3, U2, L, U3, F, U, F3],
                    vec![L3, U2, L, U2, L3, U, L],
                    vec![L3, U2, L, U, L3, U3, L],
                    vec![L2, U2, L, U, L3, U, L2],
                    vec![L3, U3, L, U3, F, U3, F3, U, F, U3, F3],
                ],
            );
            for mut put_act in put_acts[&self.pair].clone() {
                self.cube = self.cube.apply_moves(&put_act);
                if self.get_pair() == estimated {
                    match i {
                        1 => {
                            combine.push(U);
                        }
                        2 => {
                            combine.push(U2);
                        }
                        3 => {
                            combine.push(U3);
                        }
                        _ => {}
                    }
                    combine.append(&mut put_act);
                    return combine;
                }
                let put_rev = Formula { moves: put_act }.inverse().moves;
                self.cube = self.cube.apply_moves(&put_rev);
            }
            match i {
                1 => {
                    self.cube = self.cube.apply_move(U3);
                }
                2 => {
                    self.cube = self.cube.apply_move(U2);
                }
                3 => {
                    self.cube = self.cube.apply_move(U);
                }
                _ => {}
            }
        }
        Vec::new()
    }

    /// Check if the pair is solved.
    fn is_solved(&self) -> bool {
        self.get_pair() == self.estimated_position()
    }
}

#[cfg(test)]
mod tests {
    use super::F2LSolver;
    use crate::cubie::CubieCube;
    use crate::moves::Move::*;
    use crate::solver::cfop::cross::CrossSolver;

    #[test]
    fn test_f2lpair() {
        let cc = CubieCube::default();
        let moves = vec![L2, R, F2, L, D, U, R, D, B3];
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver { cube: cc };
        let _c = cross.solve();
        println!("{:?}", _c);
        let cc = cross.cube.clone();
        let mut f2l = F2LSolver { cube: cc };
        let _f = f2l.solve();
        println!("{:?}", _f);
    }
}
