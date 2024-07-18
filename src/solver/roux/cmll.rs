use crate::cubie::{Corner, CubieCube};
use crate::facelet::Color;
use crate::moves::Formula;
use crate::moves::Move::{self, *};
use std::collections::HashMap;

/// `CMLL` is the third step of the Roux method, after solving the first two blocks. The goal of this step is to solve the corners of the last layer without considering the M-slice.
/// In contrast to other algorithm sets like COLL, this algorithm set IS allowed to disturb the M-slice. This gives more freedom and allows more efficient algorithms for some cases.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::roux::fb::FBSolver;
/// use rcuber::solver::roux::sb::SBSolver;
/// use rcuber::solver::roux::cmll::CMLLSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let f = Formula::scramble();
///     let cc = cc.apply_formula(&f);
///     let mut fb = FBSolver::new(cc);
///     let _fb = fb.solve();
///     assert!(fb.is_solved());
///     let mut sb = SBSolver::new(fb.cube);
///     let _sb = sb.solve();
///     assert!(sb.is_solved());
///     let mut cmll = CMLLSolver::new(sb.cube);
///     let _cmll = cmll.solve();
///     assert!(cmll.is_solved());
///     println!("
///         Scramble: {:?}\nFirst Block: {:?}\nSecond Block: {:?}\nCMLL: {:?}",
///         f.moves, _fb, _sb, _cmll
///     );
/// }
/// ```

pub struct CMLLSolver {
    pub cube: CubieCube,
    pub algos: HashMap<String, Vec<Move>>,
}

