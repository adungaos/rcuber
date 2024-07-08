use super::arraycube::{ArrayCube, PermOriCube};
use super::constants::*;
use super::coord::CoordCube;
use super::utils::UT;
use static_init::dynamic;

/// Move data tables.
#[derive(Debug)]
pub struct MoveTables {
    pub move_cube: [ArrayCube; 18],
    pub urf1: ArrayCube,
    pub urf2: ArrayCube,
    pub urf_move: [[u8; 18]; 6],
    pub cube_sym: [ArrayCube; 16],
    pub sym_mult: [[u32; 16]; 16],
    pub sym_mult_inv: [[u32; 16]; 16],
}

impl MoveTables {
    pub fn new() -> Self {
        let mut move_cube: [ArrayCube; 18] = [ArrayCube::default(); 18];
        let urf1: ArrayCube = ArrayCube::from(PermOriCube {
            perm_corner: 2531,
            perm_edge: 67026819,
            twist: 1373,
            flip: 1367,
        });
        let urf2: ArrayCube = ArrayCube::from(PermOriCube {
            perm_corner: 2089,
            perm_edge: 322752913,
            twist: 1906,
            flip: 2040,
        });
        let urf_move: [[u8; 18]; 6] = [
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
            [6, 7, 8, 0, 1, 2, 3, 4, 5, 15, 16, 17, 9, 10, 11, 12, 13, 14],
            [3, 4, 5, 6, 7, 8, 0, 1, 2, 12, 13, 14, 15, 16, 17, 9, 10, 11],
            [2, 1, 0, 5, 4, 3, 8, 7, 6, 11, 10, 9, 14, 13, 12, 17, 16, 15],
            [8, 7, 6, 2, 1, 0, 5, 4, 3, 17, 16, 15, 11, 10, 9, 14, 13, 12],
            [5, 4, 3, 8, 7, 6, 2, 1, 0, 14, 13, 12, 17, 16, 15, 11, 10, 9],
        ];

        // initMove
        move_cube[0] = ArrayCube::from(PermOriCube {
            perm_corner: 15120,
            twist: 0,
            perm_edge: 119750400,
            flip: 0,
        });
        move_cube[3] = ArrayCube::from(PermOriCube {
            perm_corner: 21021,
            twist: 1494,
            perm_edge: 323403417,
            flip: 0,
        });
        move_cube[6] = ArrayCube::from(PermOriCube {
            perm_corner: 8064,
            twist: 1236,
            perm_edge: 29441808,
            flip: 550,
        });
        move_cube[9] = ArrayCube::from(PermOriCube {
            perm_corner: 9,
            twist: 0,
            perm_edge: 5880,
            flip: 0,
        });
        move_cube[12] = ArrayCube::from(PermOriCube {
            perm_corner: 1230,
            twist: 412,
            perm_edge: 2949660,
            flip: 0,
        });
        move_cube[15] = ArrayCube::from(PermOriCube {
            perm_corner: 224,
            twist: 137,
            perm_edge: 328552,
            flip: 137,
        });

        let mut cube_sym: [ArrayCube; 16] = [ArrayCube::default(); 16];
        for _a in 0..6 {
            let a = _a * 3;
            for p in 0..2 {
                move_cube[a + p + 1] = ArrayCube::default();
                move_cube[a + p + 1].ea = move_cube[a + p].edge_multiply(&move_cube[a]).ea;
                move_cube[a + p + 1].ca = move_cube[a + p].corner_multiply(&move_cube[a]).ca;
            }
        }

        let mut c = ArrayCube::default();
        let f2 = ArrayCube::from(PermOriCube {
            perm_corner: 28783,
            twist: 0,
            perm_edge: 259268407,
            flip: 0,
        });
        let u4 = ArrayCube::from(PermOriCube {
            perm_corner: 15138,
            twist: 0,
            perm_edge: 119765538,
            flip: 7,
        });
        let mut lr2 = ArrayCube::from(PermOriCube {
            perm_corner: 5167,
            twist: 0,
            perm_edge: 83473207,
            flip: 0,
        });
        for i in 0..8 {
            lr2.ca[i] |= 3 << 3;
        }
        for i in 0..16 {
            cube_sym[i] = c.clone();
            c = c.multiply_full(&u4);
            if i % 4 == 3 {
                c = c.multiply_full(&lr2);
            }
            if i % 8 == 7 {
                c = c.multiply_full(&f2);
            }
        }

        let mut sym_mult: [[u32; 16]; 16] = [[0; 16]; 16];
        let mut sym_mult_inv: [[u32; 16]; 16] = [[0; 16]; 16];
        for i in 0..16 {
            for j in 0..16 {
                c.ca = cube_sym[i].corner_multiply_full(&cube_sym[j]).ca;
                for k in 0..16 {
                    if cube_sym[k].ca == c.ca {
                        sym_mult[i][j] = k as u32; // SymMult[i][j] = (k ^ i ^ j ^ (0x14ab4 >> j & i << 1 & 2)));
                        sym_mult_inv[k][j] = i as u32; // i * j = k => k * j^-1 = i
                        break;
                    }
                }
            }
        }

        Self {
            move_cube,    // DONE!
            urf1,         // DONE!
            urf2,         // DONE!
            urf_move,     // DONE!
            cube_sym,     // DONE!
            sym_mult,     // DONE!
            sym_mult_inv, // DONE!
        }
    }
}

