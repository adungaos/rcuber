use std::cmp::{max, min};

use super::constants::{
    MAX_DEPTH2, MAX_PRE_MOVES, MIN_P1LENGTH_PRE, N_COMB, N_MPERM, OPTIMAL_SOLUTION, TRY_INVERSE,
    TRY_THREE_AXES, USE_CONJ_PRUN,
};
use super::cubie::CubieCube;
use super::tables::{CT, IT, MT, PT, ST};
use super::utils::UT;
use super::{arraycube::ArrayCube, coord::CoordCube, utils::Solution};
use crate::error::Error;
use crate::facelet::FaceCube;
use crate::moves::Formula;

/// min2phase Solver.
/// # Example
/// ```rust
/// use rcuber::{facelet::FaceCube, generator::Generator};
/// use rcuber::solver::min2phase::solver::Solver;
///
/// fn main() {
///     let mut solver = Solver::default();
///     let s = solver
///         .solve(
///             "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL",
///             21,
///             100000,
///             0,
///             0x0,
///         )
///         .unwrap();
///     println!("{}", s);
///     let n = solver.next(500, 0, 0x4).unwrap();
///     println!("{}", n);
///     let n = solver.next(500, 0, 0x4).unwrap();
///     println!("{}", n);
///     let n = solver.next(1000, 0, 0x4).unwrap();
///     println!("{}", n);
/// }
/// ```
#[derive(Debug)]
pub struct Solver {
    moves: [i32; 31],
    node_ud: [CoordCube; 21],
    node_rl: [CoordCube; 21],
    node_fb: [CoordCube; 21],
    selfsym: u64,
    conj_mask: u32,
    urf_idx: usize,
    length1: usize,
    depth1: usize,
    max_dep2: usize,
    sol_len: usize,
    pub solution: Solution,
    probe: u64,
    probe_max: u64,
    probe_min: u64,
    verbose: usize,
    valid1: u32,
    allow_shorter: bool,
    cc: ArrayCube,
    urf_cubiecube: [ArrayCube; 6],
    urf_coordcube: [CoordCube; 6],
    phase1_cubie: [ArrayCube; 21],
    pre_move_cubes: [ArrayCube; MAX_PRE_MOVES + 1],
    pre_moves: [i32; MAX_PRE_MOVES],
    pre_move_len: usize,
    max_pre_moves: usize,
    is_rec: bool,
}

impl Default for Solver {
    fn default() -> Self {
        let node_ud = [CoordCube::default(); 21];
        let node_rl = [CoordCube::default(); 21];
        let node_fb = [CoordCube::default(); 21];
        let phase1_cubie = [ArrayCube::default(); 21];

        let urf_cubiecube = [ArrayCube::default(); 6];
        let urf_coordcube = [CoordCube::default(); 6];

        let pre_move_cubes = [ArrayCube::default(); MAX_PRE_MOVES + 1];
        Self {
            moves: [0; 31],
            node_ud,
            node_rl,
            node_fb,
            selfsym: 0,
            conj_mask: 0,
            urf_idx: 0,
            length1: 0,
            depth1: 0,
            max_dep2: 0,
            sol_len: 0,
            solution: Solution::new(),
            probe: 0,
            probe_max: 0,
            probe_min: 0,
            verbose: 0,
            valid1: 0,
            allow_shorter: false,
            cc: ArrayCube::default(),
            urf_cubiecube,
            urf_coordcube,
            phase1_cubie,
            pre_move_cubes: pre_move_cubes,
            pre_moves: [0; MAX_PRE_MOVES],
            pre_move_len: 0,
            max_pre_moves: 0,
            is_rec: false,
        }
    }
}

