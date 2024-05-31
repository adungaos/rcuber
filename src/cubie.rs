use std::fmt;
use std::ops::Mul;

use self::{Corner::*, Edge::*, Move::*};
use crate::constants::*;
use crate::error::Error;
use crate::{facelet::*, moves::*};

/// Represents the 8 corners on the cube, described by the layer they are on.
/// 
/// Example: `ULB` (Up, Left, Bottom).
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Corner {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Corner {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(URF),
            1 => Ok(UFL),
            2 => Ok(ULB),
            3 => Ok(UBR),
            4 => Ok(DFR),
            5 => Ok(DLF),
            6 => Ok(DBL),
            7 => Ok(DRB),
            _ => Err(Error::InvalidCorner),
        }
    }
}

/// Represents the 12 edges on the cube, described by the layer they are on.
/// 
/// Example: `BL` (Bottom, Left).
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
pub enum Edge {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Edge {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UR),
            1 => Ok(UF),
            2 => Ok(UL),
            3 => Ok(UB),
            4 => Ok(DR),
            5 => Ok(DF),
            6 => Ok(DL),
            7 => Ok(DB),
            8 => Ok(FR),
            9 => Ok(FL),
            10 => Ok(BL),
            11 => Ok(BR),
            _ => Err(Error::InvalidEdge),
        }
    }
}

impl TryFrom<&str> for Edge {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "UR" => Ok(UR),
            "UF" => Ok(UF),
            "UL" => Ok(UL),
            "UB" => Ok(UB),
            "DR" => Ok(DR),
            "DF" => Ok(DF),
            "DL" => Ok(DL),
            "DB" => Ok(DB),
            "FR" => Ok(FR),
            "FL" => Ok(FL),
            "BL" => Ok(BL),
            "BR" => Ok(BR),
            _ => Err(Error::InvalidEdge),
        }
    }
}

/// Cube on the cubie level.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CubieCube {
    /// Center permutation, relative to SOLVED_STATE.
    pub center: [Color; 6],
    /// Corner permutation, relative to SOLVED_STATE.
    pub cp: [Corner; 8],
    /// Corner orientation, 3 possible values: 0 (correctly oriented), 1 (twisted clockwise), 2 (twisted counter-clockwise).
    pub co: [u8; 8],
    /// Edge permutation, relative to SOLVED_STATE.
    pub ep: [Edge; 12],
    /// Edge orientation, 2 possible values: 0 (correctly oriented), 1 (flipped).
    pub eo: [u8; 12],
}

/// Solved cube on the Cubie level.
pub const SOLVED_CUBIE_CUBE: CubieCube = CubieCube {
    center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
    cp: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

impl Default for CubieCube {
    fn default() -> Self {
        SOLVED_CUBIE_CUBE
    }
}

impl Mul for CubieCube {
    type Output = Self;

    fn mul(self, rhs: CubieCube) -> Self::Output {
        let mut res = CubieCube::default();
        // (A * B).c = A(B(x).c).c
        // (A * B).o = A(B(x).c).o + B(x).o

        for i in 0..8 {
            res.cp[i] = self.cp[rhs.cp[i] as usize];
            res.co[i] = (self.co[rhs.cp[i] as usize] + rhs.co[i]) % 3;
        }

        for i in 0..12 {
            res.ep[i] = self.ep[rhs.ep[i] as usize];
            res.eo[i] = (self.eo[rhs.ep[i] as usize] + rhs.eo[i]) % 2;
        }

        for i in 0..6{
            res.center[i] = self.center[rhs.center[i] as usize];
        }
        res
    }
}

impl fmt::Display for CubieCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print string for a cubie cube.
        let mut s = String::new();
        for i in 0..8 {
            let cs: String = format!("({},{})", self.cp[i], self.co[i]);
            s.push_str(&cs);
        }
        for i in 0..12 {
            let es: String = format!("({},{})", self.ep[i], self.eo[i]);
            s.push_str(&es);
        }
        write!(f, "{s}")
    }
}


impl From<&Vec<Move>> for CubieCube {
    fn from(moves: &Vec<Move>) -> Self {
        CubieCube::default().apply_moves(moves)
    }
}

