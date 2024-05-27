use ccuber::cubie::CubieCube;
use ccuber::facelet::FaceCube;
use ccuber::printer::print_facelet;
use ccuber::moves::Move::*;


fn main()
{
    let cc = CubieCube::default();
    let mvs = vec![R, U, R3, U3, M, S, E];
    let cc = cc.apply_moves(&mvs);
    // println!("{:#?}", cc);
    let fc = FaceCube::try_from(&cc).unwrap();
    println!("{}", fc);
    let _ = print_facelet(&fc);
}