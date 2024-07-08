use std::cmp::min;

use super::tables::{SymTables, IT, MT, S2RT, ST};
use super::utils::UT;
use crate::cubie::CubieCube;
use crate::error::Error;
use crate::facelet::FaceCube;

/// Represent a cube by 4 index: corner permutation, edge permutaion, twist, flip.
pub struct PermOriCube {
    pub perm_corner: u32,
    pub perm_edge: u32,
    pub twist: u16,
    pub flip: u16,
}

/// Cube on the cubie level by corner and edge array.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayCube {
    /// Corner Array, relative to SOLVED_STATE. ca = cp + co * 8
    pub ca: [u8; 8],
    /// Edge Array, relative to SOLVED_STATE. ea = ep + eo * 16
    pub ea: [u8; 12],
}

/// Solved cube on the cubie level.
pub const SOLVED_ARRAY_CUBE: ArrayCube = ArrayCube {
    ca: [0, 1, 2, 3, 4, 5, 6, 7],
    ea: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
};

impl Default for ArrayCube {
    fn default() -> Self {
        SOLVED_ARRAY_CUBE
    }
}

/// Build an ArrayCube from CubieCube.
impl From<&CubieCube> for ArrayCube {
    fn from(cc: &CubieCube) -> Self {
        let mut ca = [0; 8];
        let mut ea = [0; 12];
        for i in 0..8 {
            ca[i] = cc.cp[i] as u8 + (cc.co[i] << 3);
        }
        for i in 0..12 {
            ea[i] = cc.ep[i] as u8 + (cc.eo[i] << 4);
        }
        Self { ca, ea }
    }
}

/// Build an ArrayCube from PermOriCube.
impl From<PermOriCube> for ArrayCube {
    fn from(po: PermOriCube) -> Self {
        let mut cc = ArrayCube::default();
        cc.set_perm_corner(po.perm_corner as usize);
        cc.set_perm_edge(po.perm_edge as usize);
        cc.set_twist(po.twist);
        cc.set_flip(po.flip);
        cc
    }
}

/// Build an ArrayCube from a facelet &str.
impl From<&str> for ArrayCube {
    fn from(fc: &str) -> Self {
        let fc = FaceCube::try_from(fc).unwrap();
        let cc = CubieCube::try_from(&fc).unwrap();
        ArrayCube::from(&cc)
    }
}

impl ArrayCube {
    fn set_val_corner(val0: u8, val: u8) -> u8 {
        val | val0 & !7
    }

    fn set_val_edge(val0: u8, val: u8) -> u8 {
        val | val0 & !0xf
    }

    fn get_val_corner(val0: u8) -> u8 {
        val0 & 7
    }

    fn get_val_edge(val0: u8) -> u8 {
        val0 & 0xf
    }

    pub fn inverse_cube(&self) -> Self {
        let mut invcube = ArrayCube::default();
        for edge in 0..12 {
            invcube.ea[(self.ea[edge] & 0xf) as usize] =
                (edge & 0xf | (self.ea[edge] & 0x10) as usize) as u8;
        }
        for corner in 0..8 {
            invcube.ca[(self.ca[corner] & 0x7) as usize] =
                (corner | 0x20 >> (self.ca[corner] >> 3) & 0x18) as u8;
        }
        invcube
    }

    /// prod = self * rhs, Corner Only.
    pub fn corner_multiply(&self, rhs: &ArrayCube) -> Self {
        let mut prod = self.clone();
        for corn in 0..8 {
            let ori_a = self.ca[(rhs.ca[corn] & 7) as usize] >> 3;
            let ori_b = rhs.ca[corn] >> 3;
            prod.ca[corn] =
                (self.ca[(rhs.ca[corn] & 7) as usize] & 7 | (ori_a + ori_b) % 3 << 3) as u8;
        }
        prod
    }

    /// prod = self * rhs, Corner Only. With mirrored cases considered
    pub fn corner_multiply_full(&self, rhs: &ArrayCube) -> Self {
        let mut prod = self.clone();
        for corn in 0..8 {
            let ori_a = self.ca[(rhs.ca[corn] & 7) as usize] >> 3;
            let ori_b = rhs.ca[corn] >> 3;
            let ori_t = match ori_a < 3 {
                true => ori_b,
                false => 6 - ori_b,
            };
            let mut ori = ori_a + ori_t;
            let ori_t = match (ori_a < 3) == (ori_b < 3) {
                true => 0,
                false => 3,
            };
            ori = ori % 3 + ori_t;
            prod.ca[corn] = (self.ca[(rhs.ca[corn] & 7) as usize] & 7 | ori << 3) as u8;
        }
        prod
    }

