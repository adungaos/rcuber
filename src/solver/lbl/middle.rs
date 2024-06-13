use crate::cubie::Edge::{self, *};
use crate::solver::lbl::get_put_move;
use crate::{
    cubie::CubieCube,
    moves::Move::{self, *},
};

/// MiddleEdgeSolver for LBL(Layer by Layer) method, i.e, solve four middle layer edges(FR, FL, BL, BR).
/// # Example
/// ```rust
/// use rcuber::cubie::CubieCube;
/// use rcuber::scramble;
/// use rcuber::solver::lbl::cross::CrossSolver;
/// use rcuber::solver::lbl::bottom::BottomCornerSolver;
/// use rcuber::solver::lbl::middle::MiddleEdgeSolver;
///
/// fn main() {
///     let cc = CubieCube::default();
///     let moves = scramble();
///     let cc = cc.apply_moves(&moves);
///     let mut cross = CrossSolver::new(cc, true);
///     let _cs = cross.solve();
///     let mut bottom = BottomCornerSolver{cube: cross.cube};
///     let _bs = bottom.solve();
///     let mut middle = MiddleEdgeSolver{cube: bottom.cube};
///     let _ms = middle.solve();
///     assert!(middle.is_solved());
///     println!("Scramble: {:?}\nSolution: {:?}, {:?}, {:?}", moves, _cs, _bs, _ms);
/// }
/// ```
pub struct MiddleEdgeSolver {
    pub cube: CubieCube,
}

impl MiddleEdgeSolver {
    fn is_solved_edge(edge: (Edge, u8, u8)) -> bool {
        edge.0 as u8 == edge.1 && edge.2 == 0
    }

    fn get_sorted_edges(&self) -> Vec<((Edge, u8, u8), i32)> {
        let m_edges = get_edges_middle(&self.cube);
        let mut m_edges_sort = Vec::new();
        for edge in m_edges {
            if MiddleEdgeSolver::is_solved_edge(edge) {
                continue;
            }
            if edge.1 < 4 {
                m_edges_sort.push((edge, -1));
            } else {
                m_edges_sort.push((edge, -2));
            }
        }
        m_edges_sort.sort_by_key(|e| e.1);
        m_edges_sort
    }

    pub fn solve(&mut self) -> Vec<Move> {
        let mut solution = Vec::new();
        let mut m_edges_sort = self.get_sorted_edges();
        'edges: while m_edges_sort.len() > 0 {
            let (edge, _) = m_edges_sort.pop().unwrap();
            if edge.1 < 4 {
                for _y in 0..4 {
                    let mut y_put = get_put_move(_y, y);
                    let _cube = self.cube;
                    self.cube = self.cube.apply_moves(&y_put);
                    for i in 0..4 {
                        let mut u_put = get_put_move(i, U);
                        let _cube = self.cube;
                        self.cube = self.cube.apply_moves(&u_put);
                        for mut c_put in
                            [vec![R, U3, R3, U3, F3, U, F], vec![F3, U, F, U, R, U3, R3]]
                        {
                            let _cube = self.cube;
                            self.cube = self.cube.apply_moves(&c_put);
                            let mut y_put_r = get_put_move(_y, y3);
                            self.cube = self.cube.apply_moves(&y_put_r);
                            if MiddleEdgeSolver::is_solved_edge((
                                edge.0,
                                self.cube.ep[edge.0 as usize] as u8,
                                self.cube.eo[edge.0 as usize],
                            )) {
                                solution.append(&mut y_put);
                                solution.append(&mut u_put);
                                solution.append(&mut c_put);
                                solution.append(&mut y_put_r);
                                m_edges_sort = self.get_sorted_edges();
                                continue 'edges;
                            }
                            self.cube = _cube;
                        }
                        self.cube = _cube;
                    }
                    self.cube = _cube;
                }
            } else {
                for _y in 0..4 {
                    let mut y_put = get_put_move(_y, y);
                    let _cube = self.cube;
                    self.cube = self.cube.apply_moves(&y_put);
                    let mut c_get_put = vec![F3, U3, F, U, R, U, R3];
                    self.cube = self.cube.apply_moves(&c_get_put);
                    let mut y_put_r = get_put_move(_y, y3);
                    self.cube = self.cube.apply_moves(&y_put_r);
                    if self.cube.ep[0..4].contains(&edge.0) {
                        solution.append(&mut y_put);
                        solution.append(&mut c_get_put);
                        solution.append(&mut y_put_r);
                        m_edges_sort = self.get_sorted_edges();
                        continue 'edges;
                    }
                    self.cube = _cube;
                }
            }
            m_edges_sort = self.get_sorted_edges();
        }
        solution
    }

    /// Check if the middle layer's edge is solved.
    pub fn is_solved(&self) -> bool {
        let edges_m = get_edges_middle(&self.cube);
        let mut solved = 0;
        for edge in edges_m {
            match edge {
                (FR, 8, 0) | (FL, 9, 0) | (BL, 10, 0) | (BR, 11, 0) => solved += 1,
                _ => {}
            };
        }
        if solved == 4 {
            return true;
        }
        false
    }
}

pub fn get_edges_middle(cc: &CubieCube) -> Vec<(Edge, u8, u8)> {
    let mut edges = Vec::new();
    for edge in [FR, FL, BL, BR] {
        for i in 0..12 {
            if cc.ep[i] == edge {
                edges.push((edge, i as u8, cc.eo[i]));
            }
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use crate::{
        cubie::CubieCube,
        moves::optimise_moves,
        scramble,
        solver::lbl::{bottom::BottomCornerSolver, cross::CrossSolver, middle::MiddleEdgeSolver},
    };

    #[test]
    fn test_middle_layer() {
        let cc = CubieCube::default();
        let moves = scramble();
        let cc = cc.apply_moves(&moves);
        let mut cross = CrossSolver::new(cc, true);
        let _cs = cross.solve();
        let mut bottom = BottomCornerSolver { cube: cross.cube };
        let _bs = bottom.solve();
        let _bs = optimise_moves(&_bs);
        let mut middle = MiddleEdgeSolver { cube: bottom.cube };
        let _ms = middle.solve();
        assert!(middle.is_solved());
        let _ms = optimise_moves(&_ms);
        println!(
            "Scramble: {:?}\nSolution: {:?}, {:?}, {:?}",
            moves, _cs, _bs, _ms
        );
    }
}
