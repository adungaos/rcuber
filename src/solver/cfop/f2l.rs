use std::collections::HashMap;
use crate::facelet::{self, FaceCube};
use crate::solver::cfop::cross::edge_to_pos;
use crate::{
    cubie::{Corner, CubieCube, Edge},
    facelet::Color,
    moves::Move::{self, *},
    moves::inverse_moves,
};
use super::cross::edge_to_face;

//* Module for solving Rubik's Cube F2L. */
pub struct F2LSolver {
    pub cube: CubieCube,
}

impl F2LSolver {
    /// Solve the entire F2L.
    pub fn solve(&mut self) -> Vec<Vec<([facelet::Color; 2], Vec<Move>)>> {
        let mut solution = Vec::new();
        for i in 0..4 {
            for slot in [
                [Color::F, Color::R],
                [Color::F, Color::L],
                [Color::B, Color::R],
                [Color::B, Color::L],
            ] {
                let mut solver = F2LPairSolver {
                    cube: self.cube,
                    pair: slot,
                };
                let mut result = Vec::new(); // ((Color, Color), Vec<Move>);
                if !solver.is_solved() {
                    let pair_result = solver.solve();
                    self.cube = self.cube.apply_moves(&pair_result);
                    result.push((slot, pair_result));
                }
                solution.push(result);
            }
        }
        solution
    }

