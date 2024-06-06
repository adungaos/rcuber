use std::{fmt, str::FromStr};

use self::Move::*;
use crate::cubie::{Corner::*, CubieCube, Edge::*};
use crate::error::Error;
use crate::facelet::Color;

/// Face Turns Moves: Up, Right, Front, Down, Left, Back; 
/// Slice Moves: Slice moves only turn the middle layer, M follows the L direction, E follows the D direction, S follows the F direction.
/// Wide Moves: Wide moves turn 2 layers at once.They can be written in 2 ways:
///     Lower case: u, d, r, l, f, b
///     Ending in w: Uw, Dw, Rw, Lw, Fw, Bw
/// Here use second way.
/// Cube Rotations: x follows the R direction, y follows the U direction, z follows the F direction.
/// 
/// $ clockwise, $2 double, $3 counter-clockwise.
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Move {
    U, U2, U3,
    R, R2, R3,
    F, F2, F3,
    D, D2, D3,
    L, L2, L3,
    B, B2, B3,
    M, M2, M3,
    E, E2, E3,
    S, S2, S3,
    Uw, Uw2, Uw3,
    Rw, Rw2, Rw3,
    Fw, Fw2, Fw3,
    Dw, Dw2, Dw3,
    Lw, Lw2, Lw3,
    Bw, Bw2, Bw3,
    x, x2, x3,
    y, y2, y3,
    z, z2, z3,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            U3 => write!(f, "U'"),
            D3 => write!(f, "D'"),
            R3 => write!(f, "R'"),
            L3 => write!(f, "L'"),
            F3 => write!(f, "F'"),
            B3 => write!(f, "B'"),
            M3 => write!(f, "M'"),
            E3 => write!(f, "E'"),
            S3 => write!(f, "S'"),
            Uw3 => write!(f, "Uw'"),
            Dw3 => write!(f, "Dw'"),
            Rw3 => write!(f, "Rw'"),
            Lw3 => write!(f, "Lw'"),
            Fw3 => write!(f, "Fw'"),
            Bw3 => write!(f, "Bw'"),
            x3 => write!(f, "x'"),
            y3 => write!(f, "y'"),
            z3 => write!(f, "z'"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(R),
            "R'" => Ok(R3),
            "R2" => Ok(R2),
            "L" => Ok(L),
            "L'" => Ok(L3),
            "L2" => Ok(L2),
            "U" => Ok(U),
            "U'" => Ok(U3),
            "U2" => Ok(U2),
            "D" => Ok(D),
            "D'" => Ok(D3),
            "D2" => Ok(D2),
            "F" => Ok(F),
            "F'" => Ok(F3),
            "F2" => Ok(F2),
            "B" => Ok(B),
            "B'" => Ok(B3),
            "B2" => Ok(B2),
            "M" => Ok(M),
            "M'" => Ok(M3),
            "M2" => Ok(M2),
            "E" => Ok(E),
            "E'" => Ok(E3),
            "E2" => Ok(E2),
            "S" => Ok(S),
            "S'" => Ok(S3),
            "S2" => Ok(S2),
            "Rw" => Ok(Rw),
            "Rw'" => Ok(Rw3),
            "Rw2" => Ok(Rw2),
            "Lw" => Ok(Lw),
            "Lw'" => Ok(Lw3),
            "Lw2" => Ok(Lw2),
            "Uw" => Ok(Uw),
            "Uw'" => Ok(Uw3),
            "Uw2" => Ok(Uw2),
            "Dw" => Ok(Dw),
            "Dw'" => Ok(Dw3),
            "Dw2" => Ok(Dw2),
            "Fw" => Ok(Fw),
            "Fw'" => Ok(Fw3),
            "Fw2" => Ok(Fw2),
            "Bw" => Ok(Bw),
            "Bw'" => Ok(Bw3),
            "Bw2" => Ok(Bw2),
            "x" => Ok(x),
            "x'" => Ok(x3),
            "x2" => Ok(x2),
            "y" => Ok(y),
            "y'" => Ok(y3),
            "y2" => Ok(y2),
            "z" => Ok(z),
            "z'" => Ok(z3),
            "z2" => Ok(z2),
            _ => Err(Error::InvalidScramble),
        }
    }
}

#[rustfmt::skip]
impl Move {
    pub fn is_inverse(&self, other: Move) -> bool {
        matches!(
            (&self, other),
            (U | U2 | U3, D | D2 | D3) 
            | (R | R2 | R3, L | L2 | L3) 
            | (F | F2 | F3, B | B2 | B3),
        )
    }

