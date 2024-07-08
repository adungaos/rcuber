use std::str::FromStr;

use super::{get_move_face, get_put_move};
use crate::cubie::Edge::{self, *};
use crate::error::Error;
use crate::facelet::Color;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// DaisySolver for LBL(Layer by Layer) method, i.e, solve orientation of bottom edges(DR, DF, DL, DB) at Up face, no matter the order.
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::moves::Formula;
/// use rcuber::solver::lbl::daisy::DaisySolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = Formula::scramble();
///     let cc = cc.apply_formula(&moves);
///     let cc2 = cc.clone();
///     let mut daisy = DaisySolver{cube: cc};
///     let _cs = daisy.solve();
///     assert!(daisy.is_solved());
///     let cc2 = cc2.apply_moves(&_cs);
///     assert_eq!(cc2, daisy.cube);
///     println!("Scramble: {:?}\nSolution: {:?}", moves, _cs);
/// }
/// ```
pub struct DaisySolver {
    pub cube: CubieCube,
}

impl DaisySolver {
    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let cc = CubieCube::default();
        let mut d_edges = Vec::new();
        for edge in [DR, DF, DL, DB] {
            let _edge = self.get_edge(edge).unwrap();
            if _edge.1 <= 3 && _edge.2 == 0 {
                // d_edges.push((edge, 0));
            } else if _edge.1 >= 8 {
                d_edges.push((edge, 1));
            } else if (_edge.1 >= 4 && _edge.1 < 8) && _edge.2 == 0 {
                d_edges.push((edge, 2));
            } else {
                d_edges.push((edge, 3));
            }
        }
        d_edges.sort_by_key(|e| e.1);
        for edge in d_edges {
            let _edge = self.get_edge(edge.0).unwrap();
            if _edge.1 <= 3 {
                if _edge.2 == 0 {
                    continue;
                } else if _edge.2 == 1 {
                    let faces = format!("{:?}", cc.ep[_edge.1 as usize]);
                    let faces = faces.as_str();
                    let face = faces[1..2].to_string();
                    let step = Move::from_str(&face).unwrap();
                    self.cube = self.cube.apply_move(step);
                    solution.push(step);
                    let _edge = self.get_edge(edge.0).unwrap();
                    let step = self.get_move(self.get_edge(edge.0).unwrap()).unwrap();
                    for i in 0..4 {
                        let mut u_put = get_put_move(i, U);
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&u_put);
                        let face = get_move_face(step);
                        if self.is_empty_u_slot(face) {
                            let step = Move::from_str(step.to_string().as_str()).unwrap();
                            self.cube = self.cube.apply_move(step);
                            solution.append(&mut u_put);
                            solution.push(step);
                            break;
                        }
                        self.cube = _cube;
                    }
                }
            } else if _edge.1 >= 4 && _edge.1 < 8 {
                if _edge.2 == 0 {
                    let _edge = self.get_edge(edge.0).unwrap();
                    let step = self.get_move(self.get_edge(edge.0).unwrap()).unwrap();
                    for i in 0..4 {
                        let mut u_put = get_put_move(i, U);
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&u_put);
                        let face = get_move_face(step);
                        if self.is_empty_u_slot(face) {
                            let step = Move::from_str(step.to_string().as_str()).unwrap();
                            self.cube = self.cube.apply_move(step);
                            // self.cube = self.cube.apply_move(step);
                            solution.append(&mut u_put);
                            // solution.push(step);
                            solution.push(step);
                            break;
                        }
                        self.cube = _cube;
                    }
                } else {
                    let faces = format!("{:?}", cc.ep[_edge.1 as usize]);
                    let faces = faces.as_str();
                    let face = faces[1..2].to_string();
                    let step = Move::from_str(&face).unwrap();
                    for i in 0..4 {
                        let mut u_put = get_put_move(i, U);
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&u_put);
                        let face = get_move_face(step);
                        if self.is_empty_u_slot(face) {
                            self.cube = self.cube.apply_move(step);
                            solution.append(&mut u_put);
                            solution.push(step);
                            break;
                        }
                        self.cube = _cube;
                    }
                    let _edge = self.get_edge(edge.0).unwrap();
                    let step = self.get_move(self.get_edge(edge.0).unwrap()).unwrap();
                    for i in 0..4 {
                        let mut u_put = get_put_move(i, U);
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&u_put);
                        let face = get_move_face(step);
                        if self.is_empty_u_slot(face) {
                            let step = Move::from_str(step.to_string().as_str()).unwrap();
                            self.cube = self.cube.apply_move(step);
                            solution.append(&mut u_put);
                            solution.push(step);
                            break;
                        }
                        self.cube = _cube;
                    }
                }
            } else {
                let _edge = self.get_edge(edge.0).unwrap();
                let step = self.get_move(self.get_edge(edge.0).unwrap()).unwrap();
                for i in 0..4 {
                    let mut u_put = get_put_move(i, U);
                    let _cube = self.cube;
                    self.cube = self.cube.apply_moves(&u_put);
                    let face = get_move_face(step);
                    if self.is_empty_u_slot(face) {
                        let step = Move::from_str(step.to_string().as_str()).unwrap();
                        self.cube = self.cube.apply_move(step);
                        solution.append(&mut u_put);
                        solution.push(step);
                        break;
                    }
                    self.cube = _cube;
                }
            }
        }
        solution
    }

    pub fn is_solved(&self) -> bool {
        for edge in [DR, DF, DL, DB] {
            if !self.cube.ep[0..4].contains(&edge) {
                return false;
            }
        }
        for i in 0..4 {
            if self.cube.eo[i] != 0 {
                return false;
            }
        }
        true
    }

    fn get_edge(&self, edge: Edge) -> Result<(Edge, u8, u8), Error> {
        for i in 0..12 {
            if self.cube.ep[i] == edge {
                return Ok((edge, i as u8, self.cube.eo[i]));
            }
        }
        Err(Error::InvalidEdge)
    }

    fn is_empty_u_slot(&self, face: Color) -> bool {
        let edge = format!("U{:?}", face);
        let ei = Edge::try_from(edge.as_str()).unwrap() as usize;
        if self.cube.eo[ei] == 1 {
            return true;
        } else if [DR, DF, DL, DB].contains(&self.cube.ep[ei]) {
            return false;
        }
        true
    }

    fn get_move(&self, edge: (Edge, u8, u8)) -> Result<Move, Error> {
        let cc = CubieCube::default();
        let _edge = cc.ep[edge.1 as usize];
        match _edge {
            FR => match edge.2 {
                0 => Ok(R),
                _ => Ok(F3),
            },
            FL => match edge.2 {
                0 => Ok(L3),
                _ => Ok(F),
            },
            BR => match edge.2 {
                0 => Ok(R3),
                _ => Ok(B),
            },
            BL => match edge.2 {
                0 => Ok(L),
                _ => Ok(B3),
            },
            DL => match edge.2 {
                0 => Ok(L2),
                _ => Err(Error::InvalidEdge),
            },
            DR => match edge.2 {
                0 => Ok(R2),
                _ => Err(Error::InvalidEdge),
            },
            DB => match edge.2 {
                0 => Ok(B2),
                _ => Err(Error::InvalidEdge),
            },
            DF => match edge.2 {
                0 => Ok(F2),
                _ => Err(Error::InvalidEdge),
            },
            _ => Err(Error::InvalidEdge),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cubie::CubieCube, moves::Formula, solver::lbl::daisy::DaisySolver};

    #[test]
    fn test_daisy() {
        let cc = CubieCube::default();
        let moves = Formula::scramble();
        let cc = cc.apply_formula(&moves);
        let cc2 = cc.clone();
        let mut daisy = DaisySolver { cube: cc };
        let _cs = daisy.solve();
        assert!(daisy.is_solved());
        let cc2 = cc2.apply_moves(&_cs);
        assert_eq!(cc2, daisy.cube);
        println!("Scramble: {:?}\nSolution: {:?}", moves, _cs);
    }
}
