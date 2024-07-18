use std::time::Instant;

use rcuber::cubie::CubieCube;
use rcuber::facelet::FaceCube;
use rcuber::moves::Formula;
#[allow(unused_imports)]
use rcuber::moves::Move::*;
#[cfg(feature = "term")]
use rcuber::printer::print_facelet;
use rcuber::solver::cfop::CFOPSolver;
use rcuber::solver::lbl::LBLSolver;
use rcuber::solver::roux::RouxSolver;
use rcuber::solver::Min2PhaseSolver;

fn main() {
    let cc = CubieCube::default();
    let moves = Formula::scramble();
    println!("Scramble: {:?}", moves);
    let cc = cc.apply_formula(&moves);
    let cc2 = cc.clone();
    let cc3 = cc.clone();
    let cc4 = cc.clone();
    let fc = FaceCube::try_from(&cc).unwrap();
    let _r = print_facelet(&fc);
    let start = Instant::now();
    let mut lbl = LBLSolver { cube: cc };
    let lbls = lbl.solve();
    assert!(lbl.is_solved());
    let lble = start.elapsed();
    println!(
        "LBL Solution: {:?}, Len: {}, Time: {:?}",
        lbls,
        lbls.len(),
        lble
    );
    let start = Instant::now();
    let mut cfop = CFOPSolver { cube: cc2 };
    let cfops = cfop.solve();
    let cfope = start.elapsed();
    println!(
        "CFOP Solution: {:?}, Len: {}, Time: {:?}",
        cfops,
        cfops.len(),
        cfope
    );
    let start = Instant::now();
    let mut roux = RouxSolver { cube: cc3 };
    let rouxs = roux.solve();
    let rouxe = start.elapsed();
    println!(
        "Roux Solution: {:?}, Len: {}, Time: {:?}",
        rouxs,
        rouxs.len(),
        rouxe
    );
    let start = Instant::now();
    let mut m2p = Min2PhaseSolver { cube: cc4 };
    let m2ps = m2p.solve();
    let m2pe = start.elapsed();
    println!(
        "Min2Phase Solution: {}, Len: {}, Time: {:?}",
        m2ps,
        m2ps.moves.len(),
        m2pe
    );
    // let fc = FaceCube::try_from(&lbl.cube).unwrap();
    // let _r = print_facelet(&fc);
}
