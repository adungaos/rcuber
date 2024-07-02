use rand::random;
use static_init::dynamic;

use super::arraycube::ArrayCube;
use crate::constants::*;
pub use crate::cubie::CubieCube;
use crate::cubie::{
    Corner::*,
    Edge::{self, *},
};
use crate::{facelet::*, moves::*};

impl From<&ArrayCube> for CubieCube {
    fn from(ac: &ArrayCube) -> Self {
        let center = [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B];
        let mut cp = [URF; 8];
        let mut co = [0; 8];
        let mut ep = [UR; 12];
        let mut eo = [0; 12];
        for i in 0..8 {
            cp[i] = ALL_CORNERS[(ac.ca[i] & 0x7) as usize];
            co[i] = ac.ca[i] >> 3;
        }
        for i in 0..12 {
            ep[i] = ALL_EDGES[(ac.ea[i] & 0xf) as usize];
            eo[i] = ac.ea[i] >> 4;
        }
        Self {
            center,
            cp,
            co,
            ep,
            eo,
        }
    }
}

impl CubieCube {
    /// Multiply this cubie cube with another cubie cube b, restricted to the edges.
    pub fn edge_multiply(&mut self, b: CubieCube) {
        let mut e_perm: [Edge; 12] = [UR; 12];
        let mut e_ori = [0; 12];
        for ei in ALL_EDGES {
            let e = ei as usize;
            e_perm[e] = self.ep[b.ep[e] as usize];
            e_ori[e] = (b.eo[e] + self.eo[b.ep[e] as usize]) % 2;
        }
        for ei in ALL_EDGES {
            let e = ei as usize;
            self.ep[e] = e_perm[e];
            self.eo[e] = e_ori[e];
        }
    }

    /// Multiply this cubie cube with another cubie cube b.
    pub fn multiply(&mut self, b: CubieCube) {
        self.corner_multiply(b);
        self.edge_multiply(b);
    }

    /// Multiply this cubie cube with another cubie cube b.
    pub fn multiply_moves(&mut self, moves: &Vec<Move>) {
        moves
            .iter()
            .for_each(|&m| self.multiply(BSCT.bsc[m as usize]));
    }

    /// Get the twist of the 8 corners. 0 <= twist < 2187 in phase 1, twist = 0 in phase 2.
    pub fn get_twist(&self) -> u16 {
        let mut twist: u16 = 0;
        for i in (URF as usize)..(DRB as usize) {
            twist = 3 * twist + self.co[i] as u16;
        }
        twist
    }

    /// Set the twist of the 8 corners. 0 <= twist < 2187 in phase 1, twist = 0 in phase 2.
    pub fn set_twist(&mut self, twist: u16) {
        let mut twistparity = 0;
        let mut twist = twist;
        for i in ((URF as usize)..(DRB as usize)).rev() {
            self.co[i] = (twist % 3) as u8;
            twistparity += self.co[i];
            twist /= 3;
        }
        self.co[DRB as usize] = (3 - twistparity % 3) % 3;
    }

    /// Get the flip of the 12 edges. 0 <= flip < 2048 in phase 1, flip = 0 in phase 2.
    pub fn get_flip(&self) -> u16 {
        let mut ret: u16 = 0;
        for i in (UR as usize)..(BR as usize) {
            ret = 2 * ret + self.eo[i] as u16;
        }
        ret
    }

    /// Set the flip of the 12 edges. 0 <= flip < 2048 in phase 1, flip = 0 in phase 2.
    pub fn set_flip(&mut self, flip: u16) {
        let mut flipparity = 0;
        let mut flip = flip;
        for i in ((UR as usize)..(BR as usize)).rev() {
            self.eo[i] = (flip % 2) as u8;
            flipparity += self.eo[i];
            flip /= 2;
        }
        self.eo[BR as usize] = (2 - flipparity % 2) % 2;
    }

