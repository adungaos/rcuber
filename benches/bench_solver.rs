use criterion::{criterion_group, criterion_main, Criterion};
use rcuber::{cubie::CubieCube, moves::Formula, solver::{CFOPSolver, LBLSolver, Min2PhaseSolver}};

fn lbl() {
    let cc = CubieCube::default();
    let moves = Formula::scramble();
    // println!("Scramble: {:?}", moves);
    let cc = cc.apply_formula(&moves);
    let mut solver = LBLSolver { cube: cc };
    let _s = solver.solve();
    assert!(solver.is_solved());
}

fn cfop() {
    let cc = CubieCube::default();
    let moves = Formula::scramble();
    // println!("Scramble: {:?}", moves);
    let cc = cc.apply_formula(&moves);
    let mut solver = CFOPSolver { cube: cc };
    let _s = solver.solve();
    assert!(solver.is_solved());
}
fn m2p() {
    let cc = CubieCube::default();
    let moves = Formula::scramble();
    // println!("Scramble: {:?}", moves);
    let cc = cc.apply_formula(&moves);
    let mut solver = Min2PhaseSolver { cube: cc };
    let _s = solver.solve();
    assert!(solver.is_solved());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("LBL Solver", |b| b.iter(|| lbl()));
    c.bench_function("CFOP Solver", |b| b.iter(|| cfop()));
    c.bench_function("Min2Phase Solver", |b| b.iter(|| m2p()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