/// Gives cubie representation of a face cube (facelet).
impl TryFrom<&FaceCube> for CubieCube {
    type Error = Error;
    fn try_from(face_cube: &FaceCube) -> Result<Self, Self::Error> {
        let mut state = CubieCube::default();
        let mut ori: usize = 0;
        let mut col1;
        let mut col2;

        for i in 0..8 {
            let i = Corner::try_from(i)?;
            // get the colors of the cubie at corner i, starting with U/D
            for index in 0..3 {
                ori = index;
                if face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::U
                    || face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::D
                {
                    break;
                }
            }

            col1 = face_cube.f[CORNER_FACELET[i as usize][(ori + 1) % 3] as usize];
            col2 = face_cube.f[CORNER_FACELET[i as usize][(ori + 2) % 3] as usize];

            for j in 0..8 {
                let j = Corner::try_from(j)?;
                if col1 == CORNER_COLOR[j as usize][1] && col2 == CORNER_COLOR[j as usize][2] {
                    // in cornerposition i we have cornercubie j
                    state.cp[i as usize] = j;
                    state.co[i as usize] = ori as u8 % 3;
                    break;
                }
            }
        }

        for i in 0..12 {
            let i = Edge::try_from(i)?;
            for j in 0..12 {
                let j = Edge::try_from(j)?;
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][0]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][1]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 0;
                    break;
                }
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][1]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][0]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 1;
                    break;
                }
            }
        }

        if !state.is_solvable() {
            Err(Error::InvalidFaceletValue)
        } else {
            Ok(state)
        }
    }
}

impl CubieCube {
    /// Applies a move to the current state.
    pub fn apply_move(self, move_name: Move) -> Self {
        let move_state = match move_name {
            U => U_MOVE,
            U2 => U_MOVE * U_MOVE,
            U3 => U_MOVE * U_MOVE * U_MOVE,
            D => D_MOVE,
            D2 => D_MOVE * D_MOVE,
            D3 => D_MOVE * D_MOVE * D_MOVE,
            R => R_MOVE,
            R2 => R_MOVE * R_MOVE,
            R3 => R_MOVE * R_MOVE * R_MOVE,
            L => L_MOVE,
            L2 => L_MOVE * L_MOVE,
            L3 => L_MOVE * L_MOVE * L_MOVE,
            F => F_MOVE,
            F2 => F_MOVE * F_MOVE,
            F3 => F_MOVE * F_MOVE * F_MOVE,
            B => B_MOVE,
            B2 => B_MOVE * B_MOVE,
            B3 => B_MOVE * B_MOVE * B_MOVE,
            M => M_MOVE,
            M2 => M_MOVE * M_MOVE,
            M3 => M_MOVE * M_MOVE * M_MOVE,
            E => E_MOVE,
            E2 => E_MOVE * E_MOVE,
            E3 => E_MOVE * E_MOVE * E_MOVE,
            S => S_MOVE,
            S2 => S_MOVE * S_MOVE,
            S3 => S_MOVE * S_MOVE * S_MOVE,
            Uw => U_MOVE * E_MOVE * E_MOVE * E_MOVE,
            Uw2 => U_MOVE * U_MOVE * E_MOVE * E_MOVE,
            Uw3 => U_MOVE * U_MOVE * U_MOVE * E_MOVE,
            Dw => D_MOVE * E_MOVE,
            Dw2 => D_MOVE * D_MOVE * E_MOVE * E_MOVE,
            Dw3 => D_MOVE * D_MOVE * D_MOVE * E_MOVE * E_MOVE * E_MOVE,
            Rw => R_MOVE * M_MOVE * M_MOVE * M_MOVE,
            Rw2 => R_MOVE * R_MOVE * M_MOVE * M_MOVE,
            Rw3 => R_MOVE * R_MOVE * R_MOVE * M_MOVE,
            Lw => L_MOVE * M_MOVE,
            Lw2 => L_MOVE * L_MOVE * M_MOVE * M_MOVE,
            Lw3 => L_MOVE * L_MOVE * L_MOVE * M_MOVE * M_MOVE * M_MOVE,
            Fw => F_MOVE * S_MOVE,
            Fw2 => F_MOVE * F_MOVE * S_MOVE * S_MOVE,
            Fw3 => F_MOVE * F_MOVE * F_MOVE * S_MOVE * S_MOVE * S_MOVE,
            Bw => B_MOVE * S_MOVE * S_MOVE * S_MOVE,
            Bw2 => B_MOVE * B_MOVE * S_MOVE * S_MOVE,
            Bw3 => B_MOVE * B_MOVE * B_MOVE * S_MOVE,
            x => R_MOVE * M_MOVE * M_MOVE * M_MOVE * L_MOVE * L_MOVE * L_MOVE,
            x2 => R_MOVE * R_MOVE * M_MOVE * M_MOVE * L_MOVE * L_MOVE,
            x3 => R_MOVE * R_MOVE * R_MOVE * M_MOVE * L_MOVE,
            y => U_MOVE * E_MOVE * E_MOVE * E_MOVE * D_MOVE * D_MOVE * D_MOVE,
            y2 => U_MOVE * U_MOVE * E_MOVE * E_MOVE * D_MOVE * D_MOVE,
            y3 => U_MOVE * U_MOVE * U_MOVE * E_MOVE * D_MOVE,
            z => F_MOVE * S_MOVE * B_MOVE * B_MOVE * B_MOVE,
            z2 => F_MOVE * F_MOVE * S_MOVE * S_MOVE * B_MOVE * B_MOVE,
            z3 => F_MOVE * F_MOVE * F_MOVE * S_MOVE * S_MOVE * S_MOVE * B_MOVE,
        };

        self * move_state
    }

