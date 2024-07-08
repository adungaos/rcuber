pub const SYM_E2C_MAGIC: usize = 0x00DDDD00;
pub const N_PERM_4: usize = 24;
pub const N_MPERM: usize = 24;
pub const N_COMB: usize = 140;
pub const P2_PARITY_MOVE: usize = 0xA5;
/// number of possible face moves
pub const N_MOVES: usize = 18;
pub const N_MOVES2: usize = 10;

/// 3^7 possible corner orientations in phase 1
pub const N_TWIST: usize = 2187;
pub const N_TWIST_SYM: usize = 324;

/// 2^11 possible edge orientations in phase 1
pub const N_FLIP: usize = 2048;
pub const N_FLIP_SYM: usize = 336;
/// 12*11*10*9 possible positions of the FR, FL, BL, BR edges in phase 1
pub const N_SLICE_SORTED: usize = 11880;
/// we ignore the permutation of FR, FL, BL, BR in phase 1
pub const N_SLICE: usize = N_SLICE_SORTED / N_PERM_4;

pub const N_PERM: usize = 40320;
pub const N_PERM_SYM: usize = 2768;

pub const USE_TWIST_FLIP_PRUN: bool = true;
// Options for research purpose.
pub const MAX_PRE_MOVES: usize = 20;
pub const TRY_INVERSE: bool = true;
pub const TRY_THREE_AXES: bool = true;

pub const USE_COMBP_PRUN: bool = USE_TWIST_FLIP_PRUN;
pub const USE_CONJ_PRUN: bool = USE_TWIST_FLIP_PRUN;
pub const MIN_P1LENGTH_PRE: usize = 7;
pub const MAX_DEPTH2: usize = 12;

/**
 *     Verbose_Mask determines if a " . " separates the phase1 and phase2 parts of the solver string like in F' R B R L2 F .
 *     U2 U D for example.<br>
 */
pub const USE_SEPARATOR: usize = 0x1;

/**
 *     Verbose_Mask determines if the solution will be inversed to a scramble/state generator.
 */
pub const INVERSE_SOLUTION: usize = 0x2;

/**
 *     Verbose_Mask determines if a tag such as "(21f)" will be appended to the solution.
 */
pub const APPEND_LENGTH: usize = 0x4;

/**
 *     Verbose_Mask determines if guaranteeing the solution to be optimal.
 */
pub const OPTIMAL_SOLUTION: usize = 0x8;

pub const FIRSTTIME_FULL_INIT: bool = false;