//! # RCuber 
//! `RCuber` - crate for rubiks cube and solvers(CFOP,LBL,Roux,min2phase).

/// Module for Errors.
pub mod error;
/// Module containing 3x3 cube constants.
pub mod constants;
/// Module for represent a cube on the facelet level.
pub mod facelet;
/// Module for represent a cube on the cubie level.
pub mod cubie;
/// Module for represent moves.
pub mod moves;
/// Module for generator.(generate a random cube, a defined state, eg. cross.)
pub mod generator;
/// Module for Solvers.
pub mod solver;
#[cfg(feature = "term")]
/// Module for print a facelet cube on terminal witch color.
pub mod printer;
