use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;
use std::str::FromStr;

use crate::cube::Cube;
use crate::moves::{Direction, Move, Position};
use Corner::*;
use Edge::*;

/// The Rubik cube on the cubie level; described by 8 corner cubies, 12 edge cubies and the cubie orientations.

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Corner {
    URF = 0,
    UFL = 1,
    ULB = 2,
    UBR = 3,
    DFR = 4,
    DLF = 5,
    DBL = 6,
    DRB = 7,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Edge {
    UR = 0,
    UF = 1,
    UL = 2,
    UB = 3,
    DR = 4,
    DF = 5,
    DL = 6,
    DB = 7,
    FR = 8,
    FL = 9,
    BL = 10,
    BR = 11,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct CubieCube {
    pub(crate) cp: [Corner; 8],
    pub(crate) co: [u8; 8],
    pub(crate) ep: [Edge; 12],
    pub(crate) eo: [u8; 12],
}

impl Default for CubieCube {
    fn default() -> Self {
        CubieCube {
            cp: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
            co: [0; 8],
            ep: [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
            eo: [0; 12],
        }
    }
}

impl Cube for CubieCube {
    fn apply_move(self, action: &Move) -> Self {
        match *action {
            Move(position, Direction::Normal) => self.multiply(match position {
                Position::Up => *MOVE_U,
                Position::Down => *MOVE_D,
                Position::Left => *MOVE_L,
                Position::Right => *MOVE_R,
                Position::Front => *MOVE_F,
                Position::Back => *MOVE_B,
            }),
            Move(position, Direction::Half) => self.apply_moves(&vec![
                Move(position, Direction::Normal),
                Move(position, Direction::Normal),
            ]),
            Move(position, Direction::Prime) => self.apply_moves(&vec![
                Move(position, Direction::Normal),
                Move(position, Direction::Normal),
                Move(position, Direction::Normal),
            ]),
        }
    }
}

impl CubieCube {
    fn multiply(&self, other: Self) -> Self {
        let mut new_ep = self.ep;
        let mut new_eo = self.eo;

        for idx in 0..self.ep.len() {
            new_ep[idx] = self.ep[other.ep[idx] as usize];
            new_eo[idx] = (other.eo[idx] + self.eo[other.ep[idx] as usize]) % 2;
        }

        let mut new_cp = self.cp;
        let mut new_co = self.co;
        let mut ori: i8 = 0;

        for idx in 0..new_cp.len() {
            new_cp[idx] = self.cp[other.cp[idx] as usize];
            let ori_self = self.co[other.cp[idx] as usize];
            let ori_other = other.co[idx];

            if ori_self < 3 && ori_other < 3 {
                // two regular cubes
                ori = (ori_self + ori_other) as i8;
                if ori >= 3 {
                    ori -= 3;
                }
            } else if ori_self < 3 && ori_other >= 3 {
                // cube b is in a mirrored state
                ori = (ori_self + ori_other) as i8;
                if ori >= 6 {
                    ori -= 3;
                }
            } else if ori_self >= 3 && ori_other < 3 {
                // cube a is in a mirrored state
                ori = (ori_self - ori_other) as i8;
                if ori < 3 {
                    ori += 3;
                }
            } else if ori_self >= 3 && ori_other >= 3 {
                // if both cubes are in mirrored states
                ori = (ori_self - ori_other) as i8;
                if ori < 0 {
                    ori += 3;
                }
            }

            new_co[idx] = ori as u8;
        }

        CubieCube {
            cp: new_cp,
            co: new_co,
            ep: new_ep,
            eo: new_eo,
        }
    }
}

impl fmt::Display for CubieCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl FromStr for CubieCube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(cube) => Ok(cube),
            Err(_) => Err("Invalid cubie cube representation"),
        }
    }
}

