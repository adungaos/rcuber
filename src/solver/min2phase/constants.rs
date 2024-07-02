use crate::cubie::Corner::{self, *};
use crate::cubie::Edge::{self, *};
use crate::facelet::Color;
use crate::moves::Move::{self, *};

pub const ALL_CORNERS: [Corner; 8] = [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB];
pub const ALL_EDGES: [Edge; 12] = [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR];
pub const ALL_COLORS: [Color; 6] = [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B];
pub const ALL_MOVES: [Move; 18] = [
    U, U2, U3, R, R2, R3, F, F2, F3, D, D2, D3, L, L2, L3, B, B2, B3,
];

pub const SOLVED: u16 = 0;

pub const SYM_E2C_MAGIC: usize = 0x00DDDD00;
pub const N_PERM_4: usize = 24;
pub const N_MPERM: usize = 24;
pub const N_COMB: usize = 140;
// pub const N_COMB: usize = 70;
pub const P2_PARITY_MOVE: usize = 0xA5;

pub const N_CHOOSE_8_4: usize = 70;
/// number of cube symmetries of full group Oh
pub const N_SYM: usize = 48;
/// number of possible face moves
pub const N_MOVES: usize = 18;
pub const N_MOVES2: usize = 10;

/// 3^7 possible corner orientations in phase 1
pub const N_TWIST: usize = 2187;
pub const N_TWIST_SYM: usize = 324;
/// Number of symmetries of subgroup D4h
pub const N_SYM_D4H: usize = 16;
/// 8! permutations of the edges in the U-face and D-face in phase 2
pub const N_UD_EDGES: usize = 40320;
/// 2^11 possible edge orientations in phase 1
pub const N_FLIP: usize = 2048;
pub const N_FLIP_SYM: usize = 336;
/// 12*11*10*9 possible positions of the FR, FL, BL, BR edges in phase 1
pub const N_SLICE_SORTED: usize = 11880;
/// we ignore the permutation of FR, FL, BL, BR in phase 1
pub const N_SLICE: usize = N_SLICE_SORTED / N_PERM_4;
/// number of equivalence classes for combined flip+slice concerning symmetry group D4h
pub const N_FLIPSLICE_CLASS: usize = 64430;
/// 8! corner permutations in phase 2
pub const N_CORNERS: usize = 40320;
pub const N_PERM: usize = 40320;

/// number of equivalence classes concerning symmetry group D4h
pub const N_CORNERS_CLASS: usize = 2768;
pub const N_PERM_SYM: usize = 2768;

/// number of different positions of the edges UR, UF, UL and UB in phase 2
pub const N_U_EDGES_PHASE2: usize = 1680;

pub const USE_TWIST_FLIP_PRUN: bool = true;
//Options for research purpose.
pub const MAX_PRE_MOVES: usize = 20;
pub const TRY_INVERSE: bool = true;
pub const TRY_THREE_AXES: bool = true;

pub const USE_COMBP_PRUN: bool = USE_TWIST_FLIP_PRUN;
pub const USE_CONJ_PRUN: bool = USE_TWIST_FLIP_PRUN;
pub const MIN_P1LENGTH_PRE: usize = 7;
pub const MAX_DEPTH2: usize = 12;

/**
 *     Verbose_Mask determines if a " . " separates the phase1 and phase2 parts of the solver string like in F' R B R L2 F .
 *     U2 U D for example.<br>
 */
pub const USE_SEPARATOR: usize = 0x1;

/**
 *     Verbose_Mask determines if the solution will be inversed to a scramble/state generator.
 */
pub const INVERSE_SOLUTION: usize = 0x2;

/**
 *     Verbose_Mask determines if a tag such as "(21f)" will be appended to the solution.
 */
pub const APPEND_LENGTH: usize = 0x4;

/**
 *     Verbose_Mask determines if guaranteeing the solution to be optimal.
 */
pub const OPTIMAL_SOLUTION: usize = 0x8;

pub const FIRSTTIME_FULL_INIT: bool = false;