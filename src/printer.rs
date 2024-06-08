use std::io::stdout;
use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveUp},
    execute,
    style::{Color as TermColor, SetBackgroundColor},
};
use crate::facelet::{Color, FaceCube};

fn color_to_termcolor(color: Color) -> TermColor {
    match color {
        Color::U => TermColor::DarkYellow,
        Color::R => TermColor::Magenta,
        Color::F => TermColor::Green,
        Color::D => TermColor::White,
        Color::L => TermColor::Red,
        Color::B => TermColor::Blue,
    }
}

fn print_face(face: &[Color], offset: u16) -> Result<(), std::io::Error> {
    for i in 0..3 {
        let layer = format!(
            "{}  {}  {}  {}",
            SetBackgroundColor(color_to_termcolor(face[3 * i])),
            SetBackgroundColor(color_to_termcolor(face[(3 * i) + 1])),
            SetBackgroundColor(color_to_termcolor(face[(3 * i) + 2])),
            SetBackgroundColor(TermColor::Reset)
        );

        println!("{layer}");

        if offset != 0 {
            execute!(stdout(), MoveRight(offset))?;
        }
    }

    Ok(())
}

pub fn print_facelet(facelet: &FaceCube) -> Result<(), std::io::Error> {
    let stdout = stdout();

    println!();
    execute!(&stdout, MoveRight(6))?;
    print_face(&facelet.f[0..9], 6)?; // U (white)
    execute!(&stdout, MoveLeft(6))?;
    print_face(&facelet.f[36..45], 0)?; // L (orange)
    execute!(&stdout, MoveRight(6), MoveUp(3))?;
    print_face(&facelet.f[18..27], 6)?; // F (green)
    execute!(&stdout, MoveLeft(12), MoveUp(3), MoveRight(12))?;
    print_face(&facelet.f[9..18], 12)?; // R (red)
    execute!(&stdout, MoveLeft(12), MoveUp(3), MoveRight(18))?;
    print_face(&facelet.f[45..54], 18)?; // B (blue)
    execute!(&stdout, MoveLeft(12))?;
    print_face(&facelet.f[27..36], 6)?; // D (yellow)
    execute!(&stdout, MoveLeft(12))?;
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cubie::CubieCube;
    use crate::facelet::FaceCube;
    use crate::moves::Move::*;
    use super::*;

    #[test]
    fn test_printer () {
        let cc = CubieCube::default();
        let mvs = vec![R, U, R3, U3, M, S, E];
        let cc = cc.apply_moves(&mvs);
        println!("{:#?}", &cc);
        let fc = FaceCube::try_from(&cc).unwrap();
        let _ = print_facelet(&fc);
    }
}