//* Module for solving Rubik's Cube PLL.*/
use std::collections::HashMap;

use super::{corner_to_face, edge_to_face};
use crate::cubie::CubieCube;
use crate::facelet::Color;
use crate::moves::Move::{self, *};

pub struct PLLSolver<'a> {
    pub cube: CubieCube,
    algos: HashMap<&'a str, Vec<Move>>,
}

impl<'a> PLLSolver<'a> {
    pub fn new(cube: CubieCube) -> Self {
        let mut algos = HashMap::new();
        algos.insert(
            "RRRFFFLLLBBB",
            vec![],
        );
        // PLL1
        algos.insert(
            "FRFLFBRLLBBR",
            vec![x3, R2, D2, R3, U3, R, D2, R3, U, R3, x],
        );
        algos.insert("LRBRFRFLLBBF", vec![x3, R, U3, R, D2, R3, U, R, D2, R2, x]);
        algos.insert(
            "BRFLFRFLBRBL",
            vec![
                R2, U, R3, U3, y, R, U, R3, U3, R, U, R3, U3, R, U, R3, y3, R, U3, R2,
            ],
        );
        algos.insert(
            "FRBRBFLLLBFR",
            vec![
                R3, U3, F3, R, U, R3, U3, R3, F, R2, U3, R3, U3, R, U, R3, U, R,
            ],
        );
        algos.insert(
            "LBRFFLBRBRLF",
            vec![R2, Uw, R3, U, R3, U3, R, Uw3, R2, F3, U, F],
        );
        algos.insert(
            "BLBRFFLBRFRL",
            vec![L2, Uw3, L, U3, L, U, L3, Uw, L2, F, U3, F3],
        );

        algos.insert(
            "BLRFFBRBFLRL",
            vec![F3, U3, F, R2, Uw, R3, U, R, U3, R, Uw3, R2],
        );
        algos.insert(
            "FBLBFFLRBRLR",
            vec![F, U, F3, L2, Uw3, L, U3, L3, U, L3, Uw, L2],
        );
        // H Perm
        algos.insert("RLRFBFLRLBFB", vec![M2, U, M2, U2, M2, U, M2]);
        algos.insert(
            "RRRFLLBFFLBB",
            vec![L3, U3, L, F, L3, U3, L, U, L, F3, L2, U, L, U],
        );
        algos.insert(
            "FFBRRFLLLBBR",
            vec![R, U, R3, F3, R, U, R3, U3, R3, F, R2, U3, R3, U3],
        );
        // todo
        algos.insert(
            "RRLBBFLLRFFB",
            vec![z, D, R3, U, R2, D3, R, D, U3, R3, U, R2, D3, R, U3, R, z3],
        );
        algos.insert(
            "LRRFBBRLLBFF",
            vec![z, U3, R, D3, R2, U, R3, D, U3, R, D3, R2, U, R3, D, R3, z3],
        );
        //Ra
        algos.insert(
            "BRRFLFLFBRBL",
            vec![L, U2, L3, U2, L, F3, L3, U3, L, U, L, F, L2],
        );
        //Rb
        algos.insert(
            "BFRFRFLLBRBL",
            vec![R3, U2, R, U2, R3, F, R, U, R3, U3, R3, F3, R2],
        );
        //T
        algos.insert(
            "FLBRFFLRLBBR",
            vec![R, U, R3, U3, R3, F, R2, U3, R3, U3, R, U, R3, F3],
        );
        //Ua
        algos.insert("RLRFRFLFLBBB", vec![R, U3, R, U, R, U, R, U3, R3, U3, R2]);
        //Ub
        algos.insert("RFRFLFLRLBBB", vec![R2, U, R, U, R3, U3, R3, U3, R3, U, R3]);
        algos.insert(
            "RBLBFFLLRFRB",
            vec![R3, U, R3, U3, y, R3, F3, R2, U3, R3, U, R3, F, R, F, y3],
        );
        algos.insert(
            "RRLBFFLBRFLB",
            vec![F, R, U3, R3, U3, R, U, R3, F3, R, U, R3, U3, R3, F, R, F3],
        );
        algos.insert("RFRFRFLBLBLB", vec![M2, U, M2, U, M3, U2, M2, U2, M3, U2]);

        Self {
            cube: cube,
            algos: algos,
        }
    }

    /// Recognise which is Cube's PLL case.
    fn recognise(&self) -> [Color; 12] {
        let mut idx = [Color::U; 12];

        for i in 0..4 {
            let cp = self.cube.cp[i];
            let cp = corner_to_face(cp);
            idx[(3 * i + 2) % 12] = cp.1;
            idx[(3 * i + 3) % 12] = cp.2;
        }
        for i in 0..4 {
            let ep = self.cube.ep[i];
            let ep = edge_to_face(ep);
            idx[i * 3 + 1] = ep.1;
        }
        idx
    }

