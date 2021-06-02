use std::fmt;
use std::str::FromStr;

use Direction::*;
use Position::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Position {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Direction {
    Normal,
    Prime,
    Half,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Move(pub Position, pub Direction);

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(Self(Front, Normal)),
            "F'" => Ok(Self(Front, Prime)),
            "F2" => Ok(Self(Front, Half)),
            "B" => Ok(Self(Back, Normal)),
            "B'" => Ok(Self(Back, Prime)),
            "B2" => Ok(Self(Back, Half)),
            "L" => Ok(Self(Left, Normal)),
            "L'" => Ok(Self(Left, Prime)),
            "L2" => Ok(Self(Left, Half)),
            "R" => Ok(Self(Right, Normal)),
            "R'" => Ok(Self(Right, Prime)),
            "R2" => Ok(Self(Right, Half)),
            "U" => Ok(Self(Up, Normal)),
            "U'" => Ok(Self(Up, Prime)),
            "U2" => Ok(Self(Up, Half)),
            "D" => Ok(Self(Down, Normal)),
            "D'" => Ok(Self(Down, Prime)),
            "D2" => Ok(Self(Down, Half)),
            _ => Err("Unknown move"),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self(Front, Normal) => write!(f, "F"),
            Self(Front, Half) => write!(f, "F2"),
            Self(Front, Prime) => write!(f, "F'"),
            Self(Back, Normal) => write!(f, "B"),
            Self(Back, Half) => write!(f, "B2"),
            Self(Back, Prime) => write!(f, "B'"),
            Self(Left, Normal) => write!(f, "L"),
            Self(Left, Half) => write!(f, "L2"),
            Self(Left, Prime) => write!(f, "L'"),
            Self(Right, Normal) => write!(f, "R"),
            Self(Right, Half) => write!(f, "R2"),
            Self(Right, Prime) => write!(f, "R'"),
            Self(Up, Normal) => write!(f, "U"),
            Self(Up, Half) => write!(f, "U2"),
            Self(Up, Prime) => write!(f, "U'"),
            Self(Down, Normal) => write!(f, "D"),
            Self(Down, Half) => write!(f, "D2"),
            Self(Down, Prime) => write!(f, "D'"),
        }
    }
}

impl Move {
    pub(crate) fn available() -> [Self; 18] {
        [
            Move(Up, Normal),
            Move(Up, Prime),
            Move(Up, Half),
            Move(Down, Normal),
            Move(Down, Prime),
            Move(Down, Half),
            Move(Left, Normal),
            Move(Left, Prime),
            Move(Left, Half),
            Move(Right, Normal),
            Move(Right, Prime),
            Move(Right, Half),
            Move(Front, Normal),
            Move(Front, Prime),
            Move(Front, Half),
            Move(Back, Normal),
            Move(Back, Prime),
            Move(Back, Half),
        ]
    }

    pub(crate) fn inverse(self) -> Self {
        match self {
            Move(position, Normal) => Move(position, Prime),
            Move(position, Prime) => Move(position, Normal),
            _ => self,
        }
    }
}