    /// prod = self * rhs, Edge Only.
    pub fn edge_multiply(&self, rhs: &ArrayCube) -> Self {
        let mut prod = self.clone();
        for edge in 0..12 {
            prod.ea[edge] = (self.ea[(rhs.ea[edge] & 0xf) as usize] ^ (rhs.ea[edge] & 0x10)) as u8;
        }
        prod
    }

    /// prod = self * rhs, Corner and Edge. With mirrored cases considered.
    pub fn multiply(&self, rhs: &ArrayCube) -> Self {
        let mut prod = self.clone();
        prod.ca = prod.corner_multiply(rhs).ca;
        prod.ea = prod.edge_multiply(rhs).ea;
        prod
    }

    /// prod = self * rhs, Corner and Edge.
    pub fn multiply_full(&self, rhs: &ArrayCube) -> Self {
        let mut prod = self.clone();
        prod.ca = prod.corner_multiply_full(rhs).ca;
        prod.ea = prod.edge_multiply(rhs).ea;
        prod
    }

    /// Apply single move step using move table state.
    pub fn apply_move(&self, m: u8) -> Self {
        self.multiply(&MT.move_cube[m as usize])
    }

    /// Applies the sequence of moves to the current state.
    pub fn apply_moves(&self, moves: &[u8]) -> Self {
        moves.iter().fold(*self, |acc, &m| acc.apply_move(m))
    }

    /// b = S_idx^-1 * a * S_idx, Corner Only.
    pub fn corner_conjugate(&self, idx: usize) -> Self {
        let mut b = ArrayCube::default();
        let sinv: ArrayCube = MT.cube_sym[MT.sym_mult_inv[0][idx] as usize];
        let s = MT.cube_sym[idx];
        for corner in 0..8 {
            let ori_a = sinv.ca[(self.ca[(s.ca[corner] & 7) as usize] & 7) as usize] >> 3;
            let ori_b = self.ca[(s.ca[corner] & 7) as usize] >> 3;
            let ori = match ori_a < 3 {
                true => ori_b,
                false => (3 - ori_b) % 3,
            };
            b.ca[corner] =
                sinv.ca[(self.ca[(s.ca[corner] & 7) as usize] & 7) as usize] & 7 | ori << 3;
        }
        b
    }

    /// b = S_idx^-1 * a * S_idx, Edge Only.
    pub fn edge_conjugate(&self, idx: usize) -> Self {
        let mut b = ArrayCube::default();
        let sinv = MT.cube_sym[MT.sym_mult_inv[0][idx] as usize];
        let s = MT.cube_sym[idx];
        for edge in 0..12 {
            b.ea[edge] = sinv.ea[(self.ea[(s.ea[edge] & 0xf) as usize] & 0xf) as usize]
                ^ (self.ea[(s.ea[edge] & 0xf) as usize] & 0x10)
                ^ (s.ea[edge] & 0x10);
        }
        b
    }

    pub fn get_perm_sym_inv(idx: usize, sym: usize, is_corner: bool) -> u16 {
        let mut idxi = IT.perm_inv_edge_sym[idx];
        if is_corner {
            idxi = SymTables::esym2csym(idxi as usize) as u16;
        }
        idxi & 0xfff0 | (MT.sym_mult[(idxi & 0xf) as usize][sym]) as u16
    }

    pub fn get_skip_moves(ssym: u64) -> u32 {
        let mut ret = 0;
        let mut ssym = ssym >> 1;
        let mut i = 1;
        while ssym != 0 {
            i += 1;
            ssym >>= 1;
            if (ssym & 1) == 1 {
                ret |= ST.first_move_sym[i];
            }
        }
        ret
    }

    /// self = S_urf^-1 * self * S_urf.
    pub fn urf_conjugate(&self) -> Self {
        let mut c = self.clone();
        let urf2 = MT.urf2;
        let urf1 = MT.urf1;
        c.ca = urf2.corner_multiply(self).corner_multiply(&urf1).ca;
        c.ea = urf2.edge_multiply(self).edge_multiply(&urf1).ea;
        c
    }

    // Get and set coordinates
    // XSym : Symmetry Coordnate of X. MUST be called after initialization of
    // ClassIndexToRepresentantArrays.

