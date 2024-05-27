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

pub const N_PERM_4: usize = 24;
pub const N_CHOOSE_8_4: usize = 70;
/// number of cube symmetries of full group Oh
pub const N_SYM: usize = 48;
/// number of possible face moves
pub const N_MOVE: usize = 18;
/// 3^7 possible corner orientations in phase 1
pub const N_TWIST: usize = 2187;
/// Number of symmetries of subgroup D4h
pub const N_SYM_D4H: usize = 16;
/// 8! permutations of the edges in the U-face and D-face in phase 2
pub const N_UD_EDGES: usize = 40320;
/// 2^11 possible edge orientations in phase 1
pub const N_FLIP: usize = 2048;
/// 12*11*10*9 possible positions of the FR, FL, BL, BR edges in phase 1
pub const N_SLICE_SORTED: usize = 11880;
/// we ignore the permutation of FR, FL, BL, BR in phase 1
pub const N_SLICE: usize = N_SLICE_SORTED / N_PERM_4;
/// number of equivalence classes for combined flip+slice concerning symmetry group D4h
pub const N_FLIPSLICE_CLASS: usize = 64430;
/// 8! corner permutations in phase 2
pub const N_CORNERS: usize = 40320;
/// number of equivalence classes concerning symmetry group D4h
pub const N_CORNERS_CLASS: usize = 2768;
/// number of different positions of the edges UR, UF, UL and UB in phase 2
pub const N_U_EDGES_PHASE2: usize = 1680;