    /// Applies the sequence of moves to the current state.
    pub fn apply_moves(&self, moves: &[Move]) -> Self {
        moves.iter().fold(*self, |acc, &m| acc.apply_move(m))
    }

    /// Returns the number of corner twist needed to orient the corners.
    pub fn count_corner_twist(&self) -> u8 {
        self.co.iter().fold(0, |acc, co| acc + ((3 - co) % 3))
    }

    /// Returns the number of edge twist needed to orient the edges.
    pub fn count_edge_twist(&self) -> u8 {
        self.eo.iter().sum()
    }

    /// Returns the number of corner permutations needed to solve the corners.
    pub fn count_corner_perm(&self) -> u8 {
        let mut count = 0;
        let mut cp = self.cp;

        for i in 0..8 {
            if cp[i] as usize != i {
                if let Some(j) = (i + 1..8).find(|&j| cp[j] as usize == i) {
                    cp.swap(i, j);
                    count += 1;
                }
            }
        }

        count
    }

    /// Returns the number of edge permutations needed to solve the edges.
    pub fn count_edge_perm(&self) -> u8 {
        let mut count = 0;
        let mut ep = self.ep;

        for i in 0..12 {
            if ep[i] as usize != i {
                if let Some(j) = (i + 1..12).find(|&j| ep[j] as usize == i) {
                    ep.swap(i, j);
                    count += 1;
                }
            }
        }

        count
    }

    /// Checks if CubieCube is a valid cubie representation.
    pub fn is_solvable(&self) -> bool {
        let c_perm = self.count_corner_perm();
        let e_perm = self.count_edge_perm();
        let c_twist = self.count_corner_twist();
        let e_twist = self.count_edge_twist();
        let has_even_permutation = c_perm % 2 == e_perm % 2;
        let has_valid_twist = c_twist % 3 == 0 && e_twist % 2 == 0;

        has_even_permutation && has_valid_twist
    }

    /// Multiply this cubie cube with another cubie cube b, restricted to the corners.
    pub fn corner_multiply(&mut self, b: CubieCube) {
        let mut c_perm = [URF; 8];
        let mut c_ori = [0; 8];
        let mut ori = 0;
        for ci in ALL_CORNERS {
            let c = ci as usize;
            c_perm[c] = self.cp[b.cp[c] as usize];
            let ori_a = self.co[b.cp[c] as usize];
            let ori_b = b.co[c];
            if ori_a < 3 && ori_b < 3 {
                // two regular cubes
                ori = ori_a + ori_b;
                if ori >= 3 {
                    ori -= 3;
                }
            } else if ori_a < 3 && 3 <= ori_b {
                // cube b is in a mirrored state
                ori = ori_a + ori_b;
                if ori >= 6 {
                    ori -= 3; // the composition also is in a mirrored state
                }
            } else if ori_a >= 3 && 3 > ori_b {
                // cube a is in a mirrored state
                ori = ori_a - ori_b;
                if ori < 3 {
                    ori += 3; // the composition is a mirrored cube
                }
            } else if ori_a >= 3 && ori_b >= 3 {
                // if both cubes are in mirrored states
                if ori_a >= ori_b {
                    ori = ori_a - ori_b;
                } else {
                    ori = ori_b - ori_a;
                    ori = 3 - ori; // the composition is a regular cube
                }
            }
            c_ori[c] = ori;
        }
        for c in ALL_CORNERS {
            let ci = c as usize;
            self.cp[ci] = c_perm[ci];
            self.co[ci] = c_ori[ci];
        }
    }

