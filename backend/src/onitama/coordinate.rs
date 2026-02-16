use std::{fmt::Display, ops::Add};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coordinate {
    pub y: u8,
    pub x: u8,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

#[derive(Error, Debug, PartialEq, Eq)]

pub enum IntoCoordinateError {
    #[error("The given point ({0}, {1}) is out of bounds")]
    UnsignedOutOfBounds(u8, u8),
    #[error("The given point ({0}, {1}) is out of bounds")]
    SignedOutOFBounds(i8, i8),
}

impl TryFrom<(u8, u8)> for Coordinate {
    type Error = IntoCoordinateError;

    fn try_from((y, x): (u8, u8)) -> Result<Self, Self::Error> {
        if y >= 5 || x >= 5 {
            return Err(IntoCoordinateError::UnsignedOutOfBounds(y, x));
        }
        Ok(Self { y, x })
    }
}

impl TryFrom<(i8, i8)> for Coordinate {
    type Error = IntoCoordinateError;

    fn try_from((y, x): (i8, i8)) -> Result<Self, Self::Error> {
        if !(0..5).contains(&y) || !(0..5).contains(&x) {
            return Err(IntoCoordinateError::SignedOutOFBounds(y, x));
        }
        Ok(Self {
            y: y as u8,
            x: x as u8,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Offset {
    pub y: i8,
    pub x: i8,
}

impl From<(i8, i8)> for Offset {
    fn from((y, x): (i8, i8)) -> Self {
        Self { y, x }
    }
}

impl Add<Offset> for Offset {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Coordinate {
    pub fn try_add(&self, offset: &Offset) -> Option<Self> {
        let (ny, nx) = (self.y as i8 + offset.y, self.x as i8 + offset.x);
        if !(0..5).contains(&ny) || !(0..5).contains(&nx) {
            return None;
        }
        Some(Self {
            y: ny as u8,
            x: nx as u8,
        })
    }
}

#[cfg(test)]

pub mod tests {
    use super::*;
    #[test]
    pub fn try_into_coordinate_success_u8() {
        let tuple: (u8, u8) = (3, 0);
        let coord: Result<Coordinate, _> = tuple.try_into();
        assert_eq!(coord, Ok(Coordinate { y: 3, x: 0 }))
    }
    #[test]
    pub fn try_into_coordinate_fail_u8() {
        let tuple: (u8, u8) = (5, 0);
        let coord: Result<Coordinate, _> = tuple.try_into();
        assert_eq!(coord, Err(IntoCoordinateError::UnsignedOutOfBounds(5, 0)))
    }

    #[test]
    pub fn try_into_coordinate_success_i8() {
        let tuple: (i8, i8) = (0, 4);
        let coord: Result<Coordinate, _> = tuple.try_into();
        assert_eq!(coord, Ok(Coordinate { y: 0, x: 4 }))
    }
    #[test]
    pub fn try_into_coordinate_fail_i8() {
        let tuple: (i8, i8) = (4, -1);
        let coord: Result<Coordinate, _> = tuple.try_into();
        assert_eq!(coord, Err(IntoCoordinateError::SignedOutOFBounds(4, -1)))
    }

    #[test]
    pub fn add_offsets() {
        let a: Offset = (-10, 5).into();
        let b: Offset = (5, -2).into();
        assert_eq!(a + b, Offset { y: -5, x: 3 })
    }

    #[test]
    pub fn add_coord_offset_succeed() {
        let a: Coordinate = (0u8, 2u8).try_into().expect("(0, 2) is a valid coordinate");
        let offset: Offset = (1, 2).into();

        assert_eq!(a.try_add(&offset), Some(Coordinate { y: 1, x: 4 }))
    }

    #[test]
    pub fn add_coord_offset_fail() {
        let a: Coordinate = (0u8, 2u8).try_into().expect("(0, 2) is a valid coordinate");
        let offset: Offset = (-1, 2).into();

        assert_eq!(a.try_add(&offset), None)
    }
}