/// Symmetries data tables.
#[derive(Debug)]
pub struct SymTables {
    pub move_cube_sym: [u64; 18],
    pub first_move_sym: [u32; 48],
    pub sym_move: [[u32; 18]; 16],
    pub sym8_move: [u32; 8 * 18],
    pub sym_move_ud: [[u32; 18]; 16],
}

impl SymTables {
    pub fn new() -> Self {
        let mut move_cube_sym: [u64; 18] = [0; 18];
        let mut first_move_sym: [u32; 48] = [0; 48];
        let mut sym_move: [[u32; 18]; 16] = [[0; 18]; 16];
        let mut sym8_move: [u32; 8 * 18] = [0; 8 * 18];
        let mut sym_move_ud: [[u32; 18]; 16] = [[0; 18]; 16];
        let move_cube = MT.move_cube;

        // initSym
        let mut c;
        for j in 0..18 {
            for s in 0..16 {
                c = move_cube[j].corner_conjugate(MT.sym_mult_inv[0][s] as usize);
                for m in 0..18 {
                    if move_cube[m].ca == c.ca {
                        sym_move[s][j] = m as u32;
                        sym_move_ud[s][UT.std2ud[j]] = UT.std2ud[m] as u32;
                        break;
                    }
                }
                if s % 2 == 0 {
                    sym8_move[j << 3 | s >> 1] = sym_move[s][j];
                }
            }
        }

        for i in 0..18 {
            move_cube_sym[i] = move_cube[i].symmetry();
            let mut j = i;
            for s in 0..48 {
                if sym_move[s % 16][j] < i as u32 {
                    first_move_sym[s] |= 1 << i;
                }
                if s % 16 == 15 {
                    j = MT.urf_move[2][j] as usize;
                }
            }
        }

        Self {
            move_cube_sym,  // DONE!
            first_move_sym, // DONE!
            sym_move,       // DONE!
            sym8_move,      // DONE!
            sym_move_ud,    // DONE!
        }
    }

    pub fn esym2csym(idx: usize) -> usize {
        idx ^ (SYM_E2C_MAGIC >> ((idx & 0xf) << 1) & 3)
    }
}

/// Symmetry to Raw data tables.
#[derive(Debug)]
pub struct Sym2RawTables {
    pub flip_s2r: Vec<u16>,
    pub twist_s2r: Vec<u16>,
    pub eperm_s2r: Vec<u16>,
    pub flip_r2s: Vec<u16>,
    pub twist_r2s: Vec<u16>,
    pub eperm_r2s: Vec<u16>,
    pub flip_s2rf: Vec<u16>,
    pub sym_state_twist: Vec<u16>,
    pub sym_state_flip: Vec<u16>,
    pub sym_state_perm: Vec<u16>,
}