    /// Return the inverse of this cubiecube.
    pub fn inverse_cubie_cube(&self) -> Self {
        let mut d = CubieCube::default();
        for ei in ALL_EDGES {
            let e: usize = ei as usize;
            d.ep[self.ep[e] as usize] = ei;
        }
        for ei in ALL_EDGES {
            let e: usize = ei as usize;
            d.eo[e] = self.eo[d.ep[e] as usize];
        }

        for ci in ALL_CORNERS {
            let c = ci as usize;
            d.cp[self.cp[c] as usize] = ci;
        }
        for ci in ALL_CORNERS {
            let c = ci as usize;
            let ori = self.co[d.cp[c] as usize];
            if ori >= 3 {
                d.co[c] = ori;
            } else {
                d.co[c] = 3 - ori;
                if d.co[c] == 3 {
                    d.co[c] = 0;
                }
            }
        }
        d
    }

    /// Give the parity of the corner permutation.
    pub fn corner_parity(&self) -> bool {
        let mut s = 0;
        for i in ((URF as usize + 1)..=(DRB as usize)).rev() {
            for j in ((URF as usize)..=(i - 1)).rev() {
                if self.cp[j] > self.cp[i] {
                    s += 1
                }
            }
        }
        (s % 2) == 0
    }

    /// Give the parity of the edge permutation. A solvable cube has the same corner and edge parity.
    pub fn edge_parity(&self) -> bool {
        let mut s = 0;
        for i in ((UR as usize + 1)..=(BR as usize)).rev() {
            for j in ((UR as usize)..=(i - 1)).rev() {
                if self.ep[j] > self.ep[i] {
                    s += 1;
                }
            }
        }
        (s % 2) == 0
    }

    /// Check if cubiecube is valid.
    pub fn verify(&self) -> Result<bool, Error> {
        let mut edge_count = [0; 12];
        for i in ALL_EDGES {
            edge_count[self.ep[i as usize] as usize] += 1;
        }
        for i in ALL_EDGES {
            if edge_count[i as usize] != 1 {
                return Err(Error::InvalidEdge);
            }
        }
        let mut s = 0;
        for i in ALL_EDGES {
            s += self.eo[i as usize];
        }
        if s % 2 != 0 {
            return Err(Error::InvalidEdge);
        }

        let mut corner_count = [0; 8];
        for i in ALL_CORNERS {
            corner_count[self.cp[i as usize] as usize] += 1;
        }
        for i in ALL_CORNERS {
            if corner_count[i as usize] != 1 {
                return Err(Error::InvalidCorner);
            }
        }
        let mut s = 0;
        for i in ALL_CORNERS {
            s += self.co[i as usize];
        }
        if s % 3 != 0 {
            return Err(Error::InvalidCorner);
        }

        if self.edge_parity() != self.corner_parity() {
            return Err(Error::InvalidCubieValue);
        }
        Ok(true)
    }

    pub fn get_edges_d(&self) -> Vec<(Edge, u8, u8)>{
        let mut i:u8 = 0;
        let mut result = Vec::new();
        for e in self.ep {
            match e {
                DR|DF|DL|DB => result.push((e, i, self.eo[i as usize])),
                _ => {},
            }
            i += 1;
        }
        result
    }
}


#[cfg(test)]
mod test {
    use crate::cubie::*;
    use crate::printer::print_facelet;

    #[test]
    fn test_eq() {
        let state = CubieCube::default();
        let state2 = CubieCube::default();
        assert_eq!(state, state2);
    }

    #[test]
    fn test_inverse() {
        let state = CubieCube {
            center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
            cp: [DLF, ULB, DBL, DRB, UBR, UFL, DFR, URF],
            co: [2, 1, 2, 1, 2, 2, 0, 2],
            ep: [BR, BL, UB, UR, DR, FR, FL, UF, DF, DL, DB, UL],
            eo: [1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1],
        };
        let ic = state.inverse_cubie_cube();
        let d = CubieCube {
            center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
            cp: [DRB, DLF, UFL, DFR, DBL, URF, ULB, UBR],
            co: [1, 1, 2, 1, 0, 1, 1, 2],
            ep: [UB, DB, BR, UL, DR, FR, FL, BL, DF, DL, UF, UR],
            eo: [0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1],
        };
        assert_eq!(ic, d);
        let d2 = ic.inverse_cubie_cube();
        assert_eq!(state, d2);
    }

