use crate::cubie::Corner::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// BottomCornerSolver for LBL(Layer by Layer) method, i.e, solve four bottom corners(DFR, DLF, DBL, DRB).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::scramble;
/// use rcuber::solver::lbl::cross::CrossSolver;
/// use rcuber::solver::lbl::bottom::BottomCornerSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = scramble();
///     let cc = cc.apply_moves(&moves);
///     let mut cross = CrossSolver::new(cc, true);
///     let _cs = cross.solve();
///     let mut bottom = BottomCornerSolver{cube: cross.cube};
///     let _bs = bottom.solve();
///     assert!(bottom.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}, {:?}", moves, _cs, _bs);
/// }
/// ```
pub struct BottomCornerSolver {
    pub cube: CubieCube,
}

impl BottomCornerSolver {
    fn is_solved_corner(corner: (Corner, u8, u8)) -> bool {
        corner.0 as u8 == corner.1 && corner.2 == 0
    }

    fn get_sorted_corners(&self) -> Vec<((Corner, u8, u8), i32)> {
        let d_corners = get_corners_d(&self.cube);
        let mut d_corners_sort = Vec::new();
        for corner in d_corners {
            if BottomCornerSolver::is_solved_corner(corner) {
                continue;
            }
            if corner.1 < 4 {
                if corner.2 == 1 {
                    d_corners_sort.push((corner, -1));
                } else {
                    d_corners_sort.push((corner, -2));
                }
            } else {
                d_corners_sort.push((corner, -3));
            }
        }
        d_corners_sort.sort_by_key(|e| e.1);
        d_corners_sort
    }

    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let mut d_corners_sort = self.get_sorted_corners();
        'corners: while d_corners_sort.len() > 0 {
            let (corner, _) = d_corners_sort.pop().unwrap();
            if corner.1 < 4 {
                if corner.2 == 0 {
                    for _y in 0..4 {
                        let y_cube = self.cube;
                        let mut y_put = get_put_move(_y, y);
                        self.cube = self.cube.apply_moves(&y_put);
                        for i in 0..4 {
                            let u_cube = self.cube;
                            let mut u_put = get_put_move(i, U);
                            self.cube = self.cube.apply_moves(&u_put);
                            let mut c_put = vec![R, U2, R3, U3, R, U, R3];
                            self.cube = self.cube.apply_moves(&c_put);
                            let mut y_put_r = get_put_move(_y, y3);
                            self.cube = self.cube.apply_moves(&y_put_r);
                            if BottomCornerSolver::is_solved_corner((
                                corner.0,
                                self.cube.cp[corner.0 as usize] as u8,
                                self.cube.co[corner.0 as usize],
                            )) {
                                solution.append(&mut y_put);
                                solution.append(&mut u_put);
                                solution.append(&mut c_put);
                                solution.append(&mut y_put_r);
                                d_corners_sort = self.get_sorted_corners();
                                continue 'corners;
                            }
                            self.cube = u_cube;
                        }
                        self.cube = y_cube;
                    }
                } else {
                    for _y in 0..4 {
                        let y_cube = self.cube;
                        let mut y_put = get_put_move(_y, y);
                        self.cube = self.cube.apply_moves(&y_put);
                        for i in 0..4 {
                            let u_cube = self.cube;
                            let mut u_put = get_put_move(i, U);
                            self.cube = self.cube.apply_moves(&u_put);
                            for mut c_put in [vec![R, U, R3], vec![F3, U3, F]] {
                                let c_cube = self.cube;
                                self.cube = self.cube.apply_moves(&c_put);
                                let mut y_put_r = get_put_move(_y, y3);
                                self.cube = self.cube.apply_moves(&y_put_r);
                                if BottomCornerSolver::is_solved_corner((
                                    corner.0,
                                    self.cube.cp[corner.0 as usize] as u8,
                                    self.cube.co[corner.0 as usize],
                                )) {
                                    solution.append(&mut y_put);
                                    solution.append(&mut u_put);
                                    solution.append(&mut c_put);
                                    solution.append(&mut y_put_r);
                                    d_corners_sort = self.get_sorted_corners();
                                    continue 'corners;
                                }
                                self.cube = c_cube;
                            }
                            self.cube = u_cube;
                        }
                        self.cube = y_cube;
                    }
                }
            } else {
                for _y in 0..4 {
                    let y_cube = self.cube;
                    let mut y_put = get_put_move(_y, y);
                    self.cube = self.cube.apply_moves(&y_put);
                    let mut c_get_put = vec![R, U, R3];
                    self.cube = self.cube.apply_moves(&c_get_put);
                    let mut y_put_r = get_put_move(_y, y3);
                    self.cube = self.cube.apply_moves(&y_put_r);
                    if self.cube.cp[0..4].contains(&corner.0) {
                        solution.append(&mut y_put);
                        solution.append(&mut c_get_put);
                        solution.append(&mut y_put_r);
                        d_corners_sort = self.get_sorted_corners();
                        continue 'corners;
                    }
                    self.cube = y_cube;
                }
            }
            d_corners_sort = self.get_sorted_corners();
        }
        solution
    }

    /// Check if the bottom layer's corner is solved.
    pub fn is_solved(&self) -> bool {
        let d_corners = get_corners_d(&self.cube);
        let mut solved = 0;
        for corner in d_corners {
            match corner {
                (DFR, 4, 0) | (DLF, 5, 0) | (DBL, 6, 0) | (DRB, 7, 0) => solved += 1,
                _ => {}
            };
        }
        if solved == 4 {
            return true;
        }
        false
    }
}

pub fn get_corners_d(cc: &CubieCube) -> Vec<(Corner, u8, u8)> {
    let mut d_corners = Vec::new();
    for corner in [DFR, DLF, DBL, DRB] {
        for i in 0..8 {
            if cc.cp[i] == corner {
                d_corners.push((corner, i as u8, cc.co[i]));
            }
        }
    }
    d_corners
}

#[cfg(test)]
mod tests {
    use crate::{
        cubie::CubieCube,
        moves::optimise_moves,
        scramble,
        solver::lbl::{bottom::BottomCornerSolver, cross::CrossSolver},
    };

    #[test]
    fn test_bottom_layer() {
        let cc = CubieCube::default();
        let moves = scramble();
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver::new(cc, true);
        let _cs = cross.solve();
        let mut bottom = BottomCornerSolver { cube: cross.cube };
        let _bs = bottom.solve();
        assert!(bottom.is_solved());
        let _bs = optimise_moves(&_bs);
        println!("Scramble: {:?}\nSolution: {:?}, {:?}", moves, _cs, _bs);
    }
}