impl Solver {
    /// Solve cube to an expected state(goal falcelet).
    pub fn solve_to(
        &mut self,
        facelet: &str,
        goal: &str,
        max_depth: usize,
        probe_max: u64,
        probe_min: u64,
        verbose: usize,
    ) -> Result<Formula, Error> {
        let fc0 = FaceCube::try_from(facelet)?;
        let fcg = FaceCube::try_from(goal)?;
        let cc0 = CubieCube::try_from(&fc0)?;
        let s = cc0.verify()?;
        if s != true {
            return Err(Error::InvalidFaceletString); // no valid facelet cube, gives invalid cubie cube
        }
        let ccg = CubieCube::try_from(&fcg)?;
        let s = ccg.verify()?;
        if s != true {
            return Err(Error::InvalidFaceletString); // no valid facelet cube, gives invalid cubie cube
        }
        // cc0 * S = ccg  <=> (ccg^-1 * cc0) * S = Id
        let mut cc = ccg.inverse_cubie_cube();
        cc.multiply(cc0);
        self.solve(
            FaceCube::try_from(&cc).unwrap().to_string().as_str(),
            max_depth,
            probe_max,
            probe_min,
            verbose,
        )
    }

    /// Solve cube to solved state.
    pub fn solve(
        &mut self,
        facelet: &str,
        max_depth: usize,
        probe_max: u64,
        probe_min: u64,
        verbose: usize,
    ) -> Result<Formula, Error> {
        let cubestate = self.feed_with_verify(facelet);
        if cubestate.is_err() {
            return Err(Error::InvalidFaceletString);
        }
        self.sol_len = max_depth + 1;
        self.probe = 0;
        self.probe_max = probe_max;
        self.probe_min = min(probe_min, probe_max);
        self.verbose = verbose;
        self.solution = Solution::new();
        self.is_rec = false;
        self.init_search();
        match (verbose & OPTIMAL_SOLUTION) == 0 {
            true => self.search(),
            false => self.search_opt(),
        }
    }

    /// feed facelet to self.cc and verify if its a solvable cube.
    pub fn feed_with_verify(&mut self, facelet: &str) -> Result<bool, Error> {
        self.cc = ArrayCube::from(facelet);
        self.cc.verify()
    }

    fn init_search(&mut self) {
        self.conj_mask = match TRY_INVERSE {
            true => 0,
            false => 0x38,
        };
        self.conj_mask |= match TRY_THREE_AXES {
            true => 0,
            false => 0x36,
        };
        self.selfsym = self.cc.symmetry();
        self.conj_mask |= match (self.selfsym >> 16 & 0xffff) != 0 {
            true => 0x12,
            false => 0,
        };
        self.conj_mask |= match (self.selfsym >> 32 & 0xffff) != 0 {
            true => 0x24,
            false => 0,
        };
        self.conj_mask |= match (self.selfsym >> 48 & 0xffff) != 0 {
            true => 0x38,
            false => 0,
        };
        self.selfsym &= 0xffffffffffffu64;
        self.max_pre_moves = match self.conj_mask > 7 {
            true => 0,
            false => MAX_PRE_MOVES,
        };
        for i in 0..6 {
            self.urf_cubiecube[i] = self.cc.clone();
            self.urf_coordcube[i].set_with_prun(self.urf_cubiecube[i], 20);
            self.cc = self.cc.urf_conjugate();
            if i % 3 == 2 {
                self.cc = self.cc.inverse_cube();
            }
        }
    }

    /// Continue search to find more optimal solution.
    pub fn next(
        &mut self,
        probe_max: u64,
        probe_min: u64,
        verbose: usize,
    ) -> Result<Formula, Error> {
        self.probe = 0;
        self.probe_max = probe_max;
        self.probe_min = min(probe_min, probe_max);
        self.solution = Solution::new();
        self.is_rec = (self.verbose & OPTIMAL_SOLUTION) == (verbose & OPTIMAL_SOLUTION);
        self.verbose = verbose;
        match (verbose & OPTIMAL_SOLUTION) == 0 {
            true => self.search(),
            false => self.search_opt(),
        }
    }

