use std::cmp::max;
use std::{fmt, usize};

use super::arraycube::ArrayCube;
use super::constants::*;
use super::tables::{CT, PT, S2RT, ST};

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
        }
    }
}

impl fmt::Display for CoordCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    }
}

impl CoordCube {
    pub fn set_pruning(table: &mut Vec<i32>, index: usize, value: i32) {
        table[index >> 3] ^= value.wrapping_shl((index as u32) << 2); // index << 2 <=> (index & 7) << 2
    }

    pub fn get_pruning(table: &Vec<i32>, index: usize) -> i32 {
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
