use rcuber::cubie::CubieCube;
use rcuber::facelet::FaceCube;
use rcuber::generator::Generator;
use rcuber::moves::Formula;
#[allow(unused_imports)]
use rcuber::moves::Move::*;
#[cfg(feature = "term")]
use rcuber::printer::print_facelet;
use rcuber::solver::cfop::CFOPSolver;
use rcuber::solver::lbl::LBLSolver;
use rcuber::solver::roux::fb::FBSolver;
use rcuber::solver::Min2PhaseSolver;

fn main() {
    let cc = CubieCube::default();
    let moves = Formula::scramble();
    println!("Scramble: {:?}", moves);
    let cc = cc.apply_formula(&moves);
    let cc2 = cc.clone();
    let cc3 = cc.clone();
    let fc = FaceCube::try_from(&cc).unwrap();
    let _r = print_facelet(&fc);
    let mut lbl = LBLSolver { cube: cc };
    let lbls = lbl.solve();
    assert!(lbl.is_solved());
    let fc = FaceCube::try_from(&lbl.cube).unwrap();
    let _r = print_facelet(&fc);
    println!("LBL Solution: {:?}, Len: {}", lbls, lbls.len());
    let mut cfop = CFOPSolver { cube: cc2 };
    let cfops = cfop.solve();
    println!("CFOP Solution: {:?}, Len: {}", cfops, cfops.len());
    let mut m2p = Min2PhaseSolver { cube: cc3 };
    let m2ps = m2p.solve();
    println!("Min2Phase Solution: {}, Len: {}", m2ps, m2ps.moves.len());
    let rc = Generator::roux_fb_solved();
    let fc = FaceCube::try_from(&rc).unwrap();
    let _r = print_facelet(&fc);

    let cc = CubieCube::default();
    let f = Formula::scramble();
    println!("Scramble: {:?}", f);
    let cc = cc.apply_formula(&f);
    let mut fb = FBSolver { cube: cc };
    let solution = fb.solve();
    println!("First Block Solution: {:?}", solution);
    assert!(fb.is_solved());
    println!("First Block Solution: {:?}", solution);
}