    fn phase1_pre_moves(&mut self, max1: usize, lm: i32, ac: ArrayCube, ssym: u64) -> u32 {
        self.pre_move_len = self.max_pre_moves - max1;
        if (self.is_rec && (self.depth1 == self.length1 - self.pre_move_len))
            || (!self.is_rec && (self.pre_move_len == 0 || (0x36FB7 >> lm & 1) == 0))
        {
            self.depth1 = self.length1 - self.pre_move_len;
            self.phase1_cubie[0] = ac;
            self.allow_shorter = self.depth1 == MIN_P1LENGTH_PRE && self.pre_move_len != 0;
            if self.node_ud[self.depth1 + 1].set_with_prun(ac, self.depth1 as i32)
                && self.phase1(self.node_ud[self.depth1 + 1], ssym, self.depth1, -1) == 0
            {
                return 0;
            }
        }
        if max1 == 0 || self.pre_move_len + MIN_P1LENGTH_PRE >= self.length1 {
            return 1;
        }
        let mut skip_moves = ArrayCube::get_skip_moves(ssym);
        if max1 == 1 || self.pre_move_len + 1 + MIN_P1LENGTH_PRE >= self.length1 {
            // last pre move
            skip_moves |= 0x36FB7; // 11 0110 1111 1011 0111
        }
        let lm = lm / 3 * 3;
        let mut m = 0;
        while m < 18 {
            if m == lm || m == lm - 9 || m == lm + 9 {
                m += 2;
                m += 1;
                continue;
            }
            if self.is_rec && m != self.pre_moves[self.max_pre_moves - max1]
                || (skip_moves & 1 << m) != 0
            {
                m += 1;
                continue;
            }
            self.pre_move_cubes[max1] = MT.move_cube[m as usize].multiply(&ac);
            self.pre_moves[self.max_pre_moves - max1] = m;
            if self.phase1_pre_moves(
                max1 - 1,
                m,
                self.pre_move_cubes[max1],
                ssym & ST.move_cube_sym[m as usize],
            ) == 0
            {
                return 0;
            }
            m += 1;
        }
        1
    }

    fn search(&mut self) -> Result<Formula, Error> {
        self.length1 = match self.is_rec {
            true => self.length1,
            false => 0,
        };
        while self.length1 < self.sol_len {
            self.max_dep2 = min(MAX_DEPTH2, self.sol_len - self.length1 - 1);
            self.urf_idx = match self.is_rec {
                true => self.urf_idx,
                false => 0,
            };
            while self.urf_idx < 6 {
                if (self.conj_mask & 1 << self.urf_idx) != 0 {
                    self.urf_idx += 1;
                    continue;
                }
                if self.phase1_pre_moves(
                    self.max_pre_moves,
                    -30,
                    self.urf_cubiecube[self.urf_idx],
                    self.selfsym & 0xffff,
                ) == 0
                {
                    match self.solution.length {
                        0 => return Err(Error::ProbeLimitExceeded),
                        _ => {
                            return Ok(Formula {
                                moves: self.solution.to_vec(),
                            })
                        }
                    }
                }
                self.urf_idx += 1;
            }
            self.length1 += 1;
        }
        match self.solution.length {
            0 => return Err(Error::NoSolutionForMaxDepth),
            _ => {
                return Ok(Formula {
                    moves: self.solution.to_vec(),
                })
            }
        }
    }

    fn search_opt(&mut self) -> Result<Formula, Error> {
        let mut maxprun1 = 0;
        let mut maxprun2 = 0;
        for i in 0..6 {
            self.urf_coordcube[i].calc_pruning(false);
            if i < 3 {
                maxprun1 = max(maxprun1, self.urf_coordcube[i].prun);
            } else {
                maxprun2 = max(maxprun2, self.urf_coordcube[i].prun);
            }
        }
        self.urf_idx = match maxprun2 > maxprun1 {
            true => 3,
            false => 0,
        };
        self.phase1_cubie[0] = self.urf_cubiecube[self.urf_idx];
        self.length1 = match self.is_rec {
            true => self.length1,
            false => 0,
        };
        while self.length1 < self.sol_len {
            let ud = self.urf_coordcube[0 + self.urf_idx];
            let rl = self.urf_coordcube[1 + self.urf_idx];
            let fb = self.urf_coordcube[2 + self.urf_idx];
            if ud.prun <= self.length1 as i32
                && rl.prun <= self.length1 as i32
                && fb.prun < self.length1 as i32
                && self.phase1_opt(ud, rl, fb, self.selfsym, self.length1, -1) == 0
            {
                return match self.solution.length == 0 {
                    true => Err(Error::ProbeLimitExceeded),
                    false => Ok(Formula {
                        moves: self.solution.to_vec(),
                    }),
                };
            }
            self.length1 += 1;
        }
        match self.solution.length == 0 {
            true => Err(Error::NoSolutionForMaxDepth),
            false => Ok(Formula {
                moves: self.solution.to_vec(),
            }),
        }
    }