impl Sym2RawTables {
    pub fn new() -> Self {
        let mut flip_s2rf: Vec<u16> = vec![0; N_FLIP_SYM * 8];

        // init_perm_sym2raw
        let mut eperm_s2r = vec![0; N_PERM_SYM];
        let mut eperm_r2s = vec![0; N_PERM];
        let mut sym_state_perm: Vec<u16> = vec![0; N_PERM_SYM];
        Sym2RawTables::init_sym2raw(
            N_PERM,
            &mut eperm_s2r,
            &mut eperm_r2s,
            &mut sym_state_perm,
            &mut flip_s2rf,
            2,
        );

        // init_flip_sym2raw
        let mut flip_s2r: Vec<u16> = vec![0; N_FLIP_SYM];
        let mut flip_r2s: Vec<u16> = vec![0; N_FLIP];
        let mut sym_state_flip: Vec<u16> = vec![0; N_FLIP_SYM];
        Sym2RawTables::init_sym2raw(
            N_FLIP,
            &mut flip_s2r,
            &mut flip_r2s,
            &mut sym_state_flip,
            &mut flip_s2rf,
            0,
        );

        // init_twist_sym2raw
        let mut twist_s2r: Vec<u16> = vec![0; N_TWIST_SYM];
        let mut twist_r2s: Vec<u16> = vec![0; N_TWIST];
        let mut sym_state_twist: Vec<u16> = vec![0; N_TWIST_SYM];
        Sym2RawTables::init_sym2raw(
            N_TWIST,
            &mut twist_s2r,
            &mut twist_r2s,
            &mut sym_state_twist,
            &mut flip_s2rf,
            1,
        );

        Self {
            flip_s2r,        // DONE!
            twist_s2r,       // DONE!
            eperm_s2r,       // DONE!
            flip_r2s,        // DONE!
            twist_r2s,       // DONE!
            eperm_r2s,       // DONE!
            flip_s2rf,       // DONE!
            sym_state_twist, // DONE!
            sym_state_flip,  // DONE!
            sym_state_perm,  // DONE!
        }
    }

    pub fn init_sym2raw(
        n_raw: usize,
        sym2raw: &mut Vec<u16>,
        raw2sym: &mut Vec<u16>,
        sym_state: &mut Vec<u16>,
        flip_s2rf: &mut Vec<u16>,
        coord: u32,
    ) -> usize {
        let _n_raw_half = (n_raw + 1) / 2;
        let mut c = ArrayCube::default();
        let mut d;
        let mut count = 0;
        let mut idx;
        let sym_inc = match coord >= 2 {
            true => 1,
            false => 2,
        };
        let is_edge = coord != 1;

        for i in 0..n_raw {
            if raw2sym[i] != 0 {
                continue;
            }
            match coord {
                0 => c.set_flip(i as u16),
                1 => c.set_twist(i as u16),
                _ => c.set_perm_edge_ud(i),
            }
            for s in 0..16 {
                if s % sym_inc != 0 {
                    continue;
                }
                if is_edge {
                    d = c.edge_conjugate(s);
                } else {
                    d = c.corner_conjugate(s);
                }
                match coord {
                    0 => idx = d.get_flip(),
                    1 => idx = d.get_twist(),
                    _ => idx = d.get_perm_edge_ud() as u16,
                }
                if coord == 0 {
                    flip_s2rf[count << 3 | s >> 1] = idx;
                }
                if idx == i as u16 {
                    sym_state[count] |= 1 << (s / sym_inc);
                }
                let sym_idx = (count << 4 | s) / sym_inc;
                raw2sym[idx as usize] = sym_idx as u16;
            }
            sym2raw[count] = i as u16;
            count += 1;
        }
        count
    }
}

/// Inverse data tables.
#[derive(Debug)]
pub struct InvTables {
    pub perm2_comb_p: Vec<u8>,
    pub perm_inv_edge_sym: Vec<u16>,
    pub mperm_inv: Vec<u8>,
}

impl InvTables {
    pub fn new() -> Self {
        let mut perm2_comb_p: Vec<u8> = vec![0; N_PERM_SYM];
        let mut perm_inv_edge_sym: Vec<u16> = vec![0; N_PERM_SYM];
        let mut mperm_inv: Vec<u8> = vec![0; N_MPERM];
        let mut cc = ArrayCube::default();
        for i in 0..N_PERM_SYM {
            cc.set_perm_edge_ud(S2RT.eperm_s2r[i] as usize);
            perm2_comb_p[i] = cc.get_comb_edge(0) as u8
                + (cc.get_nparity(S2RT.eperm_s2r[i] as usize, 8) * 70) as u8;
            cc = cc.inverse_cube();
            perm_inv_edge_sym[i] = cc.get_perm_edge_sym();
        }
        for i in 0..N_MPERM {
            cc.set_perm_m(i);
            cc = cc.inverse_cube();
            mperm_inv[i] = cc.get_perm_m() as u8;
        }

        Self {
            perm2_comb_p,      // DONE! u8 vs i8
            perm_inv_edge_sym, // DONE!
            mperm_inv,         // DONE!
        }
    }
}

