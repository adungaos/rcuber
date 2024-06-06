use ccuber::cubie::CubieCube;
use ccuber::facelet::FaceCube;
use ccuber::printer::print_facelet;
use ccuber::moves::Move::*;
use ccuber::solver::cfop::f2l::F2LPairSolver;
use ccuber::facelet::Color;


fn main() {
    let cc = CubieCube::default();
    let moves = vec![L2, R, F2, L, D, U, R];
    let cc = cc.apply_moves(&moves);
    let solution = vec![U3, D, F2, D2, R3, F2, D2];
    let cc = cc.apply_moves(&solution);
    // let fc = FaceCube::try_from(&cc).unwrap();
    // let _r = print_facelet(&fc).unwrap();

    for p in [
        // [Color::F, Color::R],
        // [Color::B, Color::R],
        [Color::B, Color::L],
        // [Color::F, Color::L],
    ] {
        let mut solver = F2LPairSolver {
            cube: cc,
            pair: p,
        };
        let slot = solver.get_slot();
        println!("Slot: {:?}", slot);
        let combining = solver.combining_setup();
        println!("Combining result: {:?}", combining);
        let r = solver.solve();
        println!("Solve result: {:?}", r);
    }
    
    // let fc = FaceCube::try_from(&cc).unwrap();
    // let _r = print_facelet(&fc).unwrap();
    // let fc = FaceCube::try_from(&cc).unwrap();
    // let mut f2l = F2LSolver { cube: cc };
    // let _s = f2l.solve();
    // // println!("{:?}", s);
    // let _r = print_facelet(&fc);
    // let pp = solver.get_pair();
    // println!("{:?}", pp);
    // let pp = F2LPairSolver::_rotate(pp, F);
    // println!("{:?}", pp);
    // let states_new = F2LPairSolver::combining_successors(pp, vec![U]);
    // println!("{:?}", states_new);
    // // assert_eq!(p, ((Corner::DFR,0), (Edge::FR,0)));
    // let s = solver.get_slot();
    // let (_, _, (corner, edge)) = s;
    // println!("{:?}", corner_to_pos(corner));
    // println!("{:?}", s);
    
}