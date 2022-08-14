use crate::vector::Vector;

#[derive(Debug)]
pub struct Rover {
    /// The position of the rover on a [`Grid`].
    position: Vector,

    /// The orientation of the rover, represented by a unit vector, for a rover facing east the
    /// orientation is represented by `Vector { x: 1, y: 0 }`
    orientation: Vector,

    /// A rover is `Operational` while its `position` remains in its area of operation (represented
    /// by [`Grid`]), if at any time the rover leaves its area of operation it is marked as `Lost`
    status: RoverStatus,
}

#[derive(Debug, PartialEq, Eq)]
enum RoverStatus {
    Operational,
    Lost,
}

/// A command issued to a [`Rover`].
#[derive(Debug, Clone, Copy)]
pub enum Command {
    /// Command the rover to turn left (90 degrees anticlockwise).
    Left,
    /// Command the rover to turn right (90 degrees clockwise).
    Right,
    /// Command the rover to move forward by 1. For example, if the rover is currently facing east
    /// at the position `Vector { x: 1, y: 1 }` then the rover moves to the new position given by
    /// `Vector { x: 2, y: 1 }`
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    N,
    E,
    S,
    W,
}

/// An `m * n` grid representing the area of operation for a [`Rover`]. For a `10 * 5` grid,
/// `Grid { m: 10, n: 5 }`, the northeast point of the grid is given by the [`Vector`],
/// `Vector { x: 10, y: 5 }` and the southwest point of the grid is `Vector { x: 0, y: 0 }`.
#[derive(Debug)]
pub struct Grid {
    m: i32,
    n: i32,
}

impl Rover {
    pub fn new(position: Vector, orientation: Orientation) -> Rover {
        Rover {
            position,
            orientation: orientation.to_unit_vector(),
            status: RoverStatus::Operational,
        }
    }

    pub fn follow_commands(&mut self, commands: &[Command], grid: &Grid) {
        use RoverStatus::*;

        for &command in commands {
            if self.status == Lost {
                return;
            }

            self.follow_command(command, grid);
        }
    }

    fn follow_command(&mut self, command: Command, grid: &Grid) {
        use Command::*;
        use RoverStatus::*;

        if self.status == Operational {
            match command {
                Left => {
                    self.orientation = self.orientation.rotate_left();
                }
                Right => {
                    self.orientation = self.orientation.rotate_right();
                }
                Forward => {
                    let new_pos = self.position + self.orientation;
                    if new_pos.x > grid.m || new_pos.x < 0 || new_pos.y > grid.n || new_pos.y < 0 {
                        self.status = Lost;
                    } else {
                        self.position = new_pos;
                    }
                }
            }
        }
    }
}

impl Orientation {
    fn to_unit_vector(self) -> Vector {
        use Orientation::*;

        match self {
            N => Vector::new(0, 1),
            E => Vector::new(1, 0),
            S => Vector::new(0, -1),
            W => Vector::new(-1, 0),
        }
    }

    fn from_unit_vector(v: Vector) -> Orientation {
        use Orientation::*;
        match (v.x, v.y) {
            (0, 1) => N,
            (1, 0) => E,
            (0, -1) => S,
            (-1, 0) => W,
            _ => panic!("Expected unit vector"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rover_follows_commands_operational() {
        use Command::*;

        let grid = Grid { m: 4, n: 8 };
        let mut rover = Rover::new(Vector::new(2, 3), Orientation::E);
        let commands = [Left, Forward, Right, Forward, Forward];

        rover.follow_commands(&commands, &grid);

        assert_eq!(RoverStatus::Operational, rover.status);
        assert_eq!(Vector::new(4, 4), rover.position);
        assert_eq!(
            Orientation::E,
            Orientation::from_unit_vector(rover.orientation)
        );
    }

    #[test]
    fn rover_follows_commands_lost() {
        use Command::*;

        let grid = Grid { m: 4, n: 8 };
        let mut rover = Rover::new(Vector::new(0, 2), Orientation::N);
        let commands = [Forward, Forward, Left, Forward, Right, Forward, Forward];

        rover.follow_commands(&commands, &grid);

        assert_eq!(RoverStatus::Lost, rover.status);
        assert_eq!(Vector::new(0, 4), rover.position);
        assert_eq!(
            Orientation::W,
            Orientation::from_unit_vector(rover.orientation)
        );
    }
}