/// Coordnate data tables.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CoordTables {
    // x_move = Move Table
    // x_conj = Conjugate Table

    //phase1
    pub udslice_move: Vec<Vec<u16>>,
    pub twist_move: Vec<Vec<u16>>,
    pub flip_move: Vec<Vec<u16>>,
    pub udslice_conj: Vec<Vec<u16>>,

    //phase2
    pub cperm_move: Vec<Vec<u16>>,
    pub eperm_move: Vec<Vec<u16>>,
    pub mperm_move: Vec<Vec<u16>>,
    pub mperm_conj: Vec<Vec<u16>>,
    pub ccombp_move: Vec<Vec<u16>>,
    pub ccombp_conj: Vec<Vec<u16>>,
}

impl CoordTables {
    pub fn new() -> Self {
        let mut udslice_move = vec![vec![0; N_MOVES]; N_SLICE];
        let mut twist_move = vec![vec![0; N_MOVES]; N_TWIST_SYM];
        let mut flip_move = vec![vec![0; N_MOVES]; N_FLIP_SYM];
        let mut udslice_conj = vec![vec![0; 8]; N_SLICE];
        let mut cperm_move = vec![vec![0; N_MOVES2]; N_PERM_SYM];
        let mut eperm_move = vec![vec![0; N_MOVES2]; N_PERM_SYM];
        let mut mperm_move = vec![vec![0; N_MOVES2]; N_MPERM];
        let mut mperm_conj = vec![vec![0; 16]; N_MPERM];
        let mut ccombp_move = vec![vec![0; N_MOVES2]; N_COMB];
        let mut ccombp_conj = vec![vec![0; 16]; N_COMB];

        // init_cperm_move, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_PERM_SYM {
            c.set_perm_corner(S2RT.eperm_s2r[i] as usize);
            for j in 0..N_MOVES2 {
                d.ca = c.corner_multiply(&MT.move_cube[UT.ud2std[j] as usize]).ca;
                cperm_move[i][j] = d.get_perm_corner_sym() as u16;
            }
        }