    ///     0: Found or Probe limit exceeded
    ///     1: at least 1 + maxDep2 moves away, Try next power
    ///     2: at least 2 + maxDep2 moves away, Try next axis
    fn init_phase2_pre(&mut self) -> u32 {
        self.is_rec = false;
        let _probe = match self.solution.length {
            0 => self.probe_max,
            _ => self.probe_min,
        };
        if self.probe >= _probe {
            return 0;
        }
        self.probe += 1;

        for i in (self.valid1 as usize)..self.depth1 {
            self.phase1_cubie[i + 1] =
                self.phase1_cubie[i].multiply(&MT.move_cube[self.moves[i] as usize]);
        }
        self.valid1 = self.depth1 as u32;
        let mut p2corn = self.phase1_cubie[self.depth1].get_perm_corner_sym();
        let mut p2csym = p2corn & 0xf;
        p2corn >>= 4;
        let mut p2edge = self.phase1_cubie[self.depth1].get_perm_edge_sym();
        let mut p2esym = p2edge & 0xf;
        p2edge >>= 4;
        let mut p2mid = self.phase1_cubie[self.depth1].get_perm_m();
        let mut edgei = ArrayCube::get_perm_sym_inv(p2edge as usize, p2esym as usize, false);
        let mut corni = ArrayCube::get_perm_sym_inv(p2corn, p2csym, true);
        let last_move = match self.depth1 == 0 {
            true => -1,
            false => self.moves[self.depth1 - 1],
        };
        let last_pre = match self.pre_move_len == 0 {
            true => -1,
            false => self.pre_moves[self.pre_move_len - 1],
        };
        let mut ret = 0;
        let p2switch_max = match self.pre_move_len == 0 {
            true => 1,
            false => 2,
        } * match self.depth1 == 0 {
            true => 1,
            false => 2,
        };

        let mut p2switch = 0;
        let mut p2switch_mask = (1 << p2switch_max) - 1;
        while p2switch < p2switch_max {
            if (p2switch_mask >> p2switch & 1) != 0 {
                p2switch_mask &= !(1 << p2switch);
                ret = self.init_phase2(p2corn, p2csym, p2edge, p2esym, p2mid, edgei, corni);
                if ret == 0 || ret > 2 {
                    break;
                } else if ret == 2 {
                    p2switch_mask &= 0x4 << p2switch; // 0->2; 1=>3; 2=>N/A
                }
            }
            if p2switch_mask == 0 {
                break;
            }
            if (p2switch & 1) == 0 && self.depth1 > 0 {
                let m = UT.std2ud[(last_move / 3 * 3 + 1) as usize];
                self.moves[self.depth1 - 1] =
                    (UT.ud2std[m as usize] as usize * 2) as i32 - self.moves[self.depth1 - 1];
                p2mid = CT.mperm_move[p2mid][m] as usize;
                p2corn =
                    CT.cperm_move[p2corn][ST.sym_move_ud[p2csym][m as usize] as usize] as usize;
                p2csym = MT.sym_mult[(p2corn & 0xf) as usize][p2csym] as usize;
                p2corn >>= 4;
                p2edge = CT.eperm_move[p2edge as usize]
                    [ST.sym_move_ud[p2esym as usize][m as usize] as usize];
                p2esym = MT.sym_mult[p2edge as usize & 0xf][p2esym as usize] as u16;
                p2edge >>= 4;
                corni = ArrayCube::get_perm_sym_inv(p2corn, p2csym, true);
                edgei = ArrayCube::get_perm_sym_inv(p2edge as usize, p2esym as usize, false);
            } else if self.pre_move_len > 0 {
                let m = UT.std2ud[(last_pre / 3 * 3 + 1) as usize];
                self.pre_moves[self.pre_move_len - 1] =
                    UT.ud2std[m] as i32 * 2 - self.pre_moves[self.pre_move_len - 1];
                p2mid =
                    IT.mperm_inv[CT.mperm_move[IT.mperm_inv[p2mid] as usize][m] as usize] as usize;
                p2corn = CT.cperm_move[corni as usize >> 4]
                    [ST.sym_move_ud[corni as usize & 0xf][m] as usize]
                    as usize;
                corni =
                    p2corn as u16 & !0xf | MT.sym_mult[p2corn & 0xf][corni as usize & 0xf] as u16;
                p2corn =
                    ArrayCube::get_perm_sym_inv(corni as usize >> 4, corni as usize & 0xf, true)
                        as usize;
                p2csym = p2corn & 0xf;
                p2corn >>= 4;
                p2edge = CT.eperm_move[edgei as usize >> 4]
                    [ST.sym_move_ud[edgei as usize & 0xf][m] as usize];
                edgei =
                    p2edge & !0xf | MT.sym_mult[p2edge as usize & 0xf][edgei as usize & 0xf] as u16;
                p2edge =
                    ArrayCube::get_perm_sym_inv(edgei as usize >> 4, edgei as usize & 0xf, false);
                p2esym = p2edge & 0xf;
                p2edge >>= 4;
            }
            p2switch += 1;
        }
        if self.depth1 > 0 {
            self.moves[self.depth1 - 1] = last_move;
        }
        if self.pre_move_len > 0 {
            self.pre_moves[self.pre_move_len - 1] = last_pre;
        }
        match ret == 0 {
            true => 0,
            false => 2,
        }
    }

