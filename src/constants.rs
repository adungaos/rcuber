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

pub const ALL_MOVES_FULL: [Move; 55] = [
    U, U2, U3, R, R2, R3, F, F2, F3, D, D2, D3, L, L2, L3, B, B2, B3, M, M2, M3, E, E2, E3, S, S2,
    S3, Uw, Uw2, Uw3, Rw, Rw2, Rw3, Fw, Fw2, Fw3, Dw, Dw2, Dw3, Lw, Lw2, Lw3, Bw, Bw2, Bw3, x, x2,
    x3, y, y2, y3, z, z2, z3, N,
];
