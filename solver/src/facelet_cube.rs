use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use crate::cube::Cube;
use crate::cubie_cube::CubieCube;
use crate::moves::Move;
use Colour::*;
use Facelet::*;

/// The Rubik cube on the facelet level; described by positions of the coloured stickers.

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Colour {
    U = 0,
    R = 1,
    F = 2,
    D = 3,
    L = 4,
    B = 5,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
enum Facelet {
    U1 = 0,
    U2 = 1,
    U3 = 2,
    U4 = 3,
    U5 = 4,
    U6 = 5,
    U7 = 6,
    U8 = 7,
    U9 = 8,
    R1 = 9,
    R2 = 10,
    R3 = 11,
    R4 = 12,
    R5 = 13,
    R6 = 14,
    R7 = 15,
    R8 = 16,
    R9 = 17,
    F1 = 18,
    F2 = 19,
    F3 = 20,
    F4 = 21,
    F5 = 22,
    F6 = 23,
    F7 = 24,
    F8 = 25,
    F9 = 26,
    D1 = 27,
    D2 = 28,
    D3 = 29,
    D4 = 30,
    D5 = 31,
    D6 = 32,
    D7 = 33,
    D8 = 34,
    D9 = 35,
    L1 = 36,
    L2 = 37,
    L3 = 38,
    L4 = 39,
    L5 = 40,
    L6 = 41,
    L7 = 42,
    L8 = 43,
    L9 = 44,
    B1 = 45,
    B2 = 46,
    B3 = 47,
    B4 = 48,
    B5 = 49,
    B6 = 50,
    B7 = 51,
    B8 = 52,
    B9 = 53,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::U => write!(f, "W"),
            Self::R => write!(f, "R"),
            Self::F => write!(f, "G"),
            Self::D => write!(f, "Y"),
            Self::L => write!(f, "O"),
            Self::B => write!(f, "B"),
        }
    }
}

impl FromStr for Colour {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(Self::U),
            "R" => Ok(Self::R),
            "G" => Ok(Self::F),
            "Y" => Ok(Self::D),
            "O" => Ok(Self::L),
            "B" => Ok(Self::B),
            _ => Err("Unknown facelet colour"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct FaceletCube {
    state: [Colour; 54],
}

impl Cube for FaceletCube {
    fn apply_move(self, action: &Move) -> Self {
        FaceletCube::from(CubieCube::from(self).apply_move(action))
    }
}

impl Default for FaceletCube {
    #[rustfmt::skip]
    fn default() -> Self {
        FaceletCube {
            state: [
                U, U, U, U, U, U, U, U, U,
                R, R, R, R, R, R, R, R, R,
                F, F, F, F, F, F, F, F, F,
                D, D, D, D, D, D, D, D, D,
                L, L, L, L, L, L, L, L, L,
                B, B, B, B, B, B, B, B, B,
            ],
        }
    }
}

impl fmt::Display for FaceletCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.state
                .iter()
                .map(|face| format!("{}", face))
                .collect::<String>()
        )
    }
}

impl FromStr for FaceletCube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let facelets: Vec<Colour> = s
            .chars()
            .map(|facelet| facelet.to_string().parse())
            .collect::<Result<Vec<_>, _>>()?;

        match facelets.try_into() {
            Ok(state) => Ok(FaceletCube { state }),
            Err(_) => Err("Invalid facelet cube representation"),
        }
    }
}

static CORNER_FACELETS: [[Facelet; 3]; 8] = [
    [U9, R1, F3],
    [U7, F1, L3],
    [U1, L1, B3],
    [U3, B1, R3],
    [D3, F9, R7],
    [D1, L9, F7],
    [D7, B9, L7],
    [D9, R9, B7],
];