impl CMLLSolver {
    /// Construct the CMLLSolver.
    pub fn new(cube: CubieCube) -> Self {
        let mut algos = HashMap::new();
        // solved
        algos.insert("RFFLLBBR".to_string(), vec![]);
        // o_adjacent_swap
        algos.insert(
            "BRFLLBRF".to_string(),
            vec![R, U, R3, F3, R, U, R3, U3, R3, F, R2, U3, R3],
        );
        // o_diagonal_swap
        algos.insert(
            "LBFLRFBR".to_string(),
            vec![F, R, U3, R3, U3, R, U, R3, F3, R, U, R3, U3, R3, F, R, F3],
        );
        // h_columns
        algos.insert(
            "RUURLUUL".to_string(),
            vec![U3, R, U, R3, U, R, U3, R3, U, R, U2, R3],
        );
        // h_rows
        algos.insert(
            "LUURRUUL".to_string(),
            vec![F, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, F3],
        );
        // h_column
        algos.insert(
            "FUULLUUB".to_string(),
            vec![U3, R, U2, R2, F, R, F3, U2, R3, F, R, F3],
        );
        // h_row
        algos.insert(
            "LUURBUUB".to_string(),
            vec![U2, Rw, U3, Rw2, D3, Rw, U3, Rw3, D, Rw2, U, Rw3],
        );
        // pi_right_bar
        algos.insert(
            "FULUULUB".to_string(),
            vec![F, R, U, R3, U3, R, U, R3, U3, F3],
        );
        // pi_back_slash
        algos.insert(
            "BURUUFUR".to_string(),
            vec![U, F, R3, F3, R, U2, R, U3, R3, U, R, U2, R3],
        );
        // pi_x_checkerboard
        algos.insert(
            "BUFUUFUB".to_string(),
            vec![U3, R3, F, R, U, F, U3, R, U, R3, U3, F3],
        );
        // pi_forward_slash
        algos.insert(
            "FURUUFUL".to_string(),
            vec![R, U2, R3, U3, R, U, R3, U2, R3, F, R, F3],
        );
        // pi_columns
        algos.insert(
            "LURUULUR".to_string(),
            vec![U3, Rw, U3, Rw2, D3, Rw, U, Rw3, D, Rw2, U, Rw3],
        );
        // pi_left_bar
        algos.insert(
            "FURUULUF".to_string(),
            vec![U3, R3, U3, R3, F, R, F3, R, U3, R3, U2, R],
        );
        // u_forward_slash
        algos.insert(
            "LBRFLUUB".to_string(),
            vec![U2, R2, D, R3, U2, R, D3, R3, U2, R3],
        );
        // u_back_slash
        algos.insert("BRFLFUUL".to_string(), vec![R2, D3, R, U2, R3, D, R, U2, R]);
        // u_front_row
        algos.insert(
            "RFFLBUUB".to_string(),
            vec![R3, U3, R, U3, R3, U2, R2, U, R3, U, R, U2, R3],
        );
        // u_rows
        algos.insert(
            "RFFLRUUL".to_string(),
            vec![U3, F, R2, D, R3, U, R, D3, R2, U3, F3],
        );
        // u_x_checkerboard
        algos.insert(
            "FLRFBUUB".to_string(),
            vec![U2, Rw, U3, Rw3, U, Rw3, D3, Rw, U3, Rw3, D, Rw],
        );
        // u_back_row
        algos.insert("LBFLFUUB".to_string(), vec![F, R, U, R3, U3, F3]);
        // t_left_bar
        algos.insert("LBRFUBLU".to_string(), vec![U3, R, U, R3, U3, R3, F, R, F3]);
        // t_right_bar
        algos.insert("FLBRURBU".to_string(), vec![U, L3, U3, L, U, L, F3, L3, F]);
        // t_rows
        algos.insert(
            "BRRFUFBU".to_string(),
            vec![R, U2, R3, U3, R, U3, R2, U2, R, U, R3, U, R],
        );
        // t_front_row
        algos.insert(
            "LBBRUFFU".to_string(),
            vec![Rw3, U, Rw, U2, R2, F, R, F3, R],
        );
        // t_back_row
        algos.insert(
            "BRLBURLU".to_string(),
            vec![Rw3, D3, Rw, U, Rw3, D, Rw, U3, Rw, U, Rw3],
        );
        // t_columns
        algos.insert(
            "BRLBUFFU".to_string(),
            vec![U2, Rw2, D3, Rw, U, Rw3, D, Rw2, U3, Rw3, U3, Rw],
        );
        // s_left_bar
        algos.insert("BRFULUBU".to_string(), vec![R, U, R3, U, R, U2, R3]);
        // s_x_checkerboard
        algos.insert("RFBULURU".to_string(), vec![L3, U2, L, U2, L, F3, L3, F]);
        // s_forward_slash
        algos.insert("RFLURUBU".to_string(), vec![F, R3, F3, R, U2, R, U2, R3]);
        // s_columns
        algos.insert(
            "RFBURULU".to_string(),
            vec![R, U, R3, U3, R3, F, R, F3, R, U, R3, U, R, U2, R3],
        );
        // s_right_bar
        algos.insert(
            "LBLUFURU".to_string(),
            vec![U2, R, U, R3, U, R3, F, R, F3, R, U2, R3],
        );
        // s_back_slash
        algos.insert("RFRULUBU".to_string(), vec![R, U3, L3, U, R3, U3, L]);
        // as_right_bar
        algos.insert("UBRFUFUL".to_string(), vec![U3, R, U2, R3, U3, R, U3, R3]);
        // as_columns
        algos.insert(
            "ULRFUBUF".to_string(),
            vec![R2, D, R3, U, R, D3, R3, U, R3, U3, R, U3, R3],
        );
        // as_back_slash
        algos.insert(
            "URFLUBUL".to_string(),
            vec![U3, F3, Rw, U, Rw3, U2, Rw3, F2, Rw],
        );
        // as_x_checkerboard
        algos.insert("UFBRURUL".to_string(), vec![R, U2, R3, U2, R3, F, R, F3]);
        // as_forward_slash
        algos.insert("URBRUFUL".to_string(), vec![L3, U, R, U3, L, U, R3]);
        // as_left_bar
        algos.insert(
            "ULFLURUB".to_string(),
            vec![U2, R, U2, R3, F, R3, F3, R, U3, R, U3, R3],
        );
        // l_mirror
        algos.insert("RFLUBRUL".to_string(), vec![F, R, U3, R3, U3, R, U, R3, F3]);
        // l_inverse
        algos.insert("FLRULBUR".to_string(), vec![F, R3, F3, R, U, R, U3, R3]);
        // l_pure
        algos.insert(
            "RFLULBUB".to_string(),
            vec![U2, R, U, R3, U, R, U3, R3, U, R, U3, R3, U, R, U2, R3],
        );
        // l_front_commutator
        algos.insert("RFRUFLUL".to_string(), vec![R, U2, R, D, R3, U2, R, D3, R2]);
        // l_diag
        algos.insert(
            "BRBUFLUR".to_string(),
            vec![U2, R3, U3, R, U, R3, F3, R, U, R3, U3, R3, F, R2],
        );
        // l_back_commutator
        algos.insert(
            "BRLULBUR".to_string(),
            vec![U3, R3, U2, R3, D3, R, U2, R3, D, R2],
        );
        Self { cube, algos }
    }

    /// Recognise which is Cube's OLL case.
    fn recognise(&self) -> [Color; 8] {
        let mut idx = [Color::U; 8];
        for i in 0..4 {
            let cp = self.cube.cp[i];
            let co = self.cube.co[i];
            let colors = get_colors(cp, co);
            idx[i * 2] = colors.0;
            idx[i * 2 + 1] = colors.1;
        }
        idx
    }

