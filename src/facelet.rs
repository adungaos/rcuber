use std::fmt;

use crate::{cubie::CubieCube, error::Error};
use crate::moves::Move;

/// Names the colors of the cube facelets: up, right, front, down, left, back.
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
pub enum Color {
    U, R, F, D, L, B,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<char> for Color {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Color::U),
            'R' => Ok(Color::R),
            'F' => Ok(Color::F),
            'D' => Ok(Color::D),
            'L' => Ok(Color::L),
            'B' => Ok(Color::B),
            _ => Err(Error::InvalidColor),
        }
    }
}

/// Cube on the facelet level.
/// 
/// The facelet representation follows the ordering: U-R-F-D-L-B.
/// 
/// A solved facelet is `UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB`.
/// 
#[derive(Debug, PartialEq)]
pub struct FaceCube {
    pub f: [Color; 54],
}

/// Solved cube on the facelet level.
#[rustfmt::skip]
pub const SOLVED_FACE_CUBE: FaceCube = FaceCube {
    f: [
        Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U,
        Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R,
        Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F,
        Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D,
        Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L,
        Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B,
    ],
};

impl Default for FaceCube {
    fn default() -> Self {
        SOLVED_FACE_CUBE
    }
}

impl TryFrom<&CubieCube> for FaceCube {
    type Error = Error;
    fn try_from(value: &CubieCube) -> Result<Self, Self::Error> {
        // rotate D to D, F to F.
        let mut cc = *value;
        if cc.center[3] != Color::D {
            let mut m = Move::D;
            for i in 0..6 {
                if cc.center[i] == Color::D {
                    m = match i {
                        0 => Move::x2,
                        1 => Move::z,
                        2 => Move::x3,
                        4 => Move::z3,
                        5 => Move::x,
                        _ => Move::D, // should not happen.
                    };
                }
            }
            cc = cc.apply_move(m);
        }
        if cc.center[2] != Color::F {
            let mut m = Move::y;
            for i in 0..6 {
                if cc.center[i] == Color::F {
                    m = match i {
                        1 => Move::y,
                        4 => Move::y3,
                        5 => Move::y2,
                        _ => Move::y, // should not happen.
                    };
                }
            }
            cc = cc.apply_move(m);
        }
        if !cc.is_solvable() {
            return Err(Error::InvalidCubieValue);
        }

        let mut face = FaceCube::default();

        for (i,c) in CENTER_FACELET.iter().enumerate() {
            // println!("facelet: {:?}", *c as usize);
            face.f[*c as usize] = CENTER_COLOR[cc.center[*c as usize - 4 - i * 9 + i] as usize];
        }

        for (i, corner_faces) in CORNER_FACELET.iter().enumerate() {
            let corner = cc.cp[i] as usize;

            for (j, f) in corner_faces.iter().enumerate() {
                face.f[*f as usize] = CORNER_COLOR[corner][(j + (3 - cc.co[i] as usize)) % 3];
            }
        }

        for (i, edge_faces) in EDGE_FACELET.iter().enumerate() {
            let edge = cc.ep[i] as usize;

            for (j, f) in edge_faces.iter().enumerate() {
                face.f[*f as usize] = EDGE_COLOR[edge][(j + cc.eo[i] as usize) % 2];
            }
        }

        Ok(face)
    }
}

impl TryFrom<&str> for FaceCube {
    type Error = Error;
    fn try_from(cube_string: &str) -> Result<Self, Self::Error> {
        if cube_string.len() != 54 {
            return Err(Error::InvalidFaceletString);
        }

        let mut face_cube = FaceCube::default();

        for (i, c) in cube_string.chars().enumerate() {
            face_cube.f[i] = Color::try_from(c)?;
        }

        Ok(face_cube)
    }
}

impl fmt::Display for FaceCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let acc = String::new();
        let s = self.f.iter().fold(acc, |acc, f| format!("{acc}{f}"));

        write!(f, "{s}")
    }
}

/// The names of the facelet positions of the cube.
///
/// ```text
///             |************|
///             |*U1**U2**U3*|
///             |************|
///             |*U4**U5**U6*|
///             |************|
///             |*U7**U8**U9*|
///             |************|
/// ************|************|************|************|
/// *L1**L2**L3*|*F1**F2**F3*|*R1**R2**F3*|*B1**B2**B3*|
/// ************|************|************|************|
/// *L4**L5**L6*|*F4**F5**F6*|*R4**R5**R6*|*B4**B5**B6*|
/// ************|************|************|************|
/// *L7**L8**L9*|*F7**F8**F9*|*R7**R8**R9*|*B7**B8**B9*|
/// ************|************|************|************|
///             |************|
///             |*D1**D2**D3*|
///             |************|
///             |*D4**D5**D6*|
///             |************|
///             |*D7**D8**D9*|
///             |************|
/// ```
/// A cube definition string "UBL..." means for example: In position U1 we have the U-color, in position U2 we have the
/// B-color, in position U3 we have the L color etc. according to the order U1, U2, U3, U4, U5, U6, U7, U8, U9, R1, R2,
/// R3, R4, R5, R6, R7, R8, R9, F1, F2, F3, F4, F5, F6, F7, F8, F9, D1, D2, D3, D4, D5, D6, D7, D8, D9, L1, L2, L3, L4,
/// L5, L6, L7, L8, L9, B1, B2, B3, B4, B5, B6, B7, B8, B9 of the enum constants.
/// 
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Facelet {
    U1, U2, U3, U4, _U5, U6, U7, U8, U9,
    R1, R2, R3, R4, _R5, R6, R7, R8, R9,
    F1, F2, F3, F4, _F5, F6, F7, F8, F9,
    D1, D2, D3, D4, _D5, D6, D7, D8, D9,
    L1, L2, L3, L4, _L5, L6, L7, L8, L9,
    B1, B2, B3, B4, _B5, B6, B7, B8, B9,
}


