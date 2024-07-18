use super::arraycube::ArrayCube;
use crate::constants::*;
pub use crate::cubie::CubieCube;
use crate::cubie::{Corner::*, Edge::*};
use crate::facelet::*;

/// Convert an ArrayCube to CubieCube.
impl From<&ArrayCube> for CubieCube {
    /// Convert an ArrayCube to CubieCube.
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
