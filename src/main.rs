use rcuber::facelet::FaceCube;
// use rcuber::moves::Move::*;
#[cfg(feature = "term")]
use rcuber::printer::print_facelet;
use rcuber::scramble;
use rcuber::moves::optimise_moves;
// use rcuber::solver::cfop::f2l::F2LSolver;
// use rcuber::solver::cfop::oll::OLLSolver;
// use rcuber::solver::cfop::pll::PLLSolver;
// use rcuber::solver::cfop::CFOPSolver;
use rcuber::solver::lbl::coll::COLLSolver;
use rcuber::solver::lbl::cpll::CPLLSolver;
use rcuber::solver::lbl::eoll::EOLLSolver;
use rcuber::solver::lbl::EPLLSolver;
use rcuber::{cubie::CubieCube, solver::lbl::MiddleEdgeSolver};

use rcuber::solver::lbl::{bottom::BottomCornerSolver, CrossSolver};

fn main() {
    let cc = CubieCube::default();
    let moves = scramble();
    println!("Scramble: {:?}", moves);
    let cc = cc.apply_moves(&moves);
    let fc = FaceCube::try_from(&cc).unwrap();
    let _r = print_facelet(&fc);
    let mut cross = CrossSolver::new(cc, true);
    let _cs = cross.solve();
    let mut bottom = BottomCornerSolver { cube: cross.cube };
    let _bs = bottom.solve();
    let _bs = optimise_moves(&_bs);
    let mut middle = MiddleEdgeSolver { cube: bottom.cube };
    let _ms = middle.solve();
    let _ms = optimise_moves(&_ms);
    let mut eoll = EOLLSolver { cube: middle.cube };
    let _eos = eoll.solve();
    assert!(eoll.is_solved());
    let mut coll = COLLSolver { cube: eoll.cube };
    let _cos = coll.solve();
    assert!(coll.is_solved());
    let mut cpll = CPLLSolver { cube: coll.cube };
    let _cps = cpll.solve();
    assert!(cpll.is_solved());
    let mut epll = EPLLSolver { cube: cpll.cube };
    let _eps = epll.solve();
    assert!(epll.is_solved());
    println!("Solution: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        _cs, _bs, _ms, _eos, _cos, _cps, _eps
    );
    let fc = FaceCube::try_from(&epll.cube).unwrap();
    let _r = print_facelet(&fc);
    // for i in 0..10000 {
    //     let start = Instant::now();
    //     let cc = CubieCube::default();
    //     let moves = scramble();
    //     let cc = cc.apply_moves(&moves);
    //     #[cfg(feature = "term")]
    //     let fc = FaceCube::try_from(&cc).unwrap();
    //     #[cfg(feature = "term")]
    //     let _ = print_facelet(&fc);
    //     let mut solver = CFOPSolver{cube: cc};
    //     let solution = solver.solve();
    //     #[cfg(feature = "term")]
    //     let fc = FaceCube::try_from(&solver.cube).unwrap();
    //     #[cfg(feature = "term")]
    //     let _ = print_facelet(&fc);
    //     let duration = start.elapsed();
    //     println!("{:?} {:?} {:?}", moves, solution, duration);
    //     assert!(solver.is_solved());
    // }
    // for _ in 0..10000 {
    //     let cc = CubieCube::default();
    //     let moves = scramble();
    //     let cc = cc.apply_moves(&moves);
    //     let mut cross = CrossSolver { cube: cc };
    //     let _c = cross.solve();
    //     if !cross.is_solved() {
    //         panic!("Cross Error! {:?} : {:?}", moves, _c);
    //     }
    //     let cc = cross.cube.clone();
    //     let fc = FaceCube::try_from(&cc).unwrap();
    //     println!("Cross");
    //     let _r = print_facelet(&fc);
    //     let mut f2l = F2LSolver { cube: cc };
    //     let _f = f2l.solve();
    //     let cc = f2l.cube.clone();
    //     if !f2l.is_solved() {
    //         panic!("F2L Error! {:?} : {:?}: {:?}", moves, _c, _f);
    //     }
    //     let fc = FaceCube::try_from(&cc).unwrap();
    //     println!("F2L");
    //     let _r = print_facelet(&fc);
    //     let mut oll = OLLSolver::new(cc);
    //     let _o = oll.solve();
    //     if !oll.is_solved() {
    //         panic!("OLL Error! {:?} : {:?} : {:?} : {:?}", moves, _c, _f, _o);
    //     }
    //     let fc = FaceCube::try_from(&oll.cube).unwrap();
    //     println!("OLL");
    //     let _r = print_facelet(&fc);
    //     let cc = oll.cube.clone();
    //     let mut pll = PLLSolver::new(cc);
    //     let _p = pll.solve();
    //     if !pll.is_solved() {
    //         panic!(
    //             "PLL Error! {:?} : {:?} : {:?} : {:?} : {:?}",
    //             moves, _c, _f, _o, _p
    //         );
    //     }
    //     let fc = FaceCube::try_from(&pll.cube).unwrap();
    //     println!("PLL");
    //     let _r = print_facelet(&fc);
    // }
}
