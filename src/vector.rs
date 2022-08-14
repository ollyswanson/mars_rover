//! 2D vector in Cartesian coordinates.
use std::ops;

/// Used to represent the position and orientation of a [`Rover`] in a 2D grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    pub fn rotate_left(&self) -> Vector {
        // The rotation matrix for a left (90 degrees anticlockwise) rotation is:
        // [0, -1
        //  1,  0]
        //  The result of a left rotation for a vector [x, y] is therefore [-y, x]
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_right(&self) -> Vector {
        // The rotation matrix for a right (90 degrees clockwise) rotation is:
        // [ 0, 1
        //  -1, 0]
        //  The result of a right rotation for a vector [x, y] is therefore [y, -x]
        Vector {
            x: self.y,
            y: -self.x,
        }
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left() {
        let north = Vector::new(0, 1);
        let east = Vector::new(1, 0);
        let south = Vector::new(0, -1);
        let west = Vector::new(-1, 0);

        assert_eq!(west, north.rotate_left());
        assert_eq!(south, west.rotate_left());
        assert_eq!(east, south.rotate_left());
        assert_eq!(north, east.rotate_left());
    }

    #[test]
    fn rotate_right() {
        let north = Vector::new(0, 1);
        let east = Vector::new(1, 0);
        let south = Vector::new(0, -1);
        let west = Vector::new(-1, 0);

        assert_eq!(east, north.rotate_right());
        assert_eq!(south, east.rotate_right());
        assert_eq!(west, south.rotate_right());
        assert_eq!(north, west.rotate_right());
    }
}