    /// Get the location of the UD-slice edges FR,FL,BL and BR ignoring their permutation.
    ///
    /// 0<= slice < 495 in phase 1, slice = 0 in phase 2.
    pub fn get_slice(&self) -> u16 {
        let mut a = 0;
        let mut _x = 0;
        // Compute the index a < (12 choose 4)
        for j in ((UR as usize)..=(BR as usize)).rev() {
            if FR <= self.ep[j] && self.ep[j] <= BR {
                a += c_nk((11 - j) as u32, _x + 1);
                _x += 1;
            }
        }
        a as u16
    }

    /// Set the location of the UD-slice edges FR,FL,BL and BR ignoring their permutation.
    ///
    /// 0<= slice < 495 in phase 1, slice = 0 in phase 2.
    pub fn set_slice(&mut self, idx: u16) {
        let slice_edge = [FR, FL, BL, BR];
        let other_edge = [UR, UF, UL, UB, DR, DF, DL, DB];
        let mut a = idx; // Location
        let mut ep = [-1; 12];

        let mut _x: i32 = 4; // set slice edges
        for j in ALL_EDGES {
            if a >= c_nk((11 - j as u32) as u32, _x as u32) as u16 {
                self.ep[j as usize] = slice_edge[(4 - _x) as usize];
                ep[j as usize] = slice_edge[(4 - _x) as usize] as i32;
                a -= c_nk(11 - j as u32, _x as u32) as u16;
                _x -= 1;
            }
        }
        let mut _x = 0; // set the remaining edges UR..DB
        for j in ALL_EDGES {
            if ep[j as usize] == -1 {
                self.ep[j as usize] = other_edge[_x];
                _x += 1;
            }
        }
    }