    fn init_phase2(
        &mut self,
        p2corn: usize,
        p2csym: usize,
        p2edge: u16,
        p2esym: u16,
        p2mid: usize,
        edgei: u16,
        corni: u16,
    ) -> u32 {
        let prun = max(
            CoordCube::get_pruning(
                &PT.eperm_ccombp_prun,
                (edgei as usize >> 4) * N_COMB
                    + CT.ccombp_conj[IT.perm2_comb_p[corni as usize >> 4] as usize & 0xff]
                        [MT.sym_mult_inv[edgei as usize & 0xf][corni as usize & 0xf] as usize]
                        as usize,
            ),
            max(
                CoordCube::get_pruning(
                    &PT.eperm_ccombp_prun,
                    p2edge as usize * N_COMB
                        + CT.ccombp_conj[IT.perm2_comb_p[p2corn as usize] as usize & 0xff]
                            [MT.sym_mult_inv[p2esym as usize][p2csym as usize] as usize]
                            as usize,
                ),
                CoordCube::get_pruning(
                    &PT.mcperm_prun,
                    p2corn * N_MPERM + CT.mperm_conj[p2mid][p2csym] as usize,
                ),
            ),
        );
        if prun > self.max_dep2 as i32 {
            return prun as u32 - self.max_dep2 as u32;
        }

        let mut depth2 = self.max_dep2;
        while depth2 >= prun as usize {
            let ret = self.phase2(
                p2edge,
                p2esym,
                p2corn,
                p2csym,
                p2mid,
                depth2,
                self.depth1,
                10,
            );
            if ret < 0 {
                break;
            }
            depth2 -= ret as usize;
            self.sol_len = 0;
            self.solution = Solution::new();
            self.solution
                .set_args(self.verbose, self.urf_idx, self.depth1);
            for i in 0..self.depth1 + depth2 {
                self.solution.append_sol_move(self.moves[i]);
            }
            for i in (0..self.pre_move_len).rev() {
                self.solution.append_sol_move(self.pre_moves[i]);
            }
            self.sol_len = self.solution.length;
            depth2 -= 1;
        }
        if depth2 != self.max_dep2 {
            //At least one solution has been found.
            self.max_dep2 = min(MAX_DEPTH2, self.sol_len - self.length1 - 1);
            return match self.probe >= self.probe_min {
                true => 0,
                false => 1,
            };
        }
        1
    }