    /// Check if Cube's F2L is solved.
    pub fn is_solved(self) -> bool {
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

#[derive(Debug, PartialEq)]
pub enum SlotType {
    SOLVED,
    SLOTFREE,
    CSLOTFREE,
    ESLOTFREE,
    DIFFSLOT,
    WRONGSLOT,
}

pub struct F2LPairSolver {
    pub cube: CubieCube,
    pub pair: [Color; 2],
}

impl F2LPairSolver {
    /// Get the F2L pair (corner, edge).
    pub fn get_pair(&self) -> ((Corner, u8, u8), (Edge, u8, u8)) {
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
        // println!("Pair, cp: {:?}, ep: {:?}", self.cube.cp, self.cube.ep);
        ((corner, cp as u8, co), (edge, ep as u8, eo))
    }

    /// Get the estimated cubie of solved pair.
    pub fn estimated_position(&self) -> ((Corner, u8, u8), (Edge, u8, u8)) {
        let pair = self.get_pair();
        ((pair.0 .0, pair.0.0 as u8, 0), (pair.1 .0, pair.1.0 as u8, 0))
    }

    /// Get the slot position of this pair.
    pub fn get_slot(
        &self,
    ) -> (
        SlotType,
        ([Color; 2], [Color; 2]),
        ((Corner, u8, u8), (Edge, u8, u8)),
    ) {
        let (corner, edge) = self.get_pair();
        let cc = CubieCube::default();
        let cp = cc.cp[corner.1 as usize];
        let cp = format!("{:?}", cp).replace("D", "");
        // println!("cp_D: {:?}", cp);
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
            return (SlotType::SLOTFREE, (cp, ep), (corner, edge));
        }
        if cp.contains(&Color::U) {
            return (SlotType::CSLOTFREE, (cp, ep), (corner, edge));
        }
        if ep.contains(&Color::U) {
            return (SlotType::ESLOTFREE, (cp, ep), (corner, edge));
        }
        let mut ep_r = ep.clone();
        ep_r.reverse();
        let eps = [ep, ep_r];
        if !eps.contains(&cp) {
            return (SlotType::DIFFSLOT, (cp, ep), (corner, edge));
        }
        if (corner, edge) == self.estimated_position() {
            return (SlotType::SOLVED, (cp, ep), (corner, edge));
        }
        (SlotType::WRONGSLOT, (cp, ep), (corner, edge))
    }

    pub fn combining(&mut self) -> Vec<Move> {
        let mut combine = Vec::new();
        let mut put_acts = HashMap::new();
        put_acts.insert([Color::F, Color::R], [
            vec![R, U, R3],
            vec![R, U3, R3],
            vec![R, U2, R3],
            vec![F3, U, F],
            vec![F3, U3, F],
            vec![F3, U2, F],
        ]);
        put_acts.insert([Color::B, Color::R], [
            vec![R3, U, R],
            vec![R3, U3, R],
            vec![R3, U2, R],
            vec![B, U, B3],
            vec![B, U3, B3],
            vec![B, U2, B3],
        ]);
        put_acts.insert([Color::B, Color::L], [
            vec![B3, U, B],
            vec![B3, U3, B],
            vec![B3, U2, B],
            vec![L, U, L3],
            vec![L, U3, L3],
            vec![L, U2, L3],
        ]);
        put_acts.insert([Color::F, Color::L], [
            vec![F, U, F3],
            vec![F, U3, F3],
            vec![F, U2, F3],
            vec![L3, U, L],
            vec![L3, U3, L],
            vec![L3, U2, L],
        ]);
        let (slot_type, (cp, ep), (corner, edge)) = self.get_slot();
        if slot_type == SlotType::SOLVED || slot_type == SlotType::SLOTFREE  {
            return Vec::new();
        } else if slot_type == SlotType::CSLOTFREE || slot_type == SlotType::WRONGSLOT {
            let _edge = edge_to_pos(edge);
            let slot = [_edge[0].0, _edge[1].0];
            for i in 0..4 {
                match i {
                    1 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    3 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let (slot_type, _,_) = self.get_slot();
                    if slot_type == SlotType::SLOTFREE {
                        match i {
                            1 => {combine.push(U);},
                            2 => {combine.push(U2);},
                            3 => {combine.push(U3);},
                            _ => {},
                        }
                        combine.append(&mut act);
                        return combine;
                    }
                    let act_rev = inverse_moves(&act);
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    1 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
            }
        } else if slot_type == SlotType::ESLOTFREE {
            let mut _corner = corner_to_pos(corner);
            let mut slot = Vec::new();
            for (k, v)  in _corner {
                if k == Color::D {
                    continue;
                } else {
                    slot.push(k);
                }
            }
            println!("slot: {:?}", slot);
            let slot = [slot[0], slot[1]];
            let slot = match slot {
                [Color::R, Color::F] => [Color::F, Color::R],
                [Color::R, Color::B] => [Color::B, Color::R],
                [Color::L, Color::B] => [Color::B, Color::L],
                [Color::L, Color::F] => [Color::F, Color::L],
                _ => slot,
            };
            for i in 0..4 {
                match i {
                    1 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    3 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let (slot_type, _,_) = self.get_slot();
                    if slot_type == SlotType::SLOTFREE {
                        match i {
                            1 => {combine.push(U);},
                            2 => {combine.push(U2);},
                            3 => {combine.push(U3);},
                            _ => {},
                        }
                        combine.append(&mut act);
                        return combine;
                    }
                    let act_rev = inverse_moves(&act);
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    1 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
            }
        } else {
            let _edge = edge_to_pos(edge);
            let slot = [_edge[0].0, _edge[1].0];
            for i in 0..4 {
                match i {
                    1 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    3 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
                for mut act in put_acts[&slot].clone() {
                    self.cube = self.cube.apply_moves(&act);
                    let (slot_type, _,_) = self.get_slot();
                    if slot_type == SlotType::SLOTFREE || slot_type == SlotType::ESLOTFREE {
                        match i {
                            1 => {combine.push(U);},
                            2 => {combine.push(U2);},
                            3 => {combine.push(U3);},
                            _ => {},
                        }
                        combine.append(&mut act);
                        let mut _combine = self.combining();
                        combine.append(&mut _combine);
                        return combine;
                    }
                    let act_rev = inverse_moves(&act);
                    self.cube = self.cube.apply_moves(&act_rev);
                }
                match i {
                    3 => {self.cube = self.cube.apply_move(U);},
                    2 => {self.cube = self.cube.apply_move(U2);},
                    1 => {self.cube = self.cube.apply_move(U3);},
                    _ => {},
                }
            }
        }
        Vec::new()
    }

    /// Check if two Cubies are combined on the U face.
    pub fn combining_goal(state: ((Corner, u8, u8), (Edge, u8, u8))) -> bool {
        let (corner, edge) = state;
        let cc = CubieCube::default();
        let _corner = cc.cp[corner.1 as usize];
        let _edge = cc.ep[edge.1 as usize];
        if corner.1 > 3 || edge.1 > 3 {
            return false;
        }
        // let corner_faces = corner_to_pos(corner);
        // let edge_faces = edge_to_pos(edge);
        // let edge0_in_corner = corner_faces.get(&edge_faces[0].0);
        // let edge1_in_corner = corner_faces.get(&edge_faces[1].0);
        // if edge0_in_corner.is_some() && edge1_in_corner.is_some() {
        //     if edge0_in_corner.unwrap() == &edge_faces[0].1
        //         && edge1_in_corner.unwrap() == &edge_faces[1].1 {
        //         return true;
        //     }
        // } else if corner_faces.contains_key(&edge_faces[0].0)
        //     && corner_faces.contains_key(&edge_faces[1].0)
        // {
        //     return false;
        // }
        // let mut opposite = HashMap::new();
        // opposite.insert(Color::L, Color::R);
        // opposite.insert(Color::R, Color::L);
        // opposite.insert(Color::F, Color::B);
        // opposite.insert(Color::B, Color::F);
        // for (i, (face, color)) in edge_faces.iter().enumerate() {
        //     if face == &Color::U {
        //         if color
        //             != corner_faces
        //                 .get(&opposite[&edge_faces[(i + 1) % 2].0])
        //                 .unwrap()
        //         {
        //             return false;
        //         }
        //     } else {
        //         if color != corner_faces.get(&Color::U).unwrap() {
        //             return false;
        //         }
        //     }
        // }
        true
    }

    /// Simulate the cube rotation by updating the pair.
    pub fn _rotate(
        pair: ((Corner, u8, u8), (Edge, u8, u8)),
        step: Move,
    ) -> ((Corner, u8, u8), (Edge, u8, u8)) {
        let (corner, edge) = pair;
        let mut cc = CubieCube::default();

        let ocp = cc.cp[corner.1 as usize];
        if ocp != corner.0 {
            cc.cp[corner.1 as usize] = corner.0;
            cc.co[corner.1 as usize] = corner.2;
            cc.cp[corner.0 as usize] = ocp;
            cc.co[corner.0 as usize] = corner.2;
        } else {
            cc.co[corner.1 as usize] = corner.2;
        }

        let oep = cc.ep[edge.1 as usize];
        if oep != edge.0 {
            cc.ep[edge.1 as usize] = edge.0;
            cc.eo[edge.1 as usize] = edge.2;
            cc.ep[edge.0 as usize] = oep;
            cc.eo[edge.0 as usize] = edge.2;
        } else {
            cc.eo[edge.1 as usize] = edge.2;
        }

        let cc = cc.apply_move(step);
        let pair = edge_to_face(edge.0);
        let pair = [pair.0, pair.1];
        let f2lps = F2LPairSolver {
            cube: cc,
            pair: pair,
        };
        f2lps.get_pair()
    }

    /// Successors function for finding path of combining F2L pair.
    pub fn combining_successors(
        state: ((Corner, u8, u8), (Edge, u8, u8)),
        last_action: Vec<Move>,
    ) -> Vec<(Vec<Move>, ((Corner, u8, u8), (Edge, u8, u8)))> {
        let (corner, edge) = state;
        let mut u_turns = match last_action.len() {
            1 => Vec::new(),
            _ => vec![vec![U], vec![U2], vec![U3]],
        };
        let mut r_turns = match last_action.contains(&R) {
            true => Vec::new(),
            false => vec![vec![R, U, R3], vec![R, U3, R3], vec![R, U2, R3]],
        };
        let mut f_turns = match last_action.contains(&F) {
            true => Vec::new(),
            false => vec![vec![F3, U, F], vec![F3, U3, F], vec![F3, U2, F]],
        };
        let mut results = Vec::new();
        let mut all_turns = Vec::new();
        all_turns.append(&mut u_turns);
        all_turns.append(&mut r_turns);
        all_turns.append(&mut f_turns);
        for act in all_turns {
            let mut state_new = (corner, edge);
            for step in act.clone() {
                state_new = F2LPairSolver::_rotate(state_new, step);
            }
            results.push((act, state_new));
        }
        results
    }

    /// Searching the path for combining the pair.
    pub fn combining_search(&self) -> Vec<Move> {
        let start = self.get_pair();
        // println!("combining_search {:?}", start);
        let result = a_star_search_f2l(
            start,
            F2LPairSolver::combining_successors,
            |(_c, _e)| 2,
            F2LPairSolver::combining_goal,
        );
        // println!("combining_search result {:?}", result);
        // return sum(path_actions(), Formula())
        result
    }

    /// Setup for some special F2L cases.
    pub fn combining_setup(&self) -> ([Color; 2], Vec<Move>) {
        let (slot_type, (corner_slot, edge_slot), (corner, edge)) = self.get_slot();
        let cycle = [
            [Color::F, Color::R],
            [Color::B, Color::R],
            [Color::B, Color::L],
            [Color::F, Color::L],
        ];
        // println!("Combining_setup {:?}, slot type: {:?}", self.pair, slot_type);
        if slot_type == SlotType::SLOTFREE {
            // let idx = pair_index(self.pair);
            // let step = match idx {
            //     1 => vec!(Move::y),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y3),
            //     _ => Vec::new(),
            // };
            let step = Vec::new();
            return ([Color::F, Color::R], step);
        } else if slot_type == SlotType::CSLOTFREE {
            // let idx = pair_index(edge_slot);
            // let step = match idx {
            //     1 => vec!(Move::y),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y3),
            //     _ => Vec::new(),
            // };
            let step = Vec::new();
            let pair = cycle[pair_index(self.pair) - pair_index(edge_slot)];
            return (pair, step);
        } else if slot_type == SlotType::ESLOTFREE || slot_type == SlotType::WRONGSLOT {
            // let idx = pair_index(corner_slot);
            // let step = match idx {
            //     1 => vec!(Move::y),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y3),
            //     _ => Vec::new(),
            // };
            let step = Vec::new();
            let pair = cycle[pair_index(self.pair) - pair_index(corner_slot)];
            return (pair, step);
        } else if slot_type == SlotType::DIFFSLOT {
            let (corner_slot, edge_slot) = match corner_slot != self.pair {
                true => (edge_slot, corner_slot),
                false => (corner_slot, edge_slot),
            };
            // let idx = pair_index(edge_slot);
            // let mut step = match idx {
            //     1 => vec!(Move::y),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y3),
            //     _ => Vec::new(),
            // };
            let mut setup_acts = HashMap::new();
            setup_acts.insert([Color::F, Color::R],vec![R, U, R3]);
            setup_acts.insert([Color::B, Color::R],vec![R3, U3, R]);
            setup_acts.insert([Color::B, Color::L],vec![L, U3, L3]);
            setup_acts.insert([Color::F, Color::L],vec![F, U, F3]);
            let mut setup = setup_acts[&self.pair].clone();
            // let mut step = Vec::new();
            let mut result = Vec::new();
            result.append(&mut setup);
            // result.push(R);
            // result.push(U);
            // result.push(R3);
            // let idx = pair_index(edge_slot);
            // let mut step = match idx {
            //     1 => vec!(Move::y3),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y),
            //     _ => Vec::new(),
            // };
            // result.append(&mut step);

            // let idx = pair_index(corner_slot);
            // let mut step = match idx {
            //     1 => vec!(Move::y),
            //     2 => vec!(Move::y2),
            //     3 => vec!(Move::y3),
            //     _ => Vec::new(),
            // };
            // result.append(&mut step);
            // if result[-1].face == "y" and result[-2].face == "y"{
            //     result[-2] += result[-1];
            //     del result[-1];
            // }
            let pair = cycle[pair_index(corner_slot) - pair_index(self.pair)];
            // println!("Combining_setup result {:?}, {:?}", pair, result,);
            return (pair, result);
        } else {
            let pair = cycle[4 - pair_index(self.pair)];
            return (pair, Vec::new());
        }
    }

    /// Combine the pair.
    pub fn combine(&mut self) -> Vec<Move> {
        let (_pair, setup) = self.combining_setup();
        // self.pair = pair;
        self.cube = self.cube.apply_moves(&setup);
        let actual = self.combining_search();
        self.cube = self.cube.apply_moves(&actual);
        let mut result = setup.clone();
        let mut a = actual;
        result.append(&mut a);
        result
    }

    /// Solve the pair.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut combine = self.combine();
        // let idx = pair_index(self.pair);
        // let mut put = match idx {
        //     1 => vec!(Move::y),
        //     2 => vec!(Move::y2),
        //     3 => vec!(Move::y3),
        //     _ => Vec::new(),
        // };
        // self.cube.apply_moves(&put);
        // self.pair = [Color::F, Color::R];
        let estimated = self.estimated_position();
        let pair = self.get_pair();
        // println!("Solve estimated: {:?}", estimated);
        let ofc = FaceCube::try_from(&self.cube).unwrap();
        let ofc_str = ofc.to_string();
        for i in 0..4 {
            // println!("S: {:?}", self.cube);
            match i {
                1 => {self.cube = self.cube.apply_move(U);},
                2 => {self.cube = self.cube.apply_move(U2);},
                3 => {self.cube = self.cube.apply_move(U3);},
                _ => {},
            }
            // println!("{i}: {:?}", self.cube);
            let mut put_acts = HashMap::new();
            put_acts.insert([Color::F, Color::R], [
                vec![R, U, R3],
                vec![R, U3, R3],
                vec![R, U2, R3],
                vec![F3, U, F],
                vec![F3, U3, F],
                vec![F3, U2, F],
            ]);
            put_acts.insert([Color::B, Color::R], [
                vec![R3, U, R],
                vec![R3, U3, R],
                vec![R3, U2, R],
                vec![B, U, B3],
                vec![B, U3, B3],
                vec![B, U2, B3],
            ]);
            put_acts.insert([Color::B, Color::L], [
                vec![B3, U, B],
                vec![B3, U3, B],
                vec![B3, U2, B],
                vec![L, U, L3],
                vec![L, U3, L3],
                vec![L, U2, L3],
            ]);
            put_acts.insert([Color::F, Color::L], [
                vec![F, U, F3],
                vec![F, U3, F3],
                vec![F, U2, F3],
                vec![L3, U, L],
                vec![L3, U3, L],
                vec![L3, U2, L],
            ]);
            for mut put_act in put_acts[&self.pair].clone() {
                self.cube = self.cube.apply_moves(&put_act);
                // println!("P: {:?}", self.cube);
                let fc = FaceCube::try_from(&self.cube).unwrap();
                let fc_str = fc.to_string();
                // println!("OFC: {}, \nFC : {}", ofc_str, fc_str);
                // println!("Put act, {} {:?},Solve estimated: {:?}, opair: {:?}, u_pair: {:?}, pair: {:?}",i, &put_act, estimated, pair, u_pair, self.get_pair());
                if self.get_pair() == estimated {
                    // combine.append(&mut put);
                    match i {
                        1 => {combine.push(U);},
                        2 => {combine.push(U2);},
                        3 => {combine.push(U3);},
                        _ => {},
                    }
                    combine.append(&mut put_act);
                    println!("Combine {:?} successful, {:?}",self.pair, combine);
                    return combine;
                }
                let put_rev = inverse_moves(&put_act);
                // println!("Put: {:?}, RPUT: {:?}", put_act, put_rev);
                self.cube = self.cube.apply_moves(&put_rev);
            }
            match i {
                1 => {self.cube = self.cube.apply_move(U3);},
                2 => {self.cube = self.cube.apply_move(U2);},
                3 => {self.cube = self.cube.apply_move(U);},
                _ => {},
            }
        }
        Vec::new()
    }

    
    /// Solve the pair.
    pub fn solve_pair(&mut self) -> Vec<Move> {
        let mut combine = self.combining();
        let estimated = self.estimated_position();
        // println!("Solve estimated: {:?}", estimated);
        for i in 0..4 {
            match i {
                1 => {self.cube = self.cube.apply_move(U);},
                2 => {self.cube = self.cube.apply_move(U2);},
                3 => {self.cube = self.cube.apply_move(U3);},
                _ => {},
            }
            let mut put_acts = HashMap::new();
            put_acts.insert([Color::F, Color::R], vec![
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
            ]);
            put_acts.insert([Color::B, Color::R], vec![
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
            ]);
            put_acts.insert([Color::B, Color::L], vec![
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
            ]);
            put_acts.insert([Color::F, Color::L], vec![
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
            ]);
            for mut put_act in put_acts[&self.pair].clone() {
                self.cube = self.cube.apply_moves(&put_act);
                if self.get_pair() == estimated {
                    match i {
                        1 => {combine.push(U);},
                        2 => {combine.push(U2);},
                        3 => {combine.push(U3);},
                        _ => {},
                    }
                    combine.append(&mut put_act);
                    println!("Combine {:?} successful, {:?}",self.pair, combine);
                    return combine;
                }
                let put_rev = inverse_moves(&put_act);
                self.cube = self.cube.apply_moves(&put_rev);
            }
            match i {
                1 => {self.cube = self.cube.apply_move(U3);},
                2 => {self.cube = self.cube.apply_move(U2);},
                3 => {self.cube = self.cube.apply_move(U);},
                _ => {},
            }
        }
        Vec::new()
    }

    /// Check if the cube is solved.
    pub fn is_solved(&self) -> bool {
        self.get_pair() == self.estimated_position()
    }
}

pub fn pair_index(pair: [Color; 2]) -> usize {
    let cycle = [
        [Color::F, Color::R],
        [Color::B, Color::R],
        [Color::B, Color::L],
        [Color::F, Color::L],
    ];
    let mut idx = 0;
    for i in 0..4 {
        if cycle[i] == pair {
            idx = i;
        }
    }
    idx
}

/// This is a searching function of A* for f2l.
pub fn a_star_search_f2l<'a, S, V, G>(
    start: ((Corner, u8, u8), (Edge, u8, u8)),
    successors: S,
    state_value: V,
    is_goal: G,
) -> Vec<Move>
where
    S: Fn(
        ((Corner, u8, u8), (Edge, u8, u8)),
        Vec<Move>,
    ) -> Vec<(Vec<Move>, ((Corner, u8, u8), (Edge, u8, u8)))>,
    V: Fn(((Corner, u8, u8), (Edge, u8, u8))) -> u32,
    G: Fn(((Corner, u8, u8), (Edge, u8, u8))) -> bool,
{
    if is_goal(start.clone()) {
        return Vec::new();
    }
    let mut explored = Vec::new();
    let h = state_value(start.clone());
    let g = 1;
    let f = g + h;
    let p = vec![(Vec::new(), start)];
    let mut frontier = Vec::new();
    frontier.push((f, g, h, p));
    while frontier.len() > 0 {
        let (_f, g, _h, path) = frontier.remove(0);
        let s = path.last().unwrap();
        let la = &s.0;
        for (action, state) in successors(s.1.clone(), la.clone()) {
            if !explored.contains(&state) {
                explored.push(state.clone());
                let mut path2 = path.clone();
                path2.push((action, state.clone()));
                if is_goal(state.clone()) {
                    let mut r = Vec::new();
                    for (mut a, _s) in path2 {
                        r.append(&mut a);
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

#[cfg(test)]
mod tests {
    use super::{F2LPairSolver, F2LSolver};
    use crate::cubie::{Corner, CubieCube, Edge};
    use crate::facelet::{Color, FaceCube, Facelet};
    use crate::printer::print_facelet;
    use crate::solver::cfop::f2l::corner_to_pos;
    use crate::Move::*;

    #[test]
    fn test_f2lpair() {
        let cc = CubieCube::default();
        let moves = vec![L2, R, F2, L, D, U, R];
        let cc = cc.apply_moves(&moves);
        let solution = vec![U3, D, F2, D2, R3, F2, D2];
        let cc = cc.apply_moves(&solution);
        // let fc = FaceCube::try_from(&cc).unwrap();
        // let _r = print_facelet(&fc).unwrap();

        for p in [
            [Color::F, Color::R],
            [Color::B, Color::R],
            [Color::B, Color::L],
            [Color::F, Color::L],
        ] {
            let mut solver = F2LPairSolver {
                cube: cc,
                pair: p,
            };
            let slot = solver.get_slot();
            println!("Slot: {:?}", slot);
            let _c = solver.combining();
            println!("Combining result: {:?}", _c);

            // let combining = solver.combining_setup();
            // println!("Combining result: {:?}", combining);
            // let pair = solver.get_pair();
            // println!("Combining pair: {:?}", pair);
            let r = solver.solve_pair();
            println!("Solve result: {:?}", r);
        }
        
        // let fc = FaceCube::try_from(&cc).unwrap();
        // let _r = print_facelet(&fc).unwrap();
        // let mut f2l = F2LSolver { cube: cc };
        // let _s = f2l.solve();
        // println!("{:?}", _s);
        // let fc = FaceCube::try_from(&f2l.cube).unwrap();
        // let _r = print_facelet(&fc).unwrap();
        // let pp = solver.get_pair();
        // println!("{:?}", pp);
        // let pp = F2LPairSolver::_rotate(pp, F);
        // println!("{:?}", pp);
        // let states_new = F2LPairSolver::combining_successors(pp, vec![U]);
        // println!("{:?}", states_new);
        // // assert_eq!(p, ((Corner::DFR,0), (Edge::FR,0)));
        // let s = solver.get_slot();
        // let (_, _, (corner, edge)) = s;
        // println!("{:?}", corner_to_pos(corner));
        // println!("{:?}", s);
        
    }
}