    /// Get the permutation and location of the UD-slice edges FR,FL,BL and BR.
    ///
    /// 0 <= slice_sorted < 11880 in phase 1, 0 <= slice_sorted < 24 in phase 2, slice_sorted = 0 for solved cube.
    pub fn get_slice_sorted(&self) -> u16 {
        let mut a = 0;
        let mut _x = 0;
        let mut edge4 = [UR; 4];
        // First compute the index a < (12 choose 4) and the permutation array perm.
        for j in ((UR as usize)..=(BR as usize)).rev() {
            if FR <= self.ep[j] && self.ep[j] <= BR {
                a += c_nk((11 - j) as u32, _x + 1);
                edge4[(3 - _x) as usize] = self.ep[j as usize];
                _x += 1;
            }
        }
        // Then compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != ALL_EDGES[j + 8] {
                rotate_left(&mut edge4, 0, j);
                k += 1
            }
            b = (j + 1) * b + k;
        }
        24 * a as u16 + b as u16
    }

    /// Set the permutation and location of the UD-slice edges FR,FL,BL and BR.
    ///
    /// 0 <= slice_sorted < 11880 in phase 1, 0 <= slice_sorted < 24 in phase 2, slice_sorted = 0 for solved cube.
    pub fn set_slice_sorted(&mut self, idx: u16) {
        let mut slice_edge = [FR, FL, BL, BR];
        let other_edge = [UR, UF, UL, UB, DR, DF, DL, DB];
        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location
        let mut ep = [-1; 12]; // Invalidate all edge positions

        let mut j = 1; // generate permutation from index b
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j as usize);
                k -= 1;
            }
            j += 1;
        }

        let mut _x = 4; // set slice edges
        for j in ALL_EDGES {
            if a >= c_nk(11 - j as u32, _x) as u16 {
                self.ep[j as usize] = slice_edge[4 - _x as usize];
                ep[j as usize] = slice_edge[4 - _x as usize] as i32;
                a -= c_nk(11 - j as u32, _x) as u16;
                _x -= 1;
            }
        }

        let mut _x = 0; // set the remaining edges UR..DB
        for j in ALL_EDGES {
            if ep[j as usize] == -1 {
                self.ep[j as usize] = other_edge[_x];
                _x += 1;
            }
        }
    }

    /// Get the permutation and location of edges UR, UF, UL and UB.
    ///
    /// 0 <= u_edges < 11880 in phase 1, 0 <= u_edges < 1680 in phase 2, u_edges = 1656 for solved cube.
    pub fn get_u_edges(&self) -> u16 {
        let mut a = 0;
        let mut _x = 0;
        let mut edge4 = [UR; 4];
        let mut ep_mod = self.ep.clone();
        for _j in 0..4 {
            rotate_right(&mut ep_mod, 0, 11);
        }
        // First compute the index a < (12 choose 4) and the permutation array perm.
        for j in ((UR as usize)..=(BR as usize)).rev() {
            if UR <= ep_mod[j] && ep_mod[j] <= UB {
                a += c_nk(11 - j as u32, _x + 1);
                edge4[3 - _x as usize] = ep_mod[j];
                _x += 1;
            }
        }
        // Then compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != ALL_EDGES[j] {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        24 * a as u16 + b as u16
    }

    /// Set the permutation and location of edges UR, UF, UL and UB.
    ///
    /// 0 <= u_edges < 11880 in phase 1, 0 <= u_edges < 1680 in phase 2, u_edges = 1656 for solved cube.
    pub fn set_u_edges(&mut self, idx: u16) {
        let mut slice_edge = [UR, UF, UL, UB];
        let other_edge = [DR, DF, DL, DB, FR, FL, BL, BR];
        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location
        let mut ep = [-1; 12];

        let mut j = 1; // generate permutation from index b
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j as usize);
                k -= 1;
            }
            j += 1;
        }

        let mut _x = 4; // set slice edges
        for j in ALL_EDGES {
            if a >= c_nk(11 - j as u32, _x) as u16 {
                self.ep[j as usize] = slice_edge[4 - _x as usize];
                ep[j as usize] = slice_edge[4 - _x as usize] as i32;
                a -= c_nk(11 - j as u32, _x) as u16;
                _x -= 1;
            }
        }

        let mut _x = 0; // set the remaining edges UR..DB
        for j in ALL_EDGES {
            if ep[j as usize] == -1 {
                self.ep[j as usize] = other_edge[_x];
                _x += 1;
            }
        }
        for _j in 0..4 {
            rotate_left(&mut self.ep, 0, 11);
        }
    }

    /// Get the permutation and location of the edges DR, DF, DL and DB.
    ///
    /// 0 <= d_edges < 11880 in phase 1, 0 <= d_edges < 1680 in phase 2, d_edges = 0 for solved cube.
    pub fn get_d_edges(&self) -> u16 {
        let mut a = 0;
        let mut _x = 0;
        let mut edge4 = [UR; 4];
        let mut ep_mod = self.ep.clone();
        for _j in 0..4 {
            rotate_right(&mut ep_mod, 0, 11);
        }
        // First compute the index a < (12 choose 4) and the permutation array perm.
        for j in ((UR as usize)..=(BR as usize)).rev() {
            if DR <= ep_mod[j] && ep_mod[j] <= DB {
                a += c_nk(11 - j as u32, _x + 1);
                edge4[3 - _x as usize] = ep_mod[j];
                _x += 1;
            }
        }
        // Then compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != ALL_EDGES[j + 4] {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        24 * a as u16 + b as u16
    }

    /// Set the permutation and location of the edges DR, DF, DL and DB.
    ///
    /// 0 <= d_edges < 11880 in phase 1, 0 <= d_edges < 1680 in phase 2, d_edges = 0 for solved cube.
    pub fn set_d_edges(&mut self, idx: u16) {
        let mut slice_edge = [DR, DF, DL, DB];
        let other_edge = [FR, FL, BL, BR, UR, UF, UL, UB];
        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location
        let mut ep = [-1; 12]; // Invalidate all edge positions

        let mut j = 1; // generate permutation from index b
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j as usize);
                k -= 1;
            }
            j += 1;
        }

        let mut _x = 4; // set slice edges
        for j in ALL_EDGES {
            if a >= c_nk(11 - j as u32, _x as u32) as u16 {
                self.ep[j as usize] = slice_edge[4 - _x];
                ep[j as usize] = slice_edge[4 - _x] as i32;
                a -= c_nk(11 - j as u32, _x as u32) as u16;
                _x -= 1;
            }
        }

        let mut _x = 0; // set the remaining edges UR..DB
        for j in ALL_EDGES {
            if ep[j as usize] == -1 {
                self.ep[j as usize] = other_edge[_x];
                _x += 1;
            }
        }
        for _j in 0..4 {
            rotate_left(&mut self.ep, 0, 11);
        }
    }

    /// Get the permutation of the 8 corners.
    ///
    /// 0 <= corners < 40320 defined but unused in phase 1, 0 <= corners < 40320 in phase 2,
    ///
    /// corners = 0 for solved cube
    pub fn get_corners(&self) -> u16 {
        let mut perm = self.cp.clone(); // duplicate cp
        let mut b = 0;
        for j in ((URF as usize + 1)..=(DRB as usize)).rev() {
            let mut k = 0;
            while perm[j] != ALL_CORNERS[j] {
                rotate_left(&mut perm, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        b as u16
    }

    /// Set the permutation of the 8 corners.
    ///
    /// 0 <= corners < 40320 defined but unused in phase 1, 0 <= corners < 40320 in phase 2,
    ///
    /// corners = 0 for solved cube
    pub fn set_corners(&mut self, idx: u16) {
        self.cp = ALL_CORNERS.clone();
        let mut idx = idx;
        for j in ALL_CORNERS {
            let mut k = idx % (j as u16 + 1);
            idx /= j as u16 + 1;
            while k > 0 {
                rotate_right(&mut self.cp, 0, j as usize);
                k -= 1;
            }
        }
    }

    /// Get the permutation of the 8 U and D edges.
    ///
    /// ud_edges undefined in phase 1, 0 <= ud_edges < 40320 in phase 2, ud_edges = 0 for solved cube.
    pub fn get_ud_edges(&self) -> u16 {
        let mut perm = [UR; 8];
        for i in 0..8 {
            perm[i] = self.ep[i]; // duplicate first 8 elements of ep
        }
        let mut b = 0;
        for j in ((UR as usize + 1)..=(DB as usize)).rev() {
            let mut k = 0;
            while perm[j] != ALL_EDGES[j] {
                rotate_left(&mut perm, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        b as u16
    }

    /// Set the permutation of the 8 U and D edges.
    ///
    /// ud_edges undefined in phase 1, 0 <= ud_edges < 40320 in phase 2, ud_edges = 0 for solved cube.
    pub fn set_ud_edges(&mut self, idx: usize) {
        let mut _x: usize = idx;
        // positions of FR FL BL BR edges are not affected
        for i in 0..8 {
            self.ep[i] = ALL_EDGES[i];
        }
        for j in 0..8 {
            let mut k = _x % (j + 1);
            _x /= j + 1;
            while k > 0 {
                rotate_right(&mut self.ep, 0, j);
                k -= 1;
            }
        }
    }

    /// Generate a random cube. The probability is the same for all possible states.
    pub fn randomize(&mut self) {
        // The permutation of the 12 edges. 0 <= idx < 12!."""
        let mut idx = random::<usize>() % 479001600; // 12!
        self.cp = ALL_CORNERS.clone();
        for j in ALL_EDGES {
            let mut k = idx % (j as usize + 1);
            idx /= j as usize + 1;
            while k > 0 {
                rotate_right(&mut self.ep, 0, j as usize);
                k -= 1;
            }
        }
        let p = self.edge_parity();
        loop {
            self.set_corners(random::<u16>() % 40320); // 8!
            if p == self.corner_parity() {
                // parities of edge and corner permutations must be the same
                break;
            }
        }
        self.set_flip(random::<u16>() % 2048); // 2^11
        self.set_twist(random::<u16>() % 2187); // 3^7
    }
}

pub struct BasicMoveCubeTables {
    bsc: [CubieCube; 18],
}
impl BasicMoveCubeTables {
    pub fn new() -> Self {
        let mut bsc = [CubieCube::default(); 18];
        for i in 0..18 {
            bsc[i] = bsc[i].apply_move(ALL_MOVES[i]);
        }
        Self { bsc }
    }
}

#[dynamic]
static BSCT: BasicMoveCubeTables = BasicMoveCubeTables::new();

/// Rotate array arr right between left and right. right is includ
pub fn rotate_right<T: Copy>(arr: &mut [T], left: usize, right: usize) {
    let temp = arr[right];
    for i in (left + 1..=right).rev() {
        arr[i] = arr[i - 1];
    }
    arr[left] = temp;
}

/// Rotate array arr left between left and right. right is includ
pub fn rotate_left<T: Copy>(arr: &mut [T], left: usize, right: usize) {
    let temp = arr[left];
    for i in left..right {
        arr[i] = arr[i + 1];
    }
    arr[right] = temp;
}

/// Binomial coefficient [n choose k].
pub fn c_nk(n: u32, k: u32) -> u32 {
    let mut k = k;
    if n < k {
        return 0;
    }
    if k > (n / 2) {
        k = n - k;
    }
    let mut s = 1;
    let mut i = n;
    let mut j = 1;
    while i != n - k {
        s *= i;
        s /= j;
        i -= 1;
        j += 1;
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::cubie::{Corner::*, CubieCube, Edge::*, SOLVED_CUBIE_CUBE};
    use crate::facelet::{Color, FaceCube};
    use crate::moves::{Move::*, *};

    #[cfg(feature = "term")]
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
        let moves: Vec<Move> = vec![M2, E2, S2];
        let state = CubieCube::default().apply_moves(&moves);
        #[cfg(feature = "term")]
        let fc = FaceCube::try_from(&state).unwrap();
        #[cfg(feature = "term")]
        let _ = print_facelet(&fc);
        assert_eq!(state, M_MOVE * M_MOVE * E_MOVE * E_MOVE * S_MOVE * S_MOVE);
    }

    #[test]
    fn test_moves() {
        let empty_move = Vec::new();
        let cc = CubieCube::default();
        let cc = cc.apply_moves(&empty_move);
        assert_eq!(cc, CubieCube::default());
        let moves = vec![R, U, R3, U3, M, S, E, M, x, R, U, R3, U3, L, F, R, y, z];
        let _state = CubieCube::default().apply_moves(&moves);
        #[cfg(feature = "term")]
        let _fc = FaceCube::try_from(&_state).unwrap();
        #[cfg(feature = "term")]
        let _ = print_facelet(&_fc);
    }

    #[test]
    fn test_move_n() {
        let cc = CubieCube::default();
        let cc = cc.apply_move(N);
        assert_eq!(cc, CubieCube::default());
        let moves = vec![R, U, R3, U3];
        let cc = cc.apply_moves(&moves);
        let nc = cc.apply_move(N);
        assert_eq!(nc, cc);
        let moves = vec![R, U, R3, U3, N, R, U, R3];
        let cc = CubieCube::default();
        let ncc = cc.apply_moves(&moves);
        let moves = vec![R, U, R3, U3, R, U, R3];
        let cc = cc.apply_moves(&moves);
        assert_eq!(ncc, cc);
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

    #[test]
    fn test_get_twist() {
        let cc = CubieCube::default();
        let cc = cc.apply_moves(&vec![R, U, R3, U3]);
        println!("{}", cc.get_twist());
    }
}
