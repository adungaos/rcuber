use std::cmp::max;
use std::{fmt, usize};

use super::arraycube::ArrayCube;
use super::constants::*;
use super::cubie::CubieCube;
use super::tables::{CT, PT, S2RT, ST};
use crate::error::Error;

/// Represent a cube on the coordinate level.
///
/// In phase 1 a state is uniquely determined by the three coordinates flip, twist and slice = slicesorted / 24.
///
/// In phase 2 a state is uniquely determined by the three coordinates corners, ud_edges and slice_sorted % 24.
///
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CoordCube {
    pub twist: u16, // twist of corners
    pub tsym: u16,
    pub flip: u16, // flip of edges
    pub fsym: u16,
    pub slice: u16,
    pub prun: i32,
    pub twistc: u16,
    pub flipc: u16,
    // pub slice_sorted: u16, // Position of FR, FL, BL, BR edges. Valid in phase 1 (<11880) and phase 2 (<24)
    // // The phase 1 slice coordinate is given by slice_sorted / 24
    // pub u_edges: u16, // Valid in phase 1 (<11880) and phase 2 (<1680). 1656 is the index of solved u_edges.
    // pub d_edges: u16, // Valid in phase 1 (<11880) and phase 2 (<1680)
    // pub corners: u16, // corner permutation. Valid in phase1 and phase2
    // pub ud_edges: u16, // permutation of the ud-edges. Valid only in phase 2
    // pub flipslice_classidx: u16, // symmetry reduced flipslice coordinate used in phase 1
    // pub flipslice_sym: u8,
    // pub flipslice_rep: u32,
    // pub corner_classidx: u16, // symmetry reduced corner permutation coordinate used in phase 2
    // pub corner_sym: u8,
    // pub corner_rep: u16,
}

impl Default for CoordCube {
    fn default() -> Self {
        Self {
            twist: 0,
            tsym: 0,
            flip: 0,
            fsym: 0,
            slice: 0,
            prun: 0,
            twistc: 0,
            flipc: 0,
            // slice_sorted: 0,
            // u_edges: 1656,
            // d_edges: 0,
            // corners: 0,
            // ud_edges: 0,
            // flipslice_classidx: 0,
            // flipslice_sym: 0,
            // flipslice_rep: 0,
            // corner_classidx: 0,
            // corner_sym: 0,
            // corner_rep: 0,
        }
    }
}

impl fmt::Display for CoordCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{:?}", self)
        write!(
            f,
            "(twist: {}, tsym: {}, flip: {}, fsym: {}, slice: {}, prun: {}, twistc: {}, flipc: {})",
            self.twist,
            self.tsym,
            self.flip,
            self.fsym,
            self.slice,
            self.prun,
            self.twistc,
            self.flipc
        )
        // self.twist, self.flip, self.slice_sorted / 24, self.u_edges, self.d_edges, self.slice_sorted, self.corners, self.ud_edges,
        // self.flipslice_classidx, self.flipslice_sym, self.flipslice_rep, self.corner_classidx, self.corner_sym, self.corner_rep)
    }
}

impl CoordCube {
    /// Build a CoordCube from CubieCube(cc).
    ///
    /// Because `TryFrom(fn try_from)` can only take one argument, but we reference SymmetriesTables, so create this function.
    pub fn from_cubie(_cc: &CubieCube) -> Result<Self, Error> {
        todo!()
    }
    //     if !cc.is_solvable() {
    //         return Err(Error::InvalidCubieValue);
    //     }

    //     let twist = cc.get_twist();
    //     let flip = cc.get_flip();
    //     let slice_sorted = cc.get_slice_sorted();
    //     let u_edges = cc.get_u_edges();
    //     let d_edges = cc.get_d_edges();
    //     let corners = cc.get_corners();
    //     let ud_edges;

    //     let flipslice_classidx =
    //         sy.flipslice_classidx[N_FLIP * (slice_sorted as usize / N_PERM_4) + flip as usize];
    //     let flipslice_sym =
    //         sy.flipslice_sym[N_FLIP * (slice_sorted as usize / N_PERM_4) + flip as usize];
    //     let flipslice_rep = sy.flipslice_rep[flipslice_classidx as usize];
    //     let corner_classidx = sy.corner_classidx[corners as usize];
    //     let corner_sym = sy.corner_sym[corners as usize];
    //     let corner_rep = sy.corner_rep[corner_classidx as usize];

    //     if slice_sorted < N_PERM_4 as u16 {
    //         // phase 2 cube
    //         ud_edges = cc.get_ud_edges();
    //     } else {
    //         ud_edges = 65535; // invalid
    //     }
    //     Ok(Self {
    //         twist: twist,
    //         flip: flip,
    //         slice_sorted: slice_sorted,
    //         u_edges: u_edges,
    //         d_edges: d_edges,
    //         corners: corners,
    //         ud_edges: ud_edges,
    //         flipslice_classidx: flipslice_classidx,
    //         flipslice_sym: flipslice_sym,
    //         flipslice_rep: flipslice_rep,
    //         corner_classidx: corner_classidx,
    //         corner_sym: corner_sym,
    //         corner_rep: corner_rep,
    //     })
    // }

    pub fn set_pruning(table: &mut Vec<i32>, index: usize, value: i32) {
        table[index >> 3] ^= value.wrapping_shl((index as u32) << 2); // index << 2 <=> (index & 7) << 2
    }

    pub fn get_pruning(table: &Vec<i32>, index: usize) -> i32 {
        // println!("get_pruning index:{index}");
        table[index >> 3].wrapping_shr((index as u32) << 2) & 0xf
    }

