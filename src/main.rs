use rcuber::facelet::FaceCube;
#[allow(unused_imports)]
use rcuber::moves::Move::*;
#[cfg(feature = "term")]
use rcuber::printer::print_facelet;
use rcuber::scramble;
use rcuber::solver::cfop::CFOPSolver;
use rcuber::cubie::CubieCube;
use rcuber::solver::lbl::LBLSolver;

fn main() {
    let cc = CubieCube::default();
    let moves = scramble();
    println!("Scramble: {:?}", moves);
    let cc = cc.apply_moves(&moves);
    let cc2 = cc.clone();
    let fc = FaceCube::try_from(&cc).unwrap();
    let _r = print_facelet(&fc);
    let mut lbl = LBLSolver{cube: cc};
    let lbls = lbl.solve();
    assert!(lbl.is_solved());
    let fc = FaceCube::try_from(&lbl.cube).unwrap();
    let _r = print_facelet(&fc);
    println!("LBL Solution: {:?}, Len: {}", lbls, lbls.len());
    let mut cfop = CFOPSolver{cube: cc2};
    let cfops = cfop.solve();
    println!("CFOP Solution: {:?}, Len: {}", cfops, cfops.len());
}
