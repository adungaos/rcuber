use ccuber::cubie::CubieCube;
use ccuber::facelet::FaceCube;
use ccuber::printer::print_facelet;
use ccuber::moves::Move::*;
use ccuber::solver::cfop::cross::CrossSolver;


fn main()
{
    let cc = CubieCube::default();
    let d_edges = cc.get_edges_d();
    let solved = CrossSolver::cross_goal((cc.center, d_edges));
    assert!(solved);
    // let moves = vec![
    //     L2, B3, U, L2, F2, U, L2, D, R, B2, L3, F, U2, L2, B3, D3, F2, U3, L3, D, R2, U3, L
    // ];
    let moves = vec![L2, R, F2, L, D, U, R];
    let cc = cc.apply_moves(&moves);
    let d_edges = cc.get_edges_d();
    // let solved = CrossSolver::cross_goal((cc.center, d_edges.clone()));
    // assert!(!solved);
    // let csv = CrossSolver::cross_state_value((cc.center, d_edges.clone()));
    // assert_eq!(csv, 7);
    // for edge in d_edges.clone() {
    //     let _edge = Edge::try_from(edge.1).unwrap();
    //     println!("{:?}, {:?}, {}, {}", edge.0, _edge, edge.1, edge.2);
    // }
    // let ne = CrossSolver::_rotate(d_edges.clone(), R);
    // for edge in ne {
    //     let _edge = Edge::try_from(edge.1).unwrap();
    //     println!("{:?}, {:?}, {}, {}", edge.0, _edge, edge.1, edge.2);
    // }
    let mut cs = CrossSolver{cube: cc};
    let result = cs.solve();
    println!("{:?}", result);
}