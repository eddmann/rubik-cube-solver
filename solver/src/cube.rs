use rand::prelude::SliceRandom;
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum Facelet {
    White,
    Orange,
    Green,
    Red,
    Yellow,
    Blue,
}

impl fmt::Display for Facelet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::White => write!(f, "W"),
            Self::Orange => write!(f, "O"),
            Self::Green => write!(f, "G"),
            Self::Red => write!(f, "R"),
            Self::Yellow => write!(f, "Y"),
            Self::Blue => write!(f, "B"),
        }
    }
}

impl FromStr for Facelet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(Self::White),
            "O" => Ok(Self::Orange),
            "G" => Ok(Self::Green),
            "R" => Ok(Self::Red),
            "Y" => Ok(Self::Yellow),
            "B" => Ok(Self::Blue),
            _ => Err("Unknown facelet colour"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Move {
    Forward,
    ForwardPrime,
    ForwardHalf,
    Up,
    UpPrime,
    UpHalf,
    Right,
    RightPrime,
    RightHalf,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Forward => write!(f, "F"),
            Self::ForwardPrime => write!(f, "F'"),
            Self::ForwardHalf => write!(f, "F2"),
            Self::Up => write!(f, "U"),
            Self::UpPrime => write!(f, "U'"),
            Self::UpHalf => write!(f, "U2"),
            Self::Right => write!(f, "R"),
            Self::RightPrime => write!(f, "R'"),
            Self::RightHalf => write!(f, "R2"),
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(Self::Forward),
            "F'" => Ok(Self::ForwardPrime),
            "F2" => Ok(Self::ForwardHalf),
            "U" => Ok(Self::Up),
            "U'" => Ok(Self::UpPrime),
            "U2" => Ok(Self::UpHalf),
            "R" => Ok(Self::Right),
            "R'" => Ok(Self::RightPrime),
            "R2" => Ok(Self::RightHalf),
            _ => Err("Unknown move"),
        }
    }
}

impl Move {
    fn available() -> [Self; 9] {
        [
            Self::Forward,
            Self::ForwardPrime,
            Self::ForwardHalf,
            Self::Up,
            Self::UpPrime,
            Self::UpHalf,
            Self::Right,
            Self::RightPrime,
            Self::RightHalf,
        ]
    }

    pub fn inverse(self) -> Self {
        match self {
            Self::Forward => Self::ForwardPrime,
            Self::ForwardPrime => Self::Forward,
            Self::Up => Self::UpPrime,
            Self::UpPrime => Self::Up,
            Self::Right => Self::RightPrime,
            Self::RightPrime => Self::Right,
            _ => self,
        }
    }
}

type CubeState = [Facelet; 24];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Cube {
    state: CubeState,
}

impl fmt::Display for Cube {
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

impl FromStr for Cube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let facelets: Vec<Facelet> = s
            .chars()
            .map(|facelet| facelet.to_string().parse())
            .collect::<Result<Vec<_>, _>>()?;

        let state: CubeState = match facelets.try_into() {
            Ok(cube_state) => Ok(cube_state),
            Err(_) => Err("Invalid facelet colour sequence"),
        }?;

        if (state[7], state[19], state[22]) != (Facelet::Orange, Facelet::Yellow, Facelet::Blue) {
            return Err("Orange/Yellow/Blue cubie must be positioned at DLB");
        }

        Ok(Cube { state })
    }
}

impl Cube {
    pub fn random(total_moves: u8) -> Self {
        let mut rng = rand::thread_rng();
        (0..total_moves).fold(Self::solved(), |cube, _| {
            cube.apply_move(&Move::available().choose(&mut rng).unwrap())
        })
    }

    #[rustfmt::skip]
    pub fn solved() -> Self {
        Self {
            state: [
                Facelet::White, Facelet::White, Facelet::White, Facelet::White,
                Facelet::Orange, Facelet::Orange, Facelet::Orange, Facelet::Orange,
                Facelet::Green, Facelet::Green, Facelet::Green, Facelet::Green,
                Facelet::Red, Facelet::Red, Facelet::Red, Facelet::Red,
                Facelet::Yellow, Facelet::Yellow, Facelet::Yellow, Facelet::Yellow,
                Facelet::Blue, Facelet::Blue, Facelet::Blue, Facelet::Blue,
            ],
        }
    }

    fn rotate(self, rotations: [usize; 24]) -> CubeState {
        let mut rotated = self.state;
        for idx in 0..rotated.len() {
            rotated[idx] = self.state[rotations[idx]];
        }
        rotated
    }