    /// 0: Found or Probe limit exceeded
    /// 1: Try Next Power
    /// 2: Try Next Axis
    fn phase1(&mut self, node: CoordCube, ssym: u64, maxl: usize, lm: i32) -> u32 {
        if node.prun == 0 && maxl < 5 {
            if self.allow_shorter || maxl == 0 {
                self.depth1 -= maxl;
                let ret = self.init_phase2_pre();
                self.depth1 += maxl;
                return ret;
            } else {
                return 1;
            }
        }
        let skip_moves = ArrayCube::get_skip_moves(ssym);
        for axis in 0..6 {
            let axis = axis * 3;
            if axis == lm || axis == lm - 9 {
                continue;
            }
            for power in 0..3 {
                let m = axis + power;
                if self.is_rec && m != self.moves[self.depth1 - maxl]
                    || skip_moves != 0 && (skip_moves & 1 << m) != 0
                {
                    continue;
                }

                let mut prun = self.node_ud[maxl].do_move_prun(node, m as usize, true);
                if prun > maxl as i32 {
                    break;
                } else if prun == maxl as i32 {
                    continue;
                }

                if USE_CONJ_PRUN {
                    prun = self.node_ud[maxl].do_move_prun_conj(node, m as usize);
                    if prun > maxl as i32 {
                        break;
                    } else if prun == maxl as i32 {
                        continue;
                    }
                }
                self.moves[self.depth1 - maxl] = m;
                self.valid1 = min(self.valid1, (self.depth1 - maxl) as u32);
                let ret = self.phase1(
                    self.node_ud[maxl],
                    ssym & ST.move_cube_sym[m as usize],
                    maxl - 1,
                    axis,
                );
                if ret == 0 {
                    return 0;
                } else if ret >= 2 {
                    break;
                }
            }
        }
        1
    }

    fn phase1_opt(
        &mut self,
        ud: CoordCube,
        rl: CoordCube,
        fb: CoordCube,
        ssym: u64,
        maxl: usize,
        lm: i32,
    ) -> u32 {
        if ud.prun == 0 && rl.prun == 0 && fb.prun == 0 && maxl < 5 {
            self.max_dep2 = maxl;
            self.depth1 = self.length1 - maxl;
            return match self.init_phase2_pre() == 0 {
                true => 0,
                false => 1,
            };
        }

        let skip_moves = ArrayCube::get_skip_moves(ssym);
        for axis in 0..6 {
            let axis: usize = axis * 3;
            if axis == lm as usize || (axis as i32) == lm - 9 {
                continue;
            }
            for power in 0..3 {
                let mut m = axis + power;
                if self.is_rec && m != self.moves[self.length1 - maxl] as usize
                    || skip_moves != 0 && (skip_moves & 1 << m) != 0
                {
                    continue;
                }
                // UD Axis
                let prun_ud = max(
                    self.node_ud[maxl].do_move_prun(ud, m, false),
                    self.node_ud[maxl].do_move_prun_conj(ud, m),
                ) as usize;
                if prun_ud > maxl {
                    break;
                } else if prun_ud == maxl {
                    continue;
                }

                //RL Axis
                m = MT.urf_move[2][m] as usize;
                let prun_rl = max(
                    self.node_rl[maxl].do_move_prun(rl, m, false),
                    self.node_rl[maxl].do_move_prun_conj(rl, m),
                ) as usize;
                if prun_rl > maxl {
                    break;
                } else if prun_rl == maxl {
                    continue;
                }
                //FB Axis
                m = MT.urf_move[2][m] as usize;
                let mut prun_fb = max(
                    self.node_fb[maxl].do_move_prun(fb, m, false),
                    self.node_fb[maxl].do_move_prun_conj(fb, m),
                ) as usize;
                if prun_ud == prun_rl && prun_rl == prun_fb && prun_fb != 0 {
                    prun_fb += 1;
                }
                if prun_fb > maxl {
                    break;
                } else if prun_fb == maxl {
                    continue;
                }
                m = MT.urf_move[2][m] as usize;

                self.moves[self.length1 - maxl] = m as i32;
                self.valid1 = min(self.valid1, (self.length1 - maxl) as u32);
                if self.phase1_opt(
                    self.node_ud[maxl],
                    self.node_rl[maxl],
                    self.node_fb[maxl],
                    ssym & ST.move_cube_sym[m],
                    maxl - 1,
                    axis as i32,
                ) == 0
                {
                    return 0;
                }
            }
        }
        1
    }