    #[test]
    fn test_parity() {
        let state = CubieCube::default();

        assert_eq!(state.corner_parity(), true);
        assert_eq!(state.edge_parity(), true);

        let state = CubieCube::from(&vec![R, U, R3, U3, R3, F, R, F3]);

        assert_eq!(state.corner_parity(), true);
        assert_eq!(state.edge_parity(), true);
    }

    #[test]
    fn test_mult() {
        let state = CubieCube::default().apply_move(R);
        assert_eq!(state, R_MOVE);

        let r2_state = CubieCube::default().apply_move(R).apply_move(R);
        assert_eq!(r2_state, R_MOVE * R_MOVE);

        let r3_state = r2_state.apply_move(R);
        assert_eq!(r3_state, r2_state * R_MOVE);

        let fr_state = CubieCube {
            center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
            //URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
            cp: [URF, DLF, ULB, UFL, DRB, DFR, DBL, UBR],
            co: [1, 2, 0, 2, 1, 1, 0, 2],
            ep: [UF, FL, UL, UB, BR, FR, DL, DB, DR, DF, BL, UR],
            eo: [1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        };

        assert_eq!(F_MOVE * R_MOVE, fr_state);
    }

    #[test]
    fn test_move_mes() {
        let moves: Vec<Move> = vec![
            M2, E2, S2,
        ];
        let state = CubieCube::default().apply_moves(&moves);
        let fc = FaceCube::try_from(&state).unwrap();
        let _ = print_facelet(&fc);
        assert_eq!(state, M_MOVE * M_MOVE * E_MOVE * E_MOVE * S_MOVE * S_MOVE);
    }

    #[test]
    fn test_moves() {
        let moves = vec![
            R, U, R3, U3, M, S, E, M, x, R, U, R3, U3, L, F, R
        ];
        let state = CubieCube::default().apply_moves(&moves);
        let fc = FaceCube::try_from(&state).unwrap();
        let _ = print_facelet(&fc);
    }

    #[test]
    fn test_get_edges_d () {
        let cc = CubieCube::default();
        let cc = cc.apply_moves(&vec![R, U, R3 ,U3, F, L]);
        let d_edges = cc.get_edges_d();
        println!("{:?}", d_edges);
    }


    #[test]
    fn test_move_sequence() {
        // (R U R' U') * 6
        let moves = vec![
            R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3,
        ];
        let state = CubieCube::default().apply_moves(&moves);

        assert_eq!(state, SOLVED_CUBIE_CUBE);
    }

    #[test]
    fn test_scramble() {
        // U F' D' F2 D B2 D' R2 U' F2 R2 D2 R2 U' L B L R F' D B'
        let scramble = vec![
            U, F3, D3, F2, D, B2, D3, R2, U3, F2, R2, D2, R2, U3, L, B, L, R, F3, D, B3,
        ];
        let state = CubieCube::default().apply_moves(&scramble);

        let expected = CubieCube {
            center: [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B],
            cp: [DFR, UBR, DLF, ULB, DRB, UFL, URF, DBL],
            co: [2, 0, 1, 2, 0, 0, 2, 2],
            ep: [DF, UB, FL, BL, BR, UL, DR, FR, DL, DB, UF, UR],
            eo: [1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1],
        };

        assert_eq!(state, expected);
    }

    #[test]
    fn test_perm_count() {
        let state = CubieCube::default();

        assert_eq!(state.count_corner_perm(), 0);
        assert_eq!(state.count_edge_perm(), 0);

        let state = CubieCube::from(&vec![R, U, R3, U3]);

        assert_eq!(state.count_corner_perm(), 2);
        assert_eq!(state.count_edge_perm(), 2);

        let state = CubieCube::from(&vec![
            R, U3, R3, U3, R, U, R, D, R3, U3, R, D3, R3, U2, R3, U3,
        ]);

        assert_eq!(state.count_corner_perm(), 1);
        assert_eq!(state.count_edge_perm(), 1);
    }

    #[test]
    fn test_twist_count() {
        let state = CubieCube::default();

        assert_eq!(state.count_corner_twist(), 0);
        assert_eq!(state.count_edge_twist(), 0);

        let state = CubieCube::from(&vec![R, U, R3, U3, R3, F, R, F3]);

        assert_eq!(state.count_corner_twist(), 3);
        assert_eq!(state.count_edge_twist(), 2);
    }
}