        // init_eperm_move, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_PERM_SYM {
            c.set_perm_edge_ud(S2RT.eperm_s2r[i] as usize);
            for j in 0..N_MOVES2 {
                d.ea = c.edge_multiply(&MT.move_cube[UT.ud2std[j] as usize]).ea;
                eperm_move[i][j] = d.get_perm_edge_sym() as u16;
            }
        }

        // init_mperm_move_conj, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_MPERM {
            c.set_perm_m(i);
            for j in 0..N_MOVES2 {
                d.ea = c.edge_multiply(&MT.move_cube[UT.ud2std[j] as usize]).ea;
                mperm_move[i][j] = d.get_perm_m() as u16;
            }
            for j in 0..16 {
                d.ea = c.edge_conjugate(MT.sym_mult_inv[0][j] as usize).ea;
                mperm_conj[i][j] = d.get_perm_m() as u16;
            }
        }

        //  init_comb_pmove_conj, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_COMB {
            c.set_comb_corner(i % 70, 0);
            for j in 0..N_MOVES2 {
                d.ca = c.corner_multiply(&MT.move_cube[UT.ud2std[j] as usize]).ca;
                ccombp_move[i][j] =
                    d.get_comb_corner(0) + 70 * ((P2_PARITY_MOVE >> j & 1) ^ (i / 70)) as u16;
            }
            for j in 0..16 {
                d.ca = c.corner_conjugate(MT.sym_mult_inv[0][j] as usize).ca;
                ccombp_conj[i][j] = d.get_comb_corner(0) + 70 * (i as u16 / 70);
            }
        }

        // init_flip_move, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_FLIP_SYM {
            c.set_flip(S2RT.flip_s2r[i]);
            for j in 0..N_MOVES {
                d.ea = c.edge_multiply(&MT.move_cube[j]).ea;
                flip_move[i][j] = d.get_flip_sym();
            }
        }

        // init_twist_move, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_TWIST_SYM {
            c.set_twist(S2RT.twist_s2r[i]);
            for j in 0..N_MOVES {
                d.ca = c.corner_multiply(&MT.move_cube[j]).ca;
                twist_move[i][j] = d.get_twist_sym();
            }
        }

        // init_udslice_move_conj, DONE!
        let mut c = ArrayCube::default();
        let mut d = ArrayCube::default();
        for i in 0..N_SLICE {
            c.set_ud_slice(i as u16);
            for j in 0..(N_MOVES / 3) {
                let j = j * 3;
                d.ea = c.edge_multiply(&MT.move_cube[j]).ea;
                udslice_move[i][j] = d.get_ud_slice();
            }
            for j in 0..8 {
                let j = j * 2;
                d.ea = c.edge_conjugate(MT.sym_mult_inv[0][j] as usize).ea;
                udslice_conj[i][j >> 1] = d.get_ud_slice();
            }
        }
        for i in 0..N_SLICE {
            for j in 0..N_MOVES / 3 {
                let j = j * 3;
                let mut udslice = udslice_move[i][j];
                for k in 1..3 {
                    udslice = udslice_move[udslice as usize][j];
                    udslice_move[i][j + k] = udslice;
                }
            }
        }
        Self {
            udslice_move, // DONE!
            twist_move,   // DONE!
            flip_move,    // DONE!
            udslice_conj, // DONE!
            cperm_move,   // DONE!
            eperm_move,   // DONE!
            mperm_move,   // DONE!
            mperm_conj,   // DONE!
            ccombp_move,  // DONE!
            ccombp_conj,  // DONE!
        }
    }
}

/// Prunning data tables.
#[derive(Debug)]
pub struct PruningTables {
    // x_prun = Pruning Table

    //phase1
    pub udslice_twist_prun: Vec<i32>,
    pub udslice_flip_prun: Vec<i32>,
    pub twist_flip_prun: Vec<i32>,

    //phase2
    pub mcperm_prun: Vec<i32>,
    pub eperm_ccombp_prun: Vec<i32>,
}

impl PruningTables {
    pub fn new() -> Self {
        let mut udslice_twist_prun = vec![0; N_SLICE * N_TWIST_SYM / 8 + 1];
        let mut udslice_flip_prun = vec![0; N_SLICE * N_FLIP_SYM / 8 + 1];
        let mut twist_flip_prun = vec![0; N_FLIP * N_TWIST_SYM / 8 + 1];

        let mut mcperm_prun = vec![0; N_MPERM * N_PERM_SYM / 8 + 1];
        let mut eperm_ccombp_prun = vec![0; N_COMB * N_PERM_SYM / 8 + 1];

        // init_mcperm_prun
        PruningTables::init_mcperm_prun(&mut mcperm_prun, FIRSTTIME_FULL_INIT);

        // init_perm_comb_pprun
        PruningTables::init_perm_comb_pprun(&mut eperm_ccombp_prun, FIRSTTIME_FULL_INIT);

        // init_slice_twist_prun
        PruningTables::init_slice_twist_prun(&mut udslice_twist_prun, FIRSTTIME_FULL_INIT);

        // init_slice_flip_prun
        PruningTables::init_slice_flip_prun(&mut udslice_flip_prun, FIRSTTIME_FULL_INIT);

        // init_twist_flip_prun
        PruningTables::init_twist_flip_prun(&mut twist_flip_prun, FIRSTTIME_FULL_INIT);

        Self {
            udslice_twist_prun, // DONE!
            udslice_flip_prun,  // DONE!
            twist_flip_prun,    // DONE!
            mcperm_prun,        // DONE!
            eperm_ccombp_prun,  // DONE!
        }
    }

    pub fn init_mcperm_prun(mcperm_prun: &mut Vec<i32>, full_init: bool) {
        PruningTables::init_raw_sym_prun(
            mcperm_prun,
            &CT.mperm_move,
            &CT.mperm_conj,
            &CT.cperm_move,
            &S2RT.sym_state_perm,
            0x8ea34,
            full_init,
        );
    }