static EDGE_FACELETS: [[Facelet; 2]; 12] = [
    [U6, R2],
    [U8, F2],
    [U4, L2],
    [U2, B2],
    [D6, R8],
    [D2, F8],
    [D4, L8],
    [D8, B8],
    [F6, R4],
    [F4, L6],
    [B6, L4],
    [B4, R6],
];

static CORNER_COLOURS: [[Colour; 3]; 8] = [
    [U, R, F],
    [U, F, L],
    [U, L, B],
    [U, B, R],
    [D, F, R],
    [D, L, F],
    [D, B, L],
    [D, R, B],
];

static EDGE_COLOURS: [[Colour; 2]; 12] = [
    [U, R],
    [U, F],
    [U, L],
    [U, B],
    [D, R],
    [D, F],
    [D, L],
    [D, B],
    [F, R],
    [F, L],
    [B, L],
    [B, R],
];

impl From<CubieCube> for FaceletCube {
    fn from(cube: CubieCube) -> Self {
        let mut faces = FaceletCube::default();

        for i in 0..cube.co.len() {
            for k in 0..3 {
                faces.state[CORNER_FACELETS[i][((k + cube.co[i]) % 3) as usize] as usize] =
                    CORNER_COLOURS[cube.cp[i] as usize][k as usize];
            }
        }

        for i in 0..cube.eo.len() {
            for k in 0..2 {
                faces.state[EDGE_FACELETS[i][((k + cube.eo[i]) % 2) as usize] as usize] =
                    EDGE_COLOURS[cube.ep[i] as usize][k as usize];
            }
        }

        faces
    }
}