    pub fn is_same_layer(&self, other: Move) -> bool {
        matches!(
            (&self, other),
            (U | U2 | U3, U | U2 | U3)
            | (D | D2 | D3, D | D2 | D3)
            | (R | R2 | R3, R | R2 | R3)
            | (L | L2 | L3, L | L2 | L3)
            | (F | F2 | F3, F | F2 | F3)
            | (B | B2 | B3, B | B2 | B3)
            | (M | M2 | M3, M | M2 | M3)
            | (E | E2 | E3, E | E2 | E3)
            | (S | S2 | S3, S | S2 | S3)
        )
    }

    pub fn get_inverse(self) -> Self {
        match self {
            U => U3,
            U3 => U,
            D => D3,
            D3 => D,
            R => R3,
            R3 => R,
            L => L3,
            L3 => L,
            F => F3,
            F3 => F,
            B => B3,
            B3 => B,
            M => M3,
            M3 => M,
            E => E3,
            E3 => E,
            S => S3,
            S3 => S,
            _ => self,
        }
    }

    pub fn is_counter_clockwise(self) -> bool {
        match self {
            R3|L3|U3|D3|F3|B3|M3|E3|S3|Rw3|Lw3|Uw3|Dw3|Fw3|Bw3|x3|y3|z3 => true,
            _ => false,
        }
    }

    pub fn is_clockwise(self) -> bool {
        !self.is_counter_clockwise()
    }

    pub fn is_180(self) -> bool {
        match self {
            R2|L2|U2|D2|F2|B2|M2|E2|S2|Rw2|Lw2|Uw2|Dw2|Fw2|Bw2|x2|y2|z2 => true,
            _ => false,
        }
    }

}

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// U_MOVE
pub const U_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [UBR, URF, UFL, ULB, DFR, DLF, DBL, DRB],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UB, UR, UF, UL, DR, DF, DL, DB, FR, FL, BL, BR],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// R_MOVE
pub const R_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [DFR, UFL, ULB, URF, DRB, DLF, DBL, UBR], //permutation of the corners
    co: [2, 0, 0, 1, 1, 0, 0, 2],                 //changes of the orientations of the corners
    ep: [FR, UF, UL, UB, BR, DF, DL, DB, DR, FL, BL, UR], //permutation of the edges
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     //changes of the permutations of the edges
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// F_MOVE
pub const F_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [UFL, DLF, ULB, UBR, URF, DFR, DBL, DRB],
    co: [1, 2, 0, 0, 2, 1, 0, 0],
    ep: [UR, FL, UL, UB, DR, FR, DL, DB, UF, DF, BL, BR],
    eo: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// D_MOVE
pub const D_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [URF, UFL, ULB, UBR, DLF, DBL, DRB, DFR],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UR, UF, UL, UB, DF, DL, DB, DR, FR, FL, BL, BR],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// L_MOVE
pub const L_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [URF, ULB, DBL, UBR, DFR, UFL, DLF, DRB],
    co: [0, 1, 2, 0, 0, 2, 1, 0],
    ep: [UR, UF, BL, UB, DR, DF, FL, DB, FR, UL, DL, BR],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// B_MOVE
pub const B_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [URF, UFL, UBR, DRB, DFR, DLF, ULB, DBL],
    co: [0, 0, 1, 2, 0, 0, 2, 1],
    ep: [UR, UF, UL, BR, DR, DF, DL, BL, FR, FL, UB, DB],
    eo: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// M_MOVE
pub const M_MOVE: CubieCube = CubieCube {
    center: [Color::B, Color::R, Color::U, Color::F, Color::L, Color::D],
    cp: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UR, UB, UL, DB, DR, UF, DL, DF, FR, FL, BL, BR],
    eo: [0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
};

/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// E_MOVE
pub const E_MOVE: CubieCube = CubieCube {
    center: [Color::U, Color::F, Color::L, Color::D, Color::B, Color::R],
    cp: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UR, UF, UL, UB, DR, DF, DL, DB, FL, BL, BR, FR],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
};


/// The basic nine cube moves described by permutations and changes in orientation.
/// 
/// S_MOVE
pub const S_MOVE: CubieCube = CubieCube {
    center: [Color::L, Color::U, Color::F, Color::R, Color::D, Color::B],
    cp: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UL, UF, DL, UB, UR, DF, DR, DB, FR, FL, BL, BR],
    eo: [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
};

pub fn inverse_moves(moves: &Vec<Move>) -> Vec<Move> {
    let mut rev = Vec::new();
    for m in moves {
        rev.push(m.get_inverse());
    }
    rev.reverse();
    rev
}