    /// Solve the PLL. Returns an Formula.
    pub fn solve(&mut self) -> Vec<Move> {
        let mut result = Vec::new();
        for i in 0..4 {
            let mut put = match i {
                1 => vec![Move::U],
                2 => vec![Move::U2],
                3 => vec![Move::U3],
                _ => Vec::new(),
            };
            self.cube = self.cube.apply_moves(&put);
            let case = self.recognise();
            // LBRF
            for r in 0..4 {
                let case: String = case.iter().map(|c|rotate_color(*c, r)).map(|c| format!("{:?}", c)).collect();
                let algo = self.algos.get(case.as_str());
                // println!("U: {i}, r: {r}, Case:{case}, algo: {:?}", algo);
                if algo.is_some() {
                    let algo = algo.expect("Algo");
                    self.cube = self.cube.apply_moves(algo);
                    for j in 0..4 {
                        let mut u_put = match j {
                            1 => vec![Move::U],
                            2 => vec![Move::U2],
                            3 => vec![Move::U3],
                            _ => Vec::new(),
                        };
                        self.cube = self.cube.apply_moves(&u_put);
                        if self.is_solved() {
                            result.append(&mut put);
                            result.append(&mut self.algos[case.as_str()].clone());
                            result.append(&mut u_put);
                            return result;
                        }
                        let u_put_r = match j {
                            1 => vec![Move::U3],
                            2 => vec![Move::U2],
                            3 => vec![Move::U],
                            _ => Vec::new(),
                        };
                        self.cube = self.cube.apply_moves(&u_put_r);
                    }
                }
            }
            let put_r = match i {
                3 => vec![Move::U],
                2 => vec![Move::U2],
                1 => vec![Move::U3],
                _ => Vec::new(),
            };
            self.cube = self.cube.apply_moves(&put_r);
        }
        // println!("PLL: {:?}", result);
        result
    }

    /// Check if Cube is solved.
    pub fn is_solved(&self) -> bool {
        let cc = CubieCube::default();
        if cc == self.cube {
            return true;
        }
        false
    }
}

pub fn rotate_color(color: Color, r: usize) -> Color {
    match color {
        Color::L => {
            match r {
                1 => Color::F,
                2 => Color::R,
                3 => Color::B,
                _ => Color::L,
            }
        },
        Color::F => {
            match r {
                1 => Color::R,
                2 => Color::B,
                3 => Color::L,
                _ => Color::F,
            }
        },
        Color::R => {
            match r {
                1 => Color::B,
                2 => Color::L,
                3 => Color::F,
                _ => Color::R,
            }
        },
        Color::B => {
            match r {
                1 => Color::L,
                2 => Color::F,
                3 => Color::R,
                _ => Color::B,
            }
        },
        _ => color,
    }
}

#[cfg(test)]
mod tests {
    use crate::cubie::CubieCube;
    use crate::facelet::FaceCube;
    use crate::moves::Move::*;
    use crate::printer::print_facelet;
    use crate::solver::cfop::cross::CrossSolver;
    use crate::solver::cfop::f2l::F2LSolver;
    use crate::solver::cfop::oll::OLLSolver;
    use crate::solver::cfop::pll::PLLSolver;

    #[test]
    fn test_ollsolver() {
        let cc = CubieCube::default();
        let moves = vec![
            D, B, F3, R3, D2, B3, D2, R3, L2, U, D2, B, L2, D, R, F3, U3, D2, F3, R3, D2, L3,
        ];
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver { cube: cc };
        let _c = cross.solve();
        if !cross.is_solved() {
            panic!("Cross Error! {:?} : {:?}", moves, _c);
        }
        let cc = cross.cube.clone();
        let fc = FaceCube::try_from(&cc).unwrap();
        let _r = print_facelet(&fc);
        let mut f2l = F2LSolver { cube: cc };
        let _f = f2l.solve();
        let cc = f2l.cube.clone();
        if !f2l.is_solved() {
            panic!("F2L Error! {:?} : {:?}: {:?}", moves, _c, _f);
        }
        let fc = FaceCube::try_from(&cc).unwrap();
        let _r = print_facelet(&fc);
        let mut oll = OLLSolver::new(cc);
        let _o = oll.solve();
        if !oll.is_solved() {
            panic!("OLL Error! {:?} : {:?} : {:?} : {:?}", moves, _c, _f, _o);
        }
        let fc = FaceCube::try_from(&oll.cube).unwrap();
        let _r = print_facelet(&fc);
        let cc = oll.cube.clone();
        let mut pll = PLLSolver::new(cc);
        let _p = pll.solve();
        if !pll.is_solved() {
            panic!(
                "PLL Error! {:?} : {:?} : {:?} : {:?} : {:?}",
                moves, _c, _f, _o, _p
            );
        }
        let fc = FaceCube::try_from(&pll.cube).unwrap();
        let _r = print_facelet(&fc);
    }
}