    #[rustfmt::skip]
    pub fn apply_move(self, action: &Move) -> Self {
        match action {
            Move::Forward => Self {
                state: self.rotate([
                    0, 1, 5, 6, 4, 16, 17, 7, 11, 8, 9, 10, 3,
                    13, 14, 2, 15, 12, 18, 19, 20, 21, 22, 23,
                ]),
            },
            Move::ForwardPrime => Self {
                state: self.rotate([
                    0, 1, 15, 12, 4, 2, 3, 7, 9, 10, 11, 8, 17,
                    13, 14, 16, 5, 6, 18, 19, 20, 21, 22, 23,
                ]),
            },
            Move::ForwardHalf => self.apply_moves(vec![Move::Forward, Move::Forward]),
            Move::Up => Self {
                state: self.rotate([
                    3, 0, 1, 2, 8, 9, 6, 7, 12, 13, 10, 11, 20,
                    21, 14, 15, 16, 17, 18, 19, 4, 5, 22, 23,
                ]),
            },
            Move::UpPrime => Self {
                state: self.rotate([
                    1, 2, 3, 0, 20, 21, 6, 7, 4, 5, 10, 11, 8,
                    9, 14, 15, 16, 17, 18, 19, 12, 13, 22, 23,
                ]),
            },
            Move::UpHalf => self.apply_moves(vec![Move::Up, Move::Up]),
            Move::Right => Self {
                state: self.rotate([
                    0, 9, 10, 3, 4, 5, 6, 7, 8, 17, 18, 11, 15,
                    12, 13, 14, 16, 23, 20, 19, 2, 21, 22, 1,
                ]),
            },
            Move::RightPrime => Self {
                state: self.rotate([
                    0, 23, 20, 3, 4, 5, 6, 7, 8, 1, 2, 11, 13,
                    14, 15, 12, 16, 9, 10, 19, 18, 21, 22, 17,
                ]),
            },
            Move::RightHalf => self.apply_moves(vec![Move::Right, Move::Right]),
        }
    }

    pub fn apply_moves(self, actions: Vec<Move>) -> Self {
        actions
            .iter()
            .fold(self, |cube, action| cube.apply_move(action))
    }

    pub fn next_states(self) -> [(Move, Self); 9] {
        return [
            (Move::Forward, self.apply_move(&Move::Forward)),
            (Move::ForwardPrime, self.apply_move(&Move::ForwardPrime)),
            (Move::ForwardHalf, self.apply_move(&Move::ForwardHalf)),
            (Move::Up, self.apply_move(&Move::Up)),
            (Move::UpPrime, self.apply_move(&Move::UpPrime)),
            (Move::UpHalf, self.apply_move(&Move::UpHalf)),
            (Move::Right, self.apply_move(&Move::Right)),
            (Move::RightPrime, self.apply_move(&Move::RightPrime)),
            (Move::RightHalf, self.apply_move(&Move::RightHalf)),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! cube_move_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, Cube::solved().apply_move(&input).to_string());
            }
        )*
        }
    }

    cube_move_tests! {
        forward: (Move::Forward, "WWOOOYYOGGGGWRRWRRYYBBBB"),
        forward_prime: (Move::ForwardPrime, "WWRROWWOGGGGYRRYOOYYBBBB"),
        forward_half: (Move::ForwardHalf, "WWYYORROGGGGORROWWYYBBBB"),
        up: (Move::Up, "WWWWGGOORRGGBBRRYYYYOOBB"),
        up_prime: (Move::UpPrime, "WWWWBBOOOOGGGGRRYYYYRRBB"),
        up_half: (Move::UpHalf, "WWWWRROOBBGGOORRYYYYGGBB"),
        right: (Move::Right, "WGGWOOOOGYYGRRRRYBBYWBBW"),
        right_prime: (Move::RightPrime, "WBBWOOOOGWWGRRRRYGGYYBBY"),
        right_half: (Move::RightHalf, "WYYWOOOOGBBGRRRRYWWYGBBG"),
    }

    #[test]
    fn applying_half_move_twice_returns_to_initial_state() {
        assert_eq!(
            Cube::solved(),
            Cube::solved().apply_moves(vec![Move::ForwardHalf, Move::ForwardHalf])
        );
    }

    #[test]
    fn applying_double_quarter_turn_is_the_same_as_single_half_turn() {
        assert_eq!(
            Cube::solved().apply_move(&Move::ForwardHalf),
            Cube::solved().apply_moves(vec![Move::Forward, Move::Forward])
        );

        assert_eq!(
            Cube::solved().apply_move(&Move::ForwardHalf),
            Cube::solved().apply_moves(vec![Move::ForwardPrime, Move::ForwardPrime])
        );
    }
}
