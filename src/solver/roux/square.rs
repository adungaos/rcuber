use crate::facelet::Color;

pub use crate::facelet::FaceCube;

use super::utils::{to_position, to_state_position};
#[derive(Debug, Clone)]
pub struct SquareCube {
    pub s: [[[Color; 3]; 3]; 6],
}

pub const SOLVED_SQUARE_CUBE: SquareCube = SquareCube {
    s: [
        [
            [Color::U, Color::U, Color::U],
            [Color::U, Color::U, Color::U],
            [Color::U, Color::U, Color::U],
        ],
        [
            [Color::R, Color::R, Color::R],
            [Color::R, Color::R, Color::R],
            [Color::R, Color::R, Color::R],
        ],
        [
            [Color::F, Color::F, Color::F],
            [Color::F, Color::F, Color::F],
            [Color::F, Color::F, Color::F],
        ],
        [
            [Color::D, Color::D, Color::D],
            [Color::D, Color::D, Color::D],
            [Color::D, Color::D, Color::D],
        ],
        [
            [Color::L, Color::L, Color::L],
            [Color::L, Color::L, Color::L],
            [Color::L, Color::L, Color::L],
        ],
        [
            [Color::B, Color::B, Color::B],
            [Color::B, Color::B, Color::B],
            [Color::B, Color::B, Color::B],
        ],
    ],
};
impl Default for SquareCube {
    fn default() -> Self {
        SOLVED_SQUARE_CUBE
    }
}

impl From<FaceCube> for SquareCube {
    fn from(value: FaceCube) -> Self {
        let mut s = [[[Color::U; 3]; 3]; 6];
        for i in 0..54 {
            let (face, row, col) = to_state_position(i);
            s[face][row][col] = value.f[i];
        }
        Self { s }
    }
}

impl From<[[[Color; 3]; 3]; 6]> for SquareCube {
    fn from(value: [[[Color; 3]; 3]; 6]) -> Self {
        Self { s: value }
    }
}

impl From<SquareCube> for FaceCube {
    fn from(value: SquareCube) -> Self {
        let mut f = [Color::U; 54];
        for face in 0..6 {
            for row in 0..3 {
                for col in 0..3 {
                    let pos = to_position(face, row, col);
                    f[pos] = value.s[face][row][col];
                }
            }
        }
        Self { f }
    }
}
#[cfg(test)]
mod tests {
    use crate::solver::roux::utils::generate_masked_state;

    use super::SquareCube;

    #[test]
    fn test_sc() {
        let sc = SquareCube::default();
        println!("{:?}", sc);
    }
    #[test]
    fn test_mask() {
        let fb_pieces = vec![21, 23, 27, 30, 33, 39, 40, 41, 42, 43, 44, 50, 53];
        let fb_mask = generate_masked_state(&fb_pieces);
        let mask_sc = SquareCube::from(fb_mask);
        println!("{:?}", mask_sc);
    }
}