    pub fn has_zero(val: i32) -> bool {
        ((val.wrapping_sub(0x11111111)) & !val & 0x88888888u32 as i32) != 0
    }

    pub fn calc_pruning(&mut self, is_phase1: bool) {
        let _ = is_phase1;
        self.prun = max(
            max(
                CoordCube::get_pruning(
                    &PT.udslice_twist_prun,
                    self.twist as usize * N_SLICE
                        + &CT.udslice_conj[self.slice as usize][self.tsym as usize].into(),
                ),
                CoordCube::get_pruning(
                    &PT.udslice_flip_prun,
                    self.flip as usize * N_SLICE
                        + CT.udslice_conj[self.slice as usize][self.fsym as usize] as usize,
                ),
            ),
            max(
                CoordCube::get_pruning(
                    &PT.twist_flip_prun,
                    (self.twistc as usize >> 3) << 11
                        | S2RT.flip_s2rf[self.flipc as usize ^ (self.twistc as usize & 7)] as usize,
                ),
                CoordCube::get_pruning(
                    &PT.twist_flip_prun,
                    (self.twist as usize) << 11
                        | S2RT.flip_s2rf[(self.flip << 3 | (self.fsym ^ self.tsym)) as usize]
                            as usize,
                ),
            ),
        );
    }

    pub fn set_with_prun(&mut self, cc: ArrayCube, depth: i32) -> bool {
        self.twist = cc.get_twist_sym();
        self.flip = cc.get_flip_sym();
        self.tsym = self.twist & 7;
        self.twist = self.twist >> 3;

        self.prun = CoordCube::get_pruning(
            &PT.twist_flip_prun,
            (self.twist as usize) << 11 | S2RT.flip_s2rf[(self.flip ^ self.tsym) as usize] as usize,
        );
        if self.prun > depth {
            return false;
        }

        self.fsym = self.flip & 7;
        self.flip = self.flip >> 3;

        self.slice = cc.get_ud_slice();
        self.prun = max(
            self.prun,
            max(
                CoordCube::get_pruning(
                    &PT.udslice_twist_prun,
                    self.twist as usize * N_SLICE
                        + CT.udslice_conj[self.slice as usize][self.tsym as usize] as usize,
                ),
                CoordCube::get_pruning(
                    &PT.udslice_flip_prun,
                    self.flip as usize * N_SLICE
                        + CT.udslice_conj[self.slice as usize][self.fsym as usize] as usize,
                ),
            ),
        );
        if self.prun > depth {
            return false;
        }

        let mut pc = ArrayCube::default();
        pc.ca = cc.corner_conjugate(1).ca;
        pc.ea = cc.edge_conjugate(1).ea;

        self.twistc = pc.get_twist_sym();
        self.flipc = pc.get_flip_sym();
        self.prun = max(
            self.prun,
            CoordCube::get_pruning(
                &PT.twist_flip_prun,
                ((self.twistc >> 3) as usize) << 11
                    | S2RT.flip_s2rf[(self.flipc ^ (self.twistc & 7)) as usize] as usize,
            ),
        );
        // println!("set_with_prun::prun:{}:depth:{}", self.prun, depth);
        self.prun <= depth
    }

    pub fn do_move_prun(&mut self, cc: CoordCube, m: usize, is_phase1: bool) -> i32 {
        let _ = is_phase1;
        self.slice = CT.udslice_move[cc.slice as usize][m];

        self.flip =
            CT.flip_move[cc.flip as usize][ST.sym8_move[m << 3 | cc.fsym as usize] as usize];
        self.fsym = (self.flip & 7) ^ cc.fsym;
        self.flip >>= 3;

        self.twist =
            CT.twist_move[cc.twist as usize][ST.sym8_move[m << 3 | cc.tsym as usize] as usize];
        self.tsym = (self.twist & 7) ^ cc.tsym;
        self.twist >>= 3;

        self.prun = max(
            max(
                CoordCube::get_pruning(
                    &PT.udslice_twist_prun,
                    self.twist as usize * N_SLICE
                        + CT.udslice_conj[self.slice as usize][self.tsym as usize] as usize,
                ),
                CoordCube::get_pruning(
                    &PT.udslice_flip_prun,
                    self.flip as usize * N_SLICE
                        + CT.udslice_conj[self.slice as usize][self.fsym as usize] as usize,
                ),
            ),
            CoordCube::get_pruning(
                &PT.twist_flip_prun,
                (self.twist as usize) << 11
                    | S2RT.flip_s2rf[(self.flip as usize) << 3 | (self.fsym ^ self.tsym) as usize]
                        as usize,
            ),
        );
        self.prun
    }

    pub fn do_move_prun_conj(&mut self, cc: CoordCube, m: usize) -> i32 {
        let m = ST.sym_move[3][m] as usize;
        self.flipc = CT.flip_move[(cc.flipc as usize) >> 3]
            [ST.sym8_move[m << 3 | (cc.flipc & 7) as usize] as usize]
            ^ (cc.flipc & 7);
        self.twistc = CT.twist_move[(cc.twistc as usize) >> 3]
            [ST.sym8_move[m << 3 | (cc.twistc & 7) as usize] as usize]
            ^ (cc.twistc & 7);
        CoordCube::get_pruning(
            &PT.twist_flip_prun,
            (self.twistc as usize >> 3) << 11
                | S2RT.flip_s2rf[(self.flipc ^ (self.twistc & 7)) as usize] as usize,
        )
    }
}
