use rand::random;

use crate::{
    cubie::{Corner, CubieCube, Edge},
    facelet::Color,
};

/// CubieCube generator.
pub struct Generator {}

impl Generator {
    const RANDOM: i8 = -1;
    const SOLVED: i8 = -2;

    fn fill_perm(arr: &[i8]) -> Vec<u8> {
        let cnt = arr.len();
        let mut res = vec![0; cnt];
        for i in 0..cnt {
            if arr[i] != Generator::SOLVED && arr[i] != Generator::RANDOM {
                res[i] = arr[i] as u8;
            }
            if arr[i] == Generator::SOLVED {
                res[i] = i as u8;
            }
        }
        while Generator::verify_perm(&res) != true {
            for i in 0..cnt {
                if arr[i] == Generator::RANDOM {
                    res[i] = (random::<usize>() % cnt) as u8;
                }
            }
        }
        res
    }

    fn verify_perm(arr: &Vec<u8>) -> bool {
        let cnt = arr.len();
        let mut arr = arr.clone();
        arr.sort();
        for i in 0..cnt {
            if arr[i] != i as u8 {
                return false;
            }
        }
        true
    }

    fn fill_ori(arr: &[i8]) -> Vec<u8> {
        let cnt = arr.len();
        let mut res = vec![0; cnt];
        let ori = match cnt {
            8 => 3,
            _ => 2,
        };
        loop {
            for i in 0..cnt {
                match arr[i] {
                    Generator::RANDOM => res[i] = random::<u8>() % ori,
                    _ => res[i] = arr[i] as u8,
                };
            }
            if res.iter().fold(0, |acc, x| acc + x) % ori as u8 == 0 {
                break;
            }
        }
        res
    }

    fn fix_parity(cc: &mut CubieCube, cp: &[i8; 8], ep: &[i8; 12]) {
        if cp == &[Generator::SOLVED; 8] {
            Generator::swap_edge(&mut cc.ep, ep);
            return;
        }
        if ep == &[Generator::SOLVED; 12] {
            Generator::swap_corner(&mut cc.cp, cp);
            return;
        }
        match random::<u8>() % 2 == 0 {
            true => Generator::swap_edge(&mut cc.ep, ep),
            false => Generator::swap_corner(&mut cc.cp, cp),
        }
    }

    fn swap_corner(cp: &mut [Corner; 8], cp_o: &[i8; 8]) {
        let idxx = Generator::get_unfill(cp_o);
        let m = idxx.len();
        let p1 = random::<usize>() % m;
        let mut p2 = random::<usize>() % m;
        while p1 == p2 {
            p2 = random::<usize>() % m;
        }
        cp.swap(idxx[p1], idxx[p2]);
    }

    fn swap_edge(ep: &mut [Edge; 12], ep_o: &[i8; 12]) {
        let idxx = Generator::get_unfill(ep_o);
        let m = idxx.len();
        let p1 = random::<usize>() % m;
        let mut p2 = random::<usize>() % m;
        while p1 == p2 {
            p2 = random::<usize>() % m;
        }
        ep.swap(idxx[p1], idxx[p2]);
    }

    fn get_unfill(arr: &[i8]) -> Vec<usize> {
        let mut r = Vec::new();
        for i in 0..arr.len() {
            if arr[i] == Generator::RANDOM {
                r.push(i);
            }
        }
        r
    }

    fn gen_edge(ep: [i8; 12], eo: [i8; 12]) -> ([Edge; 12], [u8; 12]) {
        // let ep = [-1; 12];
        let epv = Generator::fill_perm(&ep);
        let mut ep = [Edge::UR; 12];
        for i in 0..12 {
            ep[i] = Edge::try_from(epv[i]).unwrap();
        }
        // let eo = [-1; 12];
        let eo = Generator::fill_ori(&eo);
        let eo: [u8; 12] = eo.try_into().unwrap();
        (ep, eo)
    }

    fn gen_corner(cp: [i8; 8], co: [i8; 8]) -> ([Corner; 8], [u8; 8]) {
        // let cp = [-1; 8];
        let cpv = Generator::fill_perm(&cp);
        let mut cp = [Corner::URF; 8];
        for i in 0..8 {
            cp[i] = Corner::try_from(cpv[i]).unwrap();
        }
        // let co = [-1; 8];
        let co = Generator::fill_ori(&co);
        let co: [u8; 8] = co.try_into().unwrap();
        (cp, co)
    }