pub const CENTER_FACELET: [Facelet; 6] = [
    Facelet::_U5, Facelet::_R5, Facelet::_F5, Facelet::_D5, Facelet::_L5, Facelet::_B5,
];

pub const CENTER_COLOR: [Color; 6] = [
    Color::U, Color::R, Color::F, Color::D, Color::L, Color::B,
];

/// Map the corner positions to facelet positions.
pub const CORNER_FACELET: [[Facelet; 3]; 8] = [
    [Facelet::U9, Facelet::R1, Facelet::F3], //URF
    [Facelet::U7, Facelet::F1, Facelet::L3], //UFL
    [Facelet::U1, Facelet::L1, Facelet::B3], //ULB
    [Facelet::U3, Facelet::B1, Facelet::R3], //UBR
    [Facelet::D3, Facelet::F9, Facelet::R7], //DFR
    [Facelet::D1, Facelet::L9, Facelet::F7], //DLF
    [Facelet::D7, Facelet::B9, Facelet::L7], //DBL
    [Facelet::D9, Facelet::R9, Facelet::B7], //DRB
];

/// Map the edge positions to facelet positions.
pub const EDGE_FACELET: [[Facelet; 2]; 12] = [
    [Facelet::U6, Facelet::R2],
    [Facelet::U8, Facelet::F2],
    [Facelet::U4, Facelet::L2],
    [Facelet::U2, Facelet::B2],
    [Facelet::D6, Facelet::R8],
    [Facelet::D2, Facelet::F8],
    [Facelet::D4, Facelet::L8],
    [Facelet::D8, Facelet::B8],
    [Facelet::F6, Facelet::R4],
    [Facelet::F4, Facelet::L6],
    [Facelet::B6, Facelet::L4],
    [Facelet::B4, Facelet::R6],
];

/// Map the corner positions to facelet colors.
pub const CORNER_COLOR: [[Color; 3]; 8] = [
    [Color::U, Color::R, Color::F],
    [Color::U, Color::F, Color::L],
    [Color::U, Color::L, Color::B],
    [Color::U, Color::B, Color::R],
    [Color::D, Color::F, Color::R],
    [Color::D, Color::L, Color::F],
    [Color::D, Color::B, Color::L],
    [Color::D, Color::R, Color::B],
];

/// Map the edge positions to facelet colors.
pub const EDGE_COLOR: [[Color; 2]; 12] = [
    [Color::U, Color::R],
    [Color::U, Color::F],
    [Color::U, Color::L],
    [Color::U, Color::B],
    [Color::D, Color::R],
    [Color::D, Color::F],
    [Color::D, Color::L],
    [Color::D, Color::B],
    [Color::F, Color::R],
    [Color::F, Color::L],
    [Color::B, Color::L],
    [Color::B, Color::R],
];

#[cfg(test)]
mod test {
    use crate::cubie::{Corner::*, Edge::*, SOLVED_CUBIE_CUBE};
    use crate::facelet::*;

    #[test]
    fn test_facelet_to_cubie() {
        // One scramble that produces these faces:
        // F L' B R' U R U B' L2 R' F2 U2 L' F2 D F U R' D R U' L' R2 D2
        let faces = "DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL";
        let face_cube = FaceCube::try_from(faces).unwrap();
        let actual_state = CubieCube::try_from(&face_cube).unwrap();

        assert_eq!(
            actual_state,
            CubieCube {
                center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
                cp: [DRB, URF, DLF, ULB, DFR, UBR, DBL, UFL],
                co: [0, 2, 0, 1, 1, 0, 2, 0],
                ep: [UB, UL, DL, FR, FL, UR, BL, BR, DR, UF, DF, DB],
                eo: [0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0]
            }
        );
        // One list of moves that solves this state:
        // L2 B' D R F B' L U B R' U' B2 D L2 D2 R2 B2 D' B2 D F2 U
    }

    #[test]
    fn test_cubie_to_facelet() {
        let face_cube = FaceCube::try_from(&SOLVED_CUBIE_CUBE).unwrap();

        assert_eq!(face_cube, SOLVED_FACE_CUBE);

        let face_string = "DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL";
        let expected = FaceCube::try_from(face_string).unwrap();
        let cubie = CubieCube {
            center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
            cp: [DRB, URF, DLF, ULB, DFR, UBR, DBL, UFL],
            co: [0, 2, 0, 1, 1, 0, 2, 0],
            ep: [UB, UL, DL, FR, FL, UR, BL, BR, DR, UF, DF, DB],
            eo: [0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0],
        };
        let face_cube = FaceCube::try_from(&cubie).unwrap();

        assert_eq!(face_cube, expected);
    }
}