    // ++++++++++++++++++++ Phase 1 Coordnates ++++++++++++++++++++
    // Flip : Orientation of 12 Edges. Raw[0, 2048) Sym[0, 336 * 8)
    // Twist : Orientation of 8 Corners. Raw[0, 2187) Sym[0, 324 * 8)
    // UDSlice : Positions of the 4 UDSlice edges, the order is ignored. [0, 495)

    pub fn get_flip(&self) -> u16 {
        let mut idx: u16 = 0;
        for i in 0..11 {
            idx = idx << 1 | (self.ea[i] >> 4 & 1) as u16;
        }
        idx
    }

    pub fn set_flip(&mut self, idx: u16) {
        let mut idx = idx;
        let mut parity = 0;
        for i in (0..=10).rev() {
            self.ea[i] = self.ea[i] & 0xf | (idx as u8 & 1) << 4;
            parity ^= self.ea[i];
            idx >>= 1;
        }
        self.ea[11] = self.ea[11] & 0xf | parity & 0x10;
    }

    pub fn get_flip_sym(&self) -> u16 {
        S2RT.flip_r2s[self.get_flip() as usize]
    }

    pub fn get_twist(&self) -> u16 {
        let mut idx = 0;
        for i in 0..7 {
            idx += (idx << 1) + (self.ca[i] as u16 >> 3);
        }
        idx
    }

    pub fn set_twist(&mut self, idx: u16) {
        let mut idx = idx;
        let mut twst = 15;
        let mut val;
        for i in (0..=6).rev() {
            val = idx % 3;
            twst -= val;
            self.ca[i] = self.ca[i] & 0x7 | (val as u8) << 3;
            idx /= 3;
        }
        self.ca[7] = self.ca[7] & 0x7 | (twst as u8) % 3 << 3;
    }

    pub fn get_twist_sym(&self) -> u16 {
        S2RT.twist_r2s[self.get_twist() as usize]
    }

    pub fn get_ud_slice(&self) -> u16 {
        494 - self.get_comb_edge(8)
    }

    pub fn set_ud_slice(&mut self, idx: u16) {
        self.set_comb_edge(494 - idx, 8);
    }

    // ++++++++++++++++++++ Phase 2 Coordnates ++++++++++++++++++++
    // perm_edge: Permutations of 12 Edges.
    // perm_edge_ud : Permutations of 8 UD Edges. Raw[0, 40320) Sym[0, 2187 * 16)
    // perm_corner : Permutations of 8 Corners. Raw[0, 40320) Sym[0, 2187 * 16)
    // MPerm : Permutations of 4 UDSlice Edges. [0, 24)

    pub fn set_perm_corner(&mut self, idx: usize) {
        let mut idx = idx;
        let mut val = 0xFEDCBA9876543210u64;
        let mut extract = 0;
        for p in 2..=8 {
            extract = extract << 4 | idx % p;
            idx /= p;
        }
        for i in 0..7 {
            let v = (extract & 0xf) << 2;
            extract >>= 4;
            self.ca[i] = ArrayCube::set_val_corner(self.ca[i], (val >> v & 0xf) as u8);
            let m = (1 << v) - 1;
            val = val & m | val >> 4 & !m;
        }
        self.ca[7] = ArrayCube::set_val_corner(self.ca[7], (val & 0xf) as u8);
    }

    pub fn get_perm_corner(&self) -> usize {
        let mut idx = 0;
        let mut val = 0xFEDCBA9876543210u64;
        for i in 0..7 {
            let v = (self.ca[i] & 7) << 2;
            idx = (8 - i) * idx + (val >> v & 0xf) as usize;
            val -= 0x1111111111111110u64 << v;
        }
        idx
    }

    pub fn get_perm_corner_sym(&self) -> usize {
        SymTables::esym2csym(S2RT.eperm_r2s[self.get_perm_corner()] as usize)
    }

    pub fn get_perm_edge_ud(&self) -> usize {
        let mut idx = 0;
        let mut val = 0xFEDCBA9876543210u64;
        for i in 0..7 {
            let v = (self.ea[i] & 0xf) << 2;
            idx = (8 - i) * idx + (val >> v & 0xf) as usize;
            val -= 0x1111111111111110u64 << v;
        }
        idx
    }

    pub fn get_perm_edge(&self) -> usize {
        let mut idx = 0;
        let mut val = 0xFEDCBA9876543210u64;
        for i in 0..11 {
            let v = (self.ea[i] & 0xf) << 2;
            idx = (12 - i) * idx + (val >> v & 0xf) as usize;
            val -= 0x1111111111111110u64 << v;
        }
        idx
    }