    /// Generate a CubieCube based on defined cp, co, ep, eo.
    ///
    /// `cp, ep`: -1 = Generator::RANDOM, -2 = Generator::SOLVED, other valid number（0-7 for cp, 0-11 for ep) for expected Corner/Edge.
    ///
    /// `co, eo`: -1 = Generator::RANDOM, other valid number (0-2 for co, 0-1 for eo) for expected Co/Eo.
    ///
    pub fn gen_state(cpv: [i8; 8], cov: [i8; 8], epv: [i8; 12], eov: [i8; 12]) -> CubieCube {
        let (cp, co) = Generator::gen_corner(cpv, cov);
        let (ep, eo) = Generator::gen_edge(epv, eov);
        let center = [Color::U, Color::R, Color::F, Color::D, Color::L, Color::B];

        let mut cc = CubieCube {
            center,
            cp,
            co,
            ep,
            eo,
        };
        if cc.verify().is_err() {
            Generator::fix_parity(&mut cc, &cpv, &epv);
        }
        cc
    }

    /// Generate a random CubieCube using cc.randomize().
    pub fn random() -> CubieCube {
        let mut cc = CubieCube::default();
        cc.randomize();
        cc
    }

    /// Generate a random CubieCube use general gen_state method.
    pub fn random2() -> CubieCube {
        let cp = [-1; 8];
        let co = [-1; 8];
        let ep = [-1; 12];
        let eo = [-1; 12];
        Generator::gen_state(cp, co, ep, eo)
    }

    /// Generate a cross solved state CubieCube.
    pub fn corss_solved() -> CubieCube {
        let cp = [Generator::RANDOM; 8];
        let co = [Generator::RANDOM; 8];
        let ep = [-1, -1, -1, -1, 4, 5, 6, 7, -1, -1, -1, -1];
        let eo = [-1, -1, -1, -1, 0, 0, 0, 0, -1, -1, -1, -1];
        Generator::gen_state(cp, co, ep, eo)
    }

    /// Generate a CFOP F2L Solved(OLL) state CubieCube.
    pub fn f2l_solved() -> CubieCube {
        let cp = [-1, -1, -1, -1, 4, 5, 6, 7];
        let co = [-1, -1, -1, -1, 0, 0, 0, 0];
        let ep = [-1, -1, -1, -1, 4, 5, 6, 7, 8, 9, 10, 11];
        let eo = [-1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0];
        Generator::gen_state(cp, co, ep, eo)
    }

    /// Generate a CFOP OLL solved(PLL) state CubieCube.
    pub fn pll() -> CubieCube {
        let cp = [-1, -1, -1, -1, 4, 5, 6, 7];
        let co = [0; 8];
        let ep = [-1, -1, -1, -1, 4, 5, 6, 7, 8, 9, 10, 11];
        let eo = [0; 12];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn lastslot() -> CubieCube {
        let cp = [-1, -1, -1, -1, -1, 5, 6, 7];
        let co = [-1, -1, -1, -1, -1, 0, 0, 0];
        let ep = [-1, -1, -1, -1, 4, 5, 6, 7, -1, 9, 10, 11];
        let eo = [-1, -1, -1, -1, 0, 0, 0, 0, -1, 0, 0, 0];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn edge_ll() -> CubieCube {
        let cp = [Generator::SOLVED; 8];
        let co = [0; 8];
        let ep = [-1, -1, -1, -1, 4, 5, 6, 7, 8, 9, 10, 11];
        let eo = [-1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn corner_ll() -> CubieCube {
        let cp = [-1, -1, -1, -1, 4, 5, 6, 7];
        let co = [-1, -1, -1, -1, 0, 0, 0, 0];
        let ep = [Generator::SOLVED; 12];
        let eo = [0; 12];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn superflip() -> CubieCube {
        let cp = [Generator::SOLVED; 8];
        let co = [0; 8];
        let ep = [Generator::SOLVED; 12];
        let eo = [1; 12];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn roux_fb_solved() -> CubieCube {
        let cp = [-1, -1, -1, -1, -1, 5, 6, -1];
        let co = [-1, -1, -1, -1, -1, 0, 0, -1];
        let ep = [-1, -1, -1, -1, -1, -1, 6, -1, -1, 9, 10, -1];
        let eo = [-1, -1, -1, -1, -1, -1, 0, -1, -1, 0, 0, -1];
        Generator::gen_state(cp, co, ep, eo)
    }

    pub fn roux_fb_sb_solved() -> CubieCube {
        let cp = [-1, -1, -1, -1, 4, 5, 6, 7];
        let co = [-1, -1, -1, -1, 0, 0, 0, 0];
        let ep = [-1, -1, -1, -1, 4, -1, 6, -1, 8, 9, 10, 11];
        let eo = [-1, -1, -1, -1, 0, -1, 0, -1, 0, 0, 0, 0];
        Generator::gen_state(cp, co, ep, eo)
    }
}

#[cfg(test)]
mod tests {
    use crate::{facelet::FaceCube, printer::print_facelet};

    use super::Generator;

    #[test]
    fn test_generator() {
        let cc = Generator::roux_fb_sb_solved();
        let fc = FaceCube::try_from(&cc).unwrap();
        println!("{}", fc.to_string());
        let _ = print_facelet(&fc);
    }
}
