use std::collections::HashMap;

use crate::cubie::{Corner, CubieCube, Edge, SOLVED_CUBIE_CUBE};
use crate::facelet::Color;
use crate::{facelet::FaceCube, moves::Move};

use super::square::SOLVED_SQUARE_CUBE;

pub fn generate_solution(
    cube: &CubieCube,
    moves: &Vec<Move>,
    current: &Vec<Move>,
    left: usize,
    prunetable: &HashMap<String, usize>,
) -> Vec<Move> {
    if fb_is_solved(cube) {
        return current.clone();
    }
    // println!("Left: {left}; current: {:?}", current);
    let mut current = current.clone();
    let least_moves = prunetable.get(&FaceCube::try_from(cube).unwrap().to_string());
    let least_moves = match least_moves.is_some() {
        true => *least_moves.unwrap(),
        false => 6,
    };
    if least_moves > left {
        return Vec::new();
    }
    let mut cube = *cube;
    for m in moves.iter() {
        if current.len() > 0 && current.last().unwrap().get_face() == m.get_face() {
            continue;
        }
        current.push(*m);
        cube = cube.apply_move(*m);
        let next = generate_solution(&cube, moves, &current, left - 1, prunetable);
        if next.len() > 0 {
            return next;
        }
        cube = cube.apply_move(m.get_inverse());
        current.pop();
    }
    Vec::new()
}

pub fn fb_is_solved(cube: &CubieCube) -> bool {
    if cube.center[4] != SOLVED_CUBIE_CUBE.center[4] {
        return false;
    }
    for i in 5..=6 {
        if cube.cp[i] != SOLVED_CUBIE_CUBE.cp[i] || cube.co[i] != 0 {
            return false;
        }
    }
    for i in [6, 9, 10] {
        if cube.ep[i] != SOLVED_CUBIE_CUBE.ep[i] || cube.eo[i] != 0 {
            return false;
        }
    }
    true
}

pub fn generate_pruning_table(
    states: Vec<FaceCube>,
    depth: usize,
    moves: &Vec<Move>,
) -> HashMap<String, usize> {
    let mut table = HashMap::new();
    let mut prev = states;
    for i in 1..=depth {
        let mut next = Vec::new();
        for state in prev.iter() {
            for m in moves.iter() {
                let new_state = CubieCube::try_from(state).unwrap();
                let new_state = new_state.apply_move(*m);
                let new_state = FaceCube::try_from(&new_state).unwrap();
                let new_state_str = new_state.to_string();
                if !table.contains_key(&new_state_str) {
                    table.insert(new_state_str, i);
                    next.push(new_state);
                }
            }
        }
        prev = next;
    }
    table
}
pub fn generate_masked_state(reqs: &Vec<usize>) -> [[[Color; 3]; 3]; 6] {
    let mut masked = SOLVED_SQUARE_CUBE.s;
    for i in 0..54 {
        let (face, row, col) = to_state_position(i);
        if reqs.contains(&i) {
            continue;
        }
        masked[face][row][col] = Color::U;
    }
    masked
}

pub fn to_state_position(position: usize) -> (usize, usize, usize) {
    let face = position / 9;
    let row = (position % 9) / 3;
    let col = position % 3;
    (face, row, col)
}

pub fn to_position(face: usize, row: usize, col: usize) -> usize {
    face * 9 + row * 3 + col
}

/// This is a searching function of A*
pub fn a_star_search<'a, S, V, G>(
    start: (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>),
    successors: S,
    state_value: V,
    is_goal: G,
) -> Vec<Move>
where
    S: Fn(
        (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>),
        Option<Move>,
    ) -> Vec<(Move, (Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>))>,
    V: Fn((Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>)) -> u32,
    G: Fn((Vec<(Corner, u8, u8)>, Vec<(Edge, u8, u8)>)) -> bool,
{
    if is_goal(start.clone()) {
        return Vec::new();
    }
    let mut explored = Vec::new();
    let h = state_value(start.clone());
    let g = 1;
    let f = g + h;
    let p = vec![(None, start)];
    let mut frontier = Vec::new();
    frontier.push((f, g, h, p));
    while frontier.len() > 0 {
        let (_f, g, _h, path) = frontier.remove(0);
        let s = path.last().unwrap();
        let la = s.0;
        for (action, state) in successors(s.1.clone(), la.clone()) {
            if !explored.contains(&state) {
                explored.push(state.clone());
                let mut path2 = path.clone();
                path2.push((Some(action), state.clone()));
                if is_goal(state.clone()) {
                    let mut r = Vec::new();
                    for (a, _s) in path2 {
                        if a.is_some() {
                            r.push(a.expect("Move Error!"));
                        }
                    }
                    return r;
                } else {
                    let h2 = state_value(state.clone());
                    let g2 = g + 1;
                    let f2 = h2 + g2;
                    frontier.push((f2, g2, h2, path2));
                    frontier.sort_by_key(|k| (k.0, k.1, k.2));
                }
            }
        }
    }
    Vec::new()
}