lazy_static! {
    static ref MOVE_U: CubieCube = CubieCube {
        cp: [UBR, URF, UFL, ULB, DFR, DLF, DBL, DRB,],
        co: [0; 8],
        ep: [UB, UR, UF, UL, DR, DF, DL, DB, FR, FL, BL, BR,],
        eo: [0; 12],
    };
    static ref MOVE_R: CubieCube = CubieCube {
        cp: [DFR, UFL, ULB, URF, DRB, DLF, DBL, UBR,],
        co: [2, 0, 0, 1, 1, 0, 0, 2],
        ep: [FR, UF, UL, UB, BR, DF, DL, DB, DR, FL, BL, UR,],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    static ref MOVE_F: CubieCube = CubieCube {
        cp: [UFL, DLF, ULB, UBR, URF, DFR, DBL, DRB,],
        co: [1, 2, 0, 0, 2, 1, 0, 0],
        ep: [UR, FL, UL, UB, DR, FR, DL, DB, UF, DF, BL, BR,],
        eo: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
    };
    static ref MOVE_D: CubieCube = CubieCube {
        cp: [URF, UFL, ULB, UBR, DLF, DBL, DRB, DFR,],
        co: [0; 8],
        ep: [UR, UF, UL, UB, DF, DL, DB, DR, FR, FL, BL, BR,],
        eo: [0; 12],
    };
    static ref MOVE_L: CubieCube = CubieCube {
        cp: [URF, ULB, DBL, UBR, DFR, UFL, DLF, DRB,],
        co: [0, 1, 2, 0, 0, 2, 1, 0],
        ep: [UR, UF, BL, UB, DR, DF, FL, DB, FR, UL, DL, BR,],
        eo: [0; 12],
    };
    static ref MOVE_B: CubieCube = CubieCube {
        cp: [URF, UFL, UBR, DRB, DFR, DLF, ULB, DBL,],
        co: [0, 0, 1, 2, 0, 0, 2, 1],
        ep: [UR, UF, UL, BR, DR, DF, DL, BL, FR, FL, UB, DB,],
        eo: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::{Direction, Position};

    macro_rules! cubie_cube_move_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, CubieCube::default().apply_move(&input).to_string());
            }
        )*
        }
    }

    cubie_cube_move_tests! {
        up: (Move(Position::Up, Direction::Normal), "{\"cp\":[3,0,1,2,4,5,6,7],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[3,0,1,2,4,5,6,7,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        up_prime: (Move(Position::Up, Direction::Prime), "{\"cp\":[1,2,3,0,4,5,6,7],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[1,2,3,0,4,5,6,7,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        up_half: (Move(Position::Up, Direction::Half), "{\"cp\":[2,3,0,1,4,5,6,7],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[2,3,0,1,4,5,6,7,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        down: (Move(Position::Down, Direction::Normal), "{\"cp\":[0,1,2,3,5,6,7,4],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,1,2,3,5,6,7,4,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        down_prime: (Move(Position::Down, Direction::Prime), "{\"cp\":[0,1,2,3,7,4,5,6],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,1,2,3,7,4,5,6,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        down_half: (Move(Position::Down, Direction::Half), "{\"cp\":[0,1,2,3,6,7,4,5],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,1,2,3,6,7,4,5,8,9,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        left: (Move(Position::Left, Direction::Normal), "{\"cp\":[0,2,6,3,4,1,5,7],\"co\":[0,1,2,0,0,2,1,0],\"ep\":[0,1,10,3,4,5,9,7,8,2,6,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        left_prime: (Move(Position::Left, Direction::Prime), "{\"cp\":[0,5,1,3,4,6,2,7],\"co\":[0,1,2,0,0,2,1,0],\"ep\":[0,1,9,3,4,5,10,7,8,6,2,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        left_half: (Move(Position::Left, Direction::Half), "{\"cp\":[0,6,5,3,4,2,1,7],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,1,6,3,4,5,2,7,8,10,9,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        right: (Move(Position::Right, Direction::Normal), "{\"cp\":[4,1,2,0,7,5,6,3],\"co\":[2,0,0,1,1,0,0,2],\"ep\":[8,1,2,3,11,5,6,7,4,9,10,0],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        right_prime: (Move(Position::Right, Direction::Prime), "{\"cp\":[3,1,2,7,0,5,6,4],\"co\":[2,0,0,1,1,0,0,2],\"ep\":[11,1,2,3,8,5,6,7,0,9,10,4],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        right_half: (Move(Position::Right, Direction::Half), "{\"cp\":[7,1,2,4,3,5,6,0],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[4,1,2,3,0,5,6,7,11,9,10,8],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        front: (Move(Position::Front, Direction::Normal), "{\"cp\":[1,5,2,3,0,4,6,7],\"co\":[1,2,0,0,2,1,0,0],\"ep\":[0,9,2,3,4,8,6,7,1,5,10,11],\"eo\":[0,1,0,0,0,1,0,0,1,1,0,0]}"),
        front_prime: (Move(Position::Front, Direction::Prime), "{\"cp\":[4,0,2,3,5,1,6,7],\"co\":[1,2,0,0,2,1,0,0],\"ep\":[0,8,2,3,4,9,6,7,5,1,10,11],\"eo\":[0,1,0,0,0,1,0,0,1,1,0,0]}"),
        front_half: (Move(Position::Front, Direction::Half), "{\"cp\":[5,4,2,3,1,0,6,7],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,5,2,3,4,1,6,7,9,8,10,11],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
        back: (Move(Position::Back, Direction::Normal), "{\"cp\":[0,1,3,7,4,5,2,6],\"co\":[0,0,1,2,0,0,2,1],\"ep\":[0,1,2,11,4,5,6,10,8,9,3,7],\"eo\":[0,0,0,1,0,0,0,1,0,0,1,1]}"),
        back_prime: (Move(Position::Back, Direction::Prime), "{\"cp\":[0,1,6,2,4,5,7,3],\"co\":[0,0,1,2,0,0,2,1],\"ep\":[0,1,2,10,4,5,6,11,8,9,7,3],\"eo\":[0,0,0,1,0,0,0,1,0,0,1,1]}"),
        back_half: (Move(Position::Back, Direction::Half), "{\"cp\":[0,1,7,6,4,5,3,2],\"co\":[0,0,0,0,0,0,0,0],\"ep\":[0,1,2,7,4,5,6,3,8,9,11,10],\"eo\":[0,0,0,0,0,0,0,0,0,0,0,0]}"),
    }

    #[test]
    fn applying_half_move_twice_returns_to_initial_state() {
        assert_eq!(
            CubieCube::default(),
            CubieCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Half),
                Move(Position::Front, Direction::Half)
            ])
        );
    }

    #[test]
    fn applying_double_quarter_turn_is_the_same_as_single_half_turn() {
        assert_eq!(
            CubieCube::default().apply_move(&Move(Position::Front, Direction::Half)),
            CubieCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Normal),
                Move(Position::Front, Direction::Normal)
            ])
        );

        assert_eq!(
            CubieCube::default().apply_move(&Move(Position::Front, Direction::Half)),
            CubieCube::default().apply_moves(&vec![
                Move(Position::Front, Direction::Prime),
                Move(Position::Front, Direction::Prime)
            ])
        );
    }
}