    pub fn init_perm_comb_pprun(eperm_ccombp_prun: &mut Vec<i32>, full_init: bool) {
        PruningTables::init_raw_sym_prun(
            eperm_ccombp_prun,
            &CT.ccombp_move,
            &CT.ccombp_conj,
            &CT.eperm_move,
            &S2RT.sym_state_perm,
            0x7d824,
            full_init,
        );
    }

    pub fn init_slice_twist_prun(udslice_twist_prun: &mut Vec<i32>, full_init: bool) {
        PruningTables::init_raw_sym_prun(
            udslice_twist_prun,
            &CT.udslice_move,
            &CT.udslice_conj,
            &CT.twist_move,
            &S2RT.sym_state_twist,
            0x69603,
            full_init,
        );
    }

    pub fn init_slice_flip_prun(udslice_flip_prun: &mut Vec<i32>, full_init: bool) {
        PruningTables::init_raw_sym_prun(
            udslice_flip_prun,
            &CT.udslice_move,
            &CT.udslice_conj,
            &CT.flip_move,
            &S2RT.sym_state_flip,
            0x69603,
            full_init,
        );
    }

    pub fn init_twist_flip_prun(twist_flip_prun: &mut Vec<i32>, full_init: bool) {
        PruningTables::init_raw_sym_prun(
            twist_flip_prun,
            &CT.flip_move,
            &vec![],
            &CT.twist_move,
            &S2RT.sym_state_twist,
            0x19603,
            full_init,
        );
    }