    /// Solve the OLL. Returns an Formula.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut result = Vec::new();
        for i in 0..4 {
            let mut i_put = match i {
                1 => Formula {
                    moves: vec![Move::U],
                },
                2 => Formula {
                    moves: vec![Move::U2],
                },
                3 => Formula {
                    moves: vec![Move::U3],
                },
                _ => Formula { moves: vec![] },
            };
            self.cube = self.cube.apply_formula(&i_put);
            let case = self.recognise();
            for r in 0..4 {
                let case: String = case
                    .iter()
                    .map(|c| rotate_color(*c, r))
                    .map(|c| format!("{:?}", c))
                    .collect();
                let algo = self.algos.get(&case);
                // println!("I: {}, Case: {:?}, algo: {:?}", i, case, algo);
                if algo.is_some() {
                    let algo = Formula {
                        moves: algo.unwrap().clone(),
                    };
                    for j in 0..4 {
                        let mut j_put = match j {
                            1 => Formula {
                                moves: vec![Move::U],
                            },
                            2 => Formula {
                                moves: vec![Move::U2],
                            },
                            3 => Formula {
                                moves: vec![Move::U3],
                            },
                            _ => Formula { moves: vec![] },
                        };
                        self.cube = self.cube.apply_formula(&j_put);
                        self.cube = self.cube.apply_formula(&algo);
                        // println!("Case: {:?}, algo: {:?}", case, algo);
                        for k in 0..4 {
                            let mut k_put = match k {
                                1 => Formula {
                                    moves: vec![Move::U],
                                },
                                2 => Formula {
                                    moves: vec![Move::U2],
                                },
                                3 => Formula {
                                    moves: vec![Move::U3],
                                },
                                _ => Formula { moves: vec![] },
                            };
                            self.cube = self.cube.apply_formula(&k_put);
                            if self.is_solved() {
                                // println!("i: {}, j: {}, k: {}, Case: {:?}, algo: {:?}", i, j, k, case, algo);
                                result.append(&mut i_put.moves);
                                result.append(&mut j_put.moves);
                                result.append(&mut self.algos[&case].clone());
                                result.append(&mut k_put.moves);
                                return result;
                            }
                            self.cube = self.cube.apply_formula(&k_put.inverse());
                        }
                        self.cube = self.cube.apply_formula(&algo.inverse());
                        self.cube = self.cube.apply_formula(&j_put.inverse());
                    }
                }
            }
            self.cube = self.cube.apply_formula(&i_put.inverse());
        }
        result
    }

    /// Check if Cube is solved.
    pub fn is_solved(&self) -> bool {
        let mut solved = 0;
        for i in 0..4 {
            match (i, self.cube.cp[i], self.cube.co[i]) {
                (0, Corner::URF, 0)
                | (1, Corner::UFL, 0)
                | (2, Corner::ULB, 0)
                | (3, Corner::UBR, 0) => solved += 1,
                _ => {}
            }
        }
        solved == 4
    }
}

fn get_colors(cp: Corner, co: u8) -> (Color, Color) {
    let color = corner_to_face(cp);
    match co {
        0 => (color.1, color.2),
        1 => (color.0, color.1),
        _ => (color.2, color.0),
    }
}

/// rotate color for cover all CMLL cases.
fn rotate_color(color: Color, r: usize) -> Color {
    match color {
        Color::L => match r {
            1 => Color::F,
            2 => Color::R,
            3 => Color::B,
            _ => Color::L,
        },
        Color::F => match r {
            1 => Color::R,
            2 => Color::B,
            3 => Color::L,
            _ => Color::F,
        },
        Color::R => match r {
            1 => Color::B,
            2 => Color::L,
            3 => Color::F,
            _ => Color::R,
        },
        Color::B => match r {
            1 => Color::L,
            2 => Color::F,
            3 => Color::R,
            _ => Color::B,
        },
        _ => color,
    }
}

/// Split Corner expression (ex URF) to three faces(ex U, R, F).  
fn corner_to_face(corner: Corner) -> (Color, Color, Color) {
    let corner = format!("{:?}", corner);
    let corner = corner.as_bytes();
    (
        Color::try_from(char::from(corner[0])).unwrap(),
        Color::try_from(char::from(corner[1])).unwrap(),
        Color::try_from(char::from(corner[2])).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::super::fb::FBSolver;
    use super::super::sb::SBSolver;
    use super::CMLLSolver;
    use crate::{cubie::CubieCube, moves::Formula};

    #[test]
    fn test_cmll() {
        let cc = CubieCube::default();
        let f = Formula::scramble();
        let cc = cc.apply_formula(&f);
        let mut fb = FBSolver::new(cc);
        let _fb = fb.solve();
        assert!(fb.is_solved());
        let mut sb = SBSolver::new(fb.cube);
        let _sb = sb.solve();
        assert!(sb.is_solved());
        let mut cmll = CMLLSolver::new(sb.cube);
        let _cmll = cmll.solve();
        assert!(cmll.is_solved());
        println!(
            "Scramble: {:?}\nFirst Block: {:?}\nSecond Block: {:?}\nCMLL: {:?}",
            f.moves, _fb, _sb, _cmll
        );
    }
}