    //-1: no solution found
    // X: solution with X moves shorter than expectation. Hence, the length of the solution is  depth - X
    fn phase2(
        &mut self,
        edge: u16,
        esym: u16,
        corn: usize,
        csym: usize,
        mid: usize,
        maxl: usize,
        depth: usize,
        lm: usize,
    ) -> i32 {
        if edge == 0 && corn == 0 && mid == 0 {
            return maxl as i32;
        }
        let move_mask = UT.ckmv2bit[lm];
        let mut m: i32 = 0;
        while m < 10 {
            if (move_mask >> m & 1) != 0 {
                m += 0x42 >> m & 3;
                m += 1;
                continue;
            }
            let midx = CT.mperm_move[mid][m as usize];
            let mut cornx = CT.cperm_move[corn][ST.sym_move_ud[csym][m as usize] as usize];
            let csymx = MT.sym_mult[cornx as usize & 0xf][csym];
            cornx >>= 4;
            let mut edgex =
                CT.eperm_move[edge as usize][ST.sym_move_ud[esym as usize][m as usize] as usize];
            let esymx = MT.sym_mult[edgex as usize & 0xf][esym as usize];
            edgex >>= 4;
            let edgei = ArrayCube::get_perm_sym_inv(edgex as usize, esymx as usize, false);
            let corni = ArrayCube::get_perm_sym_inv(cornx as usize, csymx as usize, true);
            let mut prun = CoordCube::get_pruning(
                &PT.eperm_ccombp_prun,
                (edgei as usize >> 4) * N_COMB
                    + CT.ccombp_conj[IT.perm2_comb_p[corni as usize >> 4] as usize & 0xff]
                        [MT.sym_mult_inv[edgei as usize & 0xf][corni as usize & 0xf] as usize]
                        as usize,
            );
            if prun > maxl as i32 + 1 {
                return maxl as i32 - prun + 1;
            } else if prun >= maxl as i32 {
                m += 0x42 >> m & 3 & (maxl as i32 - prun);
                m += 1;
                continue;
            }
            prun = max(
                CoordCube::get_pruning(
                    &PT.mcperm_prun,
                    cornx as usize * N_MPERM
                        + CT.mperm_conj[midx as usize][csymx as usize] as usize,
                ),
                CoordCube::get_pruning(
                    &PT.eperm_ccombp_prun,
                    edgex as usize * N_COMB
                        + CT.ccombp_conj[IT.perm2_comb_p[cornx as usize] as usize & 0xff]
                            [MT.sym_mult_inv[esymx as usize][csymx as usize] as usize]
                            as usize,
                ),
            );
            if prun >= maxl as i32 {
                m += 0x42 >> m & 3 & (maxl as i32 - prun);
                m += 1;
                continue;
            }
            let depthx = depth + 1;
            let maxlx = maxl - 1;
            let ret = self.phase2(
                edgex,
                esymx as u16,
                cornx as usize,
                csymx as usize,
                midx as usize,
                maxlx,
                depthx,
                m as usize,
            );
            if ret >= 0 {
                self.moves[depth] = UT.ud2std[m as usize] as i32;
                return ret;
            }
            if ret < -2 {
                break;
            }
            if ret < -1 {
                m += 0x42 >> m & 3;
            }
            m += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use crate::{facelet::FaceCube, generator::Generator};

    #[test]
    fn test_solver() {
        let mut solver = Solver::default();
        let s = solver
            .solve(
                "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL",
                21,
                100000,
                0,
                0x4,
            )
            .unwrap();
        println!("{}", s);
        let n = solver.next(500, 0, 0x4).unwrap();
        println!("{}", n);
        let n = solver.next(500, 0, 0x4).unwrap();
        println!("{}", n);
        let n = solver.next(1000, 0, 0x4).unwrap();
        println!("{}", n);
    }

    #[test]
    fn test_solve_to() {
        let gc = Generator::superflip();
        let mut solver = Solver::default();
        let s = solver
            .solve_to(
                "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB",
                FaceCube::try_from(&gc).unwrap().to_string().as_str(),
                21,
                100000,
                10000,
                0,
            )
            .unwrap();
        println!("{}", s);
    }
}