    //          |   4 bits  |   4 bits  |   4 bits  |  2 bits | 1b |  1b |   4 bits  |
    //PrunFlag: | MIN_DEPTH | MAX_DEPTH | INV_DEPTH | Padding | P2 | E2C | SYM_SHIFT |
    pub fn init_raw_sym_prun(
        prun_table: &mut Vec<i32>,
        raw_move: &Vec<Vec<u16>>,
        raw_conj: &Vec<Vec<u16>>,
        sym_move: &Vec<Vec<u16>>,
        sym_state: &Vec<u16>,
        prun_flag: i32,
        full_init: bool,
    ) {
        let sym_shift = prun_flag & 0xf;
        let _sym_e2c_magic = match ((prun_flag >> 4) & 1) == 1 {
            true => SYM_E2C_MAGIC,
            false => 0x00000000,
        };
        let is_phase2 = ((prun_flag >> 5) & 1) == 1;
        let inv_depth = prun_flag >> 8 & 0xf;
        let max_depth = prun_flag >> 12 & 0xf;
        let min_depth = prun_flag >> 16 & 0xf;
        let search_depth = match full_init {
            true => max_depth,
            false => min_depth,
        };

        let sym_mask = (1 << sym_shift) - 1;
        let istfp = raw_conj.len() == 0;
        let n_raw = match istfp {
            true => N_FLIP,
            false => raw_move.len(),
        };
        let n_size = n_raw * sym_move.len();
        let _n_moves = match is_phase2 {
            true => 10,
            false => 18,
        };
        let _next_axis_magic = match _n_moves == 10 {
            true => 0x42,
            false => 0x92492,
        };

        let mut depth: i32 = CoordCube::get_pruning(prun_table, n_size) - 1;
        let mut _done = 0;

        if depth == -1 {
            for i in 0..(n_size / 8 + 1) {
                prun_table[i] = 0x11111111;
            }
            CoordCube::set_pruning(prun_table, 0, 0 ^ 1);
            depth = 0;
            _done = 1;
        }

        while depth < search_depth {
            let mask = (depth + 1).wrapping_mul(0x11111111) ^ 0xffffffffu32 as i32;
            for i in 0..prun_table.len() {
                let mut val = prun_table[i] ^ mask;
                val &= val >> 1;
                prun_table[i] = prun_table[i].wrapping_add(val & (val >> 2) & 0x11111111);
            }
            let inv = depth > inv_depth;
            let select = match inv {
                true => depth + 2,
                false => depth,
            };
            let sel_arr_mask = select.wrapping_mul(0x11111111);
            let check = match inv {
                true => depth,
                false => depth + 2,
            };
            depth += 1;
            let xor_val = depth ^ (depth + 1);
            let mut val = 0;
            let mut i = 0;
            while i < n_size {
                if (i & 7) == 0 {
                    val = prun_table[i >> 3];
                    if !CoordCube::has_zero(val ^ sel_arr_mask) {
                        i += 7;
                        i += 1;
                        val >>= 4;
                        continue;
                    }
                }
                if (val & 0xf) != select {
                    i += 1;
                    val >>= 4;
                    continue;
                }
                let raw = i % n_raw;
                let sym = i / n_raw;

                let mut m = 0;
                while m < _n_moves {
                    let mut symx = sym_move[sym][m];
                    let rawx;
                    if istfp {
                        let mut flip = S2RT.flip_r2s[raw];
                        let fsym = flip & 7;
                        flip >>= 3;
                        rawx = S2RT.flip_s2rf[raw_move[flip as usize]
                            [ST.sym8_move[(m << 3 | fsym as usize) as usize] as usize]
                            as usize
                            ^ fsym as usize
                            ^ (symx & sym_mask) as usize];
                    } else {
                        rawx = raw_conj[raw_move[raw as usize][m] as usize]
                            [(symx & sym_mask) as usize];
                    }
                    symx >>= sym_shift;
                    let idx = symx as usize * n_raw + rawx as usize;
                    let prun = CoordCube::get_pruning(prun_table, idx);
                    if prun != check {
                        if prun < depth - 1 {
                            m += _next_axis_magic >> m & 3;
                        }
                        m += 1;
                        continue;
                    }
                    _done += 1;
                    if inv {
                        CoordCube::set_pruning(prun_table, i, xor_val);
                        break;
                    }
                    CoordCube::set_pruning(prun_table, idx, xor_val);
                    let mut j = 1;
                    let mut symstate = sym_state[symx as usize];
                    symstate >>= 1;
                    while symstate != 0 {
                        // println!("init_raw_sym_prun::sym_state:{},j:{}",sym_state, j);
                        if (symstate & 1) != 1 {
                            symstate >>= 1;
                            j += 1;
                            continue;
                        }
                        let mut idxx: usize = symx as usize * n_raw;
                        if istfp {
                            idxx += S2RT.flip_s2rf[(S2RT.flip_r2s[rawx as usize] ^ j) as usize]
                                as usize;
                        } else {
                            idxx += raw_conj[rawx as usize]
                                [(j as usize ^ (_sym_e2c_magic >> (j << 1) & 3)) as usize]
                                as usize;
                        }
                        if CoordCube::get_pruning(prun_table, idxx) == check {
                            CoordCube::set_pruning(prun_table, idxx, xor_val);
                            _done += 1;
                        }
                        symstate >>= 1;
                        j += 1;
                    }
                    m += 1;
                }
                i += 1;
                val >>= 4;
            }
        }
    }
}

#[dynamic(lazy)]
pub static MT: MoveTables = MoveTables::new();

#[dynamic(lazy)]
pub static ST: SymTables = SymTables::new();

#[dynamic(lazy)]
pub static S2RT: Sym2RawTables = Sym2RawTables::new();

#[dynamic(lazy)]
pub static IT: InvTables = InvTables::new();

#[dynamic(lazy)]
pub static CT: CoordTables = CoordTables::new();

#[dynamic(lazy)]
pub static PT: PruningTables = PruningTables::new();

#[cfg(test)]
mod tests {
    use super::{CT, IT, MT, PT, S2RT, ST};

    #[test]
    fn test_mt() {
        // println!("{:?}", MT.cube_sym);
        println!("{:?}", MT.sym_mult);
        println!("{:?}", MT.sym_mult_inv);
    }

    #[test]
    fn test_st() {
        println!("{:?}", ST.move_cube_sym);
    }

    #[test]
    fn test_s2rt() {
        println!("{:?}", S2RT.sym_state_flip);
        println!("{:?}", S2RT.sym_state_twist);
    }

    #[test]
    fn test_it() {
        println!("{:?}", IT.perm2_comb_p);
    }

    #[test]
    fn test_ct() {
        println!("{:?}", CT.udslice_move);
    }

    #[test]
    fn test_pt() {
        println!("{:?}", PT.mcperm_prun);
    }
}