    pub fn set_perm_edge_ud(&mut self, idx: usize) {
        let mut idx = idx;
        let mut val = 0xFEDCBA9876543210u64;
        let mut extract = 0;
        for p in 2..=8 {
            extract = extract << 4 | idx % p;
            idx /= p;
        }
        for i in 0..7 {
            let v = (extract & 0xf) << 2;
            extract >>= 4;
            self.ea[i] = ArrayCube::set_val_edge(self.ea[i], (val >> v & 0xf) as u8);
            let m = (1 << v) - 1;
            val = val & m | val >> 4 & !m;
        }
        self.ea[7] = ArrayCube::set_val_edge(self.ea[7], (val & 0xf) as u8);
    }

    pub fn set_perm_edge(&mut self, idx: usize) {
        let mut idx = idx;
        let mut val = 0xFEDCBA9876543210u64;
        let mut extract = 0;
        for p in 2..=12 {
            extract = extract << 4 | idx % p;
            idx /= p;
        }
        for i in 0..11 {
            let v = (extract & 0xf) << 2;
            extract >>= 4;
            self.ea[i] = ArrayCube::set_val_edge(self.ea[i], (val >> v & 0xf) as u8);
            let m = (1 << v) - 1;
            val = val & m | val >> 4 & !m;
        }
        self.ea[11] = ArrayCube::set_val_edge(self.ea[11], (val & 0xf) as u8);
    }

    pub fn get_perm_edge_sym(&self) -> u16 {
        S2RT.eperm_r2s[self.get_perm_edge_ud()]
    }

    pub fn get_perm_m(&self) -> usize {
        self.get_perm_edge() % 24
    }

    pub fn set_perm_m(&mut self, idx: usize) {
        self.set_perm_edge(idx);
    }

    pub fn get_comb_corner(&self, mask: u8) -> u16 {
        let mut idx_c = 0;
        let mut r = 4;
        for i in (0..=7).rev() {
            let perm = ArrayCube::get_val_corner(self.ca[i].into());
            if (perm & 0xc) == mask {
                idx_c += UT.cnk[i][r];
                r -= 1;
            }
        }
        idx_c
    }

    pub fn set_comb_corner(&mut self, idx_c: usize, mask: u8) {
        let mut idx_c = idx_c;
        let mut r = 4;
        let mut fill = 7;
        for i in (0..=7).rev() {
            if idx_c >= UT.cnk[i][r as usize] as usize {
                idx_c -= UT.cnk[i][r as usize] as usize;
                r -= 1;
                self.ca[i] = ArrayCube::set_val_corner(self.ca[i], r | mask);
            } else {
                if (fill & 0xc) == mask {
                    fill -= 4;
                }
                self.ca[i] = ArrayCube::set_val_corner(self.ca[i], fill);
                fill -= 1;
            }
        }
    }

    pub fn get_comb_edge(&self, mask: u8) -> u16 {
        let mut idx_c = 0;
        let mut r = 4;
        for i in (0..=11).rev() {
            let perm = ArrayCube::get_val_edge(self.ea[i].into());
            if (perm & 0xc) == mask {
                idx_c += UT.cnk[i][r];
                r -= 1;
            }
        }
        idx_c
    }

    pub fn set_comb_edge(&mut self, idx_c: u16, mask: u8) {
        let mut idx_c = idx_c;
        let mut r = 4;
        let mut fill: i8 = 11;
        for i in (0..=11).rev() {
            if idx_c >= UT.cnk[i][r as usize] {
                idx_c -= UT.cnk[i][r as usize];
                r -= 1;
                self.ea[i] = ArrayCube::set_val_edge(self.ea[i], r | mask);
            } else {
                if (fill & 0xc) as u8 == mask {
                    fill -= 4;
                }
                self.ea[i] = ArrayCube::set_val_edge(self.ea[i], fill as u8);
                fill -= 1;
            }
        }
    }

    
    /// Check a cubiecube for solvability. Return the error code.
    pub fn verify(&self) -> Result<bool, Error> {
        let mut sum = 0;
        let mut edge_mask = 0;
        for e in 0..12 {
            edge_mask |= 1 << (self.ea[e] & 0xf);
            sum ^= self.ea[e] & 0x10;
        }
        if edge_mask != 0xfff {
            return Err(Error::InvalidEdge); // missing edges
        }
        if sum != 0 {
            return Err(Error::FlipError);
        }
        let mut corn_mask = 0;
        sum = 0;
        for c in 0..8 {
            corn_mask |= 1 << (self.ca[c] & 7);
            sum += self.ca[c] >> 3;
        }
        if corn_mask != 0xff {
            return Err(Error::InvalidCorner); // missing corners
        }
        if sum % 3 != 0 {
            return Err(Error::TwistError); // twisted corner
        }
        if (self.get_nparity(self.get_perm_edge(), 12)
            ^ self.get_nparity(self.get_perm_corner(), 8))
            != 0
        {
            return Err(Error::ParityError); // parity error
        }
        Ok(true) // cube ok
    }

    pub fn get_nparity(&self, idx: usize, n: usize) -> usize {
        let mut p = 0;
        let mut idx = idx;
        for i in (0..=(n - 2)).rev() {
            p ^= idx % (n - i);
            idx /= n - i;
        }
        p & 1
    }

    pub fn symmetry(&self) -> u64 {
        // println!("Symmetry...");
        let mut c = self.clone();
        let mut d = ArrayCube::default();
        let cperm = c.get_perm_corner_sym() >> 4;
        let mut sym: u64 = 0;
        for urf_inv in 0..6 {
            let cpermx = c.get_perm_corner_sym() >> 4;
            // println!("Symmetry::cperm:cpermx::{}:{}",cperm, cpermx);
            if cperm == cpermx {
                for i in 0..16 {
                    d.ca = c.corner_conjugate(MT.sym_mult_inv[0][i] as usize).ca;
                    if d.ca == self.ca {
                        d.ea = c.edge_conjugate(MT.sym_mult_inv[0][i] as usize).ea;
                        if d.ea == self.ea {
                            sym |= 1u64 << min(urf_inv << 4 | i, 48);
                        }
                    }
                }
            }
            c = c.urf_conjugate();
            if urf_inv % 3 == 2 {
                c = c.inverse_cube();
            }
        }
        sym
    }
}

#[cfg(test)]
mod tests {
    use super::super::tables::MT;
    use super::{ArrayCube, PermOriCube};
    use crate::cubie::CubieCube;
    use crate::facelet::FaceCube;
    use std::time::Instant;
    #[test]
    fn test_sym() {
        let fc = "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL";
        let ac = ArrayCube::from(fc);
        let sym = ac.symmetry();
        println!("{}", sym);
        for c in MT.move_cube {
            println!("{}", c.symmetry());
        }
    }

    #[test]
    fn test_urfconj() {
        let fc = "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL";
        let mut ac = ArrayCube::from(fc);

        println!("{}", ac.get_perm_corner());
        println!("{}", ac.get_perm_edge());
        println!("{}", ac.get_twist());
        println!("{}", ac.get_flip());
        println!("{:?}", ac);
        ac = ac.urf_conjugate();
        println!("{:?}", ac);
        assert_eq!(ac.get_perm_corner(), 9818);
        assert_eq!(ac.get_perm_edge_ud(), 26927);
        assert_eq!(ac.get_twist(), 1228);
        assert_eq!(ac.get_flip(), 1051);
    }

    #[test]
    fn test_inverse() {
        let start = Instant::now();
        // let fc = "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL";
        let ca = ArrayCube::from(PermOriCube {
            perm_corner: 24369,
            perm_edge: 436583227,
            twist: 1470,
            flip: 1306,
        });
        // println!("{:?}", ca.verify());
        let cc = CubieCube::from(&ca);
        // println!("{:?}", cc.verify());
        // println!("{:?}", cc);
        let fc = FaceCube::try_from(&cc).unwrap();
        assert!(fc.to_string() == "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL");
        let fc = "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL";
        let ac = ArrayCube::from(fc);
        // println!("{:?}", ac);
        println!(
            "{:?}, {}, {}, {}",
            ac.get_perm_corner(),
            ac.get_perm_edge(),
            ac.get_twist(),
            ac.get_flip()
        );

        let ccac = ca.clone();

        println!("{:?}", ccac.verify());
        println!("{:?}", ccac);
        let ccai = ca.inverse_cube();
        println!("{:?}", ccai);
        let urf1 = ArrayCube::from(PermOriCube {
            perm_corner: 2531,
            perm_edge: 67026819,
            twist: 1373,
            flip: 1367,
        });
        println!("{:?}", urf1);
        let elapsed = start.elapsed();
        println!("{:?}", elapsed);
    }
}