impl From<FaceletCube> for CubieCube {
    fn from(faces: FaceletCube) -> Self {
        let mut cube = CubieCube::default();

        for i in 0..cube.cp.len() {
            let fac = CORNER_FACELETS[i];

            let mut ori = 0;
            for _ in 0..3 {
                if faces.state[fac[ori] as usize] == Colour::U
                    || faces.state[fac[ori] as usize] == Colour::D
                {
                    break;
                }
                ori += 1;
            }

            let col1 = faces.state[fac[(ori + 1) % 3] as usize];
            let col2 = faces.state[fac[(ori + 2) % 3] as usize];

            for j in 0..cube.cp.len() {
                let col = CORNER_COLOURS[j];
                if col1 == col[1] && col2 == col[2] {
                    cube.cp[i] = unsafe { std::mem::transmute(j as u8) };
                    cube.co[i] = ori as u8;
                    break;
                }
            }
        }

        for i in 0..cube.ep.len() {
            for j in 0..cube.ep.len() {
                if faces.state[EDGE_FACELETS[i][0] as usize] == EDGE_COLOURS[j][0]
                    && faces.state[EDGE_FACELETS[i][1] as usize] == EDGE_COLOURS[j][1]
                {
                    cube.ep[i] = unsafe { std::mem::transmute(j as u8) };
                    cube.eo[i] = 0;
                    break;
                }

                if faces.state[EDGE_FACELETS[i][0] as usize] == EDGE_COLOURS[j][1]
                    && faces.state[EDGE_FACELETS[i][1] as usize] == EDGE_COLOURS[j][0]
                {
                    cube.ep[i] = unsafe { std::mem::transmute(j as u8) };
                    cube.eo[i] = 1;
                    break;
                }
            }
        }

        cube
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::{Direction, Position};

    macro_rules! facelet_cube_move_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, FaceletCube::default().apply_move(&input).to_string());
            }
        )*
        }
    }

    facelet_cube_move_tests! {
        up: (Move(Position::Up, Direction::Normal), "WWWWWWWWWBBBRRRRRRRRRGGGGGGYYYYYYYYYGGGOOOOOOOOOBBBBBB"),
        up_prime: (Move(Position::Up, Direction::Prime), "WWWWWWWWWGGGRRRRRROOOGGGGGGYYYYYYYYYBBBOOOOOORRRBBBBBB"),
        up_half: (Move(Position::Up, Direction::Half), "WWWWWWWWWOOORRRRRRBBBGGGGGGYYYYYYYYYRRROOOOOOGGGBBBBBB"),
        down: (Move(Position::Down, Direction::Normal), "WWWWWWWWWRRRRRRGGGGGGGGGOOOYYYYYYYYYOOOOOOBBBBBBBBBRRR"),
        down_prime: (Move(Position::Down, Direction::Prime), "WWWWWWWWWRRRRRRBBBGGGGGGRRRYYYYYYYYYOOOOOOGGGBBBBBBOOO"),
        down_half: (Move(Position::Down, Direction::Half), "WWWWWWWWWRRRRRROOOGGGGGGBBBYYYYYYYYYOOOOOORRRBBBBBBGGG"),
        left: (Move(Position::Left, Direction::Normal), "BWWBWWBWWRRRRRRRRRWGGWGGWGGGYYGYYGYYOOOOOOOOOBBYBBYBBY"),
        left_prime: (Move(Position::Left, Direction::Prime), "GWWGWWGWWRRRRRRRRRYGGYGGYGGBYYBYYBYYOOOOOOOOOBBWBBWBBW"),
        left_half: (Move(Position::Left, Direction::Half), "YWWYWWYWWRRRRRRRRRBGGBGGBGGWYYWYYWYYOOOOOOOOOBBGBBGBBG"),
        right: (Move(Position::Right, Direction::Normal), "WWGWWGWWGRRRRRRRRRGGYGGYGGYYYBYYBYYBOOOOOOOOOWBBWBBWBB"),
        right_prime: (Move(Position::Right, Direction::Prime), "WWBWWBWWBRRRRRRRRRGGWGGWGGWYYGYYGYYGOOOOOOOOOYBBYBBYBB"),
        right_half: (Move(Position::Right, Direction::Half), "WWYWWYWWYRRRRRRRRRGGBGGBGGBYYWYYWYYWOOOOOOOOOGBBGBBGBB"),
        front: (Move(Position::Front, Direction::Normal), "WWWWWWOOOWRRWRRWRRGGGGGGGGGRRRYYYYYYOOYOOYOOYBBBBBBBBB"),
        front_prime: (Move(Position::Front, Direction::Prime), "WWWWWWRRRYRRYRRYRRGGGGGGGGGOOOYYYYYYOOWOOWOOWBBBBBBBBB"),
        front_half: (Move(Position::Front, Direction::Half), "WWWWWWYYYORRORRORRGGGGGGGGGWWWYYYYYYOOROOROORBBBBBBBBB"),
        back: (Move(Position::Back, Direction::Normal), "RRRWWWWWWRRYRRYRRYGGGGGGGGGYYYYYYOOOWOOWOOWOOBBBBBBBBB"),
        back_prime: (Move(Position::Back, Direction::Prime), "OOOWWWWWWRRWRRWRRWGGGGGGGGGYYYYYYRRRYOOYOOYOOBBBBBBBBB"),
        back_half: (Move(Position::Back, Direction::Half), "YYYWWWWWWRRORRORROGGGGGGGGGYYYYYYWWWROOROOROOBBBBBBBBB"),
    }

    #[test]
    fn applying_half_move_twice_returns_to_initial_state() {
        assert_eq!(
            FaceletCube::default(),
            FaceletCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Half),
                Move(Position::Front, Direction::Half)
            ])
        );
    }

    #[test]
    fn applying_double_quarter_turn_is_the_same_as_single_half_turn() {
        assert_eq!(
            FaceletCube::default().apply_move(&Move(Position::Front, Direction::Half)),
            FaceletCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Normal),
                Move(Position::Front, Direction::Normal)
            ])
        );

        assert_eq!(
            FaceletCube::default().apply_move(&Move(Position::Front, Direction::Half)),
            FaceletCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Prime),
                Move(Position::Front, Direction::Prime)
            ])
        );
    }
}
