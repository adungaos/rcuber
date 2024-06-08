//* Module for solving Rubik's Cube OLL.*/
use std::collections::HashMap;
use crate::cubie::CubieCube;
use crate::moves::Move::{self, *};

pub struct OLLSolver {
    pub cube: CubieCube,
    algos: HashMap<[u8; 12], Vec<Move>>,
}

impl OLLSolver {
    pub fn new(cube: CubieCube) -> Self {
        let mut algos = HashMap::new();
        // OLL1
        algos.insert(
            [1u8, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![R, U2, R2, F, R, F3, U2, R3, F, R, F3],
        );
        algos.insert(
            [0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![F, R, U, R3, U3, F3, Fw, R, U, R3, U3, Fw3],
        );
        algos.insert(
            [1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0],
            vec![Fw, R, U, R3, U3, Fw3, U3, F, R, U, R3, U3, F3],
        );
        algos.insert(
            [0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0],
            vec![Fw, R, U, R3, U3, Fw3, U, F, R, U, R3, U3, F3],
        );
        algos.insert(
            [1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0],
            vec![Rw3, U2, R, U, R3, U, Rw],
        );
        algos.insert(
            [0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1],
            vec![Lw, U2, L3, U3, L, U3, Lw3],
        );
        algos.insert(
            [0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0],
            vec![Lw, U, L3, U, L, U2, Lw3],
        );
        algos.insert(
            [0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1],
            vec![Rw3, U3, R, U3, R3, U2, Rw],
        );
        algos.insert(
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            vec![R, U, R3, U3, R3, F, R2, U, R3, U3, F3],
        );
        algos.insert(
            [0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            vec![R, U, R3, U, R3, F, R, F3, R, U2, R3],
        );
        // OLL11
        algos.insert(
            [1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0],
            vec![F3, L3, U3, L, U, F, y, F, R, U, R3, U3, F3, y3],
        );
        algos.insert(
            [0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0],
            vec![F, R, U, R3, U3, F3, U, F, R, U, R3, U3, F3],
        );
        algos.insert(
            [1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![Rw, U3, Rw3, U3, Rw, U, Rw3, y3, R3, U, R, y],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1],
            vec![R3, F, R, U, R3, F3, R, y3, R, U3, R3, y],
        );
        algos.insert(
            [1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0],
            vec![Lw3, U3, Lw, L3, U3, L, U, Lw3, U, Lw],
        );
        algos.insert(
            [0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0],
            vec![Rw, U, Rw3, R, U, R3, U3, Rw, U3, Rw3],
        );
        algos.insert(
            [0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1],
            vec![R, U, R3, U, R3, F, R, F3, U2, R3, F, R, F3],
        );
        algos.insert(
            [0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1],
            vec![F, R, U, R3, U, y3, R3, U2, R3, F, R, F3, y],
        );
        algos.insert(
            [0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0],
            vec![Rw3, R, U, R, U, R3, U3, Rw, R2, F, R, F3],
        );
        algos.insert(
            [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
            vec![Rw3, R, U, R, U, R3, U3, M2, U, R, U3, Rw3],
        );
        // OLL21
        algos.insert(
            [0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
            vec![R, U2, R3, U3, R, U, R3, U3, R, U3, R3],
        );
        algos.insert(
            [0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1],
            vec![R, U2, R2, U3, R2, U3, R2, U2, R],
        );
        algos.insert(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![R2, D3, R, U2, R3, D, R, U2, R],
        );
        algos.insert(
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![Rw, U, R3, U3, Rw3, F, R, F3],
        );
        algos.insert(
            [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![F3, Rw, U, R3, U3, Rw3, F, R],
        );
        algos.insert(
            [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            vec![R, U2, R3, U3, R, U3, R3],
        );
        algos.insert(
            [1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![R, U, R3, U, R, U2, R3],
        );
        algos.insert(
            [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![Rw, U, R3, U3, Rw3, R, U, R, U3, R3],
        );
        algos.insert(
            [0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            vec![Fw3, L3, U3, L2, U, L, U3, L2, U, L, Fw],
        );
        algos.insert(
            [0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
            vec![Fw, R, U, R2, U3, R3, U, R2, U3, R3, Fw3],
        );
        // OLL31
        algos.insert(
            [0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0],
            vec![R3, U3, F, U, R, U3, R3, F3, R],
        );
        algos.insert(
            [0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0],
            vec![R, Dw, L3, Dw3, R3, U, Lw, U, Lw3],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0],
            vec![R, U, R3, U3, R3, F, R, F3],
        );
        algos.insert(
            [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0],
            vec![R, U, R2, U3, R3, F, R, U, R, U3, F3],
        );
        algos.insert(
            [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0],
            vec![R, U2, R2, F, R, F3, R, U2, R3],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1],
            vec![L3, U3, L, U3, L3, U, L, U, L, F3, L3, F],
        );
        algos.insert(
            [1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![F, R, U3, R3, U3, R, U, R3, F3],
        );
        algos.insert(
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![R, U, R3, U, R, U3, R3, U3, R3, F, R, F3],
        );
        algos.insert(
            [0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![L, F3, L3, U3, L, U, F, U3, L3],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1],
            vec![R3, F, R, U, R3, U3, F3, U, R],
        );
        //41
        algos.insert(
            [0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0],
            vec![R, U3, R3, U2, R, U, y, R, U3, R3, U3, F3, y3],
        );
        algos.insert(
            [0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
            vec![L3, U, L, U2, L3, U3, y3, L3, U, L, U, F, y],
        );
        algos.insert(
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![Fw3, L3, U3, L, U, Fw],
        );
        algos.insert(
            [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0],
            vec![Fw, R, U, R3, U3, Fw3],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            vec![F, R, U, R3, U3, F3],
        );
        algos.insert(
            [1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            vec![R3, U3, R3, F, R, F3, U, R],
        );
        algos.insert(
            [1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0],
            vec![F3, L3, U3, L, U, L3, U3, L, U, F],
        );
        algos.insert(
            [0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1],
            vec![F, R, U, R3, U3, R, U, R3, U3, F3],
        );
        algos.insert(
            [0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1],
            vec![R3, F, R3, F3, R2, U2, y, R3, F, R, F3, y3],
        );
        algos.insert(
            [1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0],
            vec![R3, F, R2, B3, R2, F3, R2, B, R3],
        );
        // 51
        algos.insert(
            [0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            vec![Fw, R, U, R3, U3, R, U, R3, U3, Fw3],
        );
        algos.insert(
            [1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0],
            vec![R, U, R3, U, R, Dw3, R, U3, R3, F3, Dw],
        );
        algos.insert(
            [1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0],
            vec![Rw3, U3, R, U3, R3, U, R, U3, R3, U2, Rw],
        );
        algos.insert(
            [1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0],
            vec![Rw, U, R3, U, R, U3, R3, U, R, U2, Rw3],
        );
        algos.insert(
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1],
            vec![R, U2, R2, U3, R, U3, R3, U2, F, R, F3],
        );
        algos.insert(
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            vec![Fw, R, U, R3, U3, Fw3, F, R, U, R3, U3, R, U, R3, U3, F3],
        );
        algos.insert(
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0],
            vec![R, U, R3, U3, M3, U, R, U3, Rw3],
        );
        Self {
            cube: cube,
            algos: algos,
        }
    }

    /// Recognise which is Cube's OLL case.
    fn recognise(&self) -> [u8; 12] {
        let mut idx = [0u8; 12];

        for i in 0..4 {
            match self.cube.co[i] {
                0 => {
                    idx[(3 * i + 2) % 12] = 0;
                    idx[(3 * i + 3) % 12] = 0;
                }
                1 => {
                    idx[(3 * i + 2) % 12] = 1;
                    idx[(3 * i + 3) % 12] = 0;
                }
                _ => {
                    idx[(3 * i + 2) % 12] = 0;
                    idx[(3 * i + 3) % 12] = 1;
                }
            }
        }
        for i in 0..4 {
            idx[i * 3 + 1] = self.cube.eo[i];
        }
        idx
    }

    /// Solve the OLL. Returns an Formula.
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
            let algo = self.algos.get(&case);
            // println!("I: {} Case: {:?}, algo: {:?}", i, case, algo);
            if algo.is_some() {
                let algo = algo.expect("Algo");
                self.cube = self.cube.apply_moves(algo);
                // println!("Case: {:?}, algo: {:?}", case, algo);
                if self.is_solved() {
                    result.append(&mut put);
                    result.append(&mut self.algos[&case].clone());
                    break;
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
        result
    }

    /// Check if Cube is solved.
    pub fn is_solved(&self) -> bool {
        let cc = CubieCube::default();
        for i in 0..4 {
            if !self.cube.cp[0..4].contains(&cc.cp[i]) {
                return false;
            }
            if self.cube.co[i] != 0 {
                return false;
            }
            if !self.cube.ep[0..4].contains(&cc.ep[i]) {
                return false;
            }
            if self.cube.eo[i] != 0 {
                return false;
            }
        }
        true
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

    #[test]
    fn test_ollsolver() {
        let cc = CubieCube::default();
        let moves = vec![R, F3, B2, D3, F2, L, U2, L2, R2, L3, R, U2, F2, B2, R, B3, U, F2, D2, U2, B];
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver { cube: cc };
        let _c = cross.solve();
        println!("{:?}", _c);
        let cc = cross.cube.clone();
        let fc = FaceCube::try_from(&cc).unwrap();
        let _r = print_facelet(&fc);
        let mut f2l = F2LSolver { cube: cc };
        let _f = f2l.solve();
        println!("{:?}", _f);
        let fc = FaceCube::try_from(&f2l.cube).unwrap();
        let _r = print_facelet(&fc);
        let cc = f2l.cube.clone();
        let mut oll = OLLSolver::new(cc);
        let _o = oll.solve();
        println!("{:?}", _o);
        let fc = FaceCube::try_from(&oll.cube).unwrap();
        let _r = print_facelet(&fc);
    }
}
