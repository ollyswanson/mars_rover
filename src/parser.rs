use chumsky::prelude::*;

use crate::rover::{Command, Grid, Orientation, Rover};
use crate::vector::Vector;

/// Parses input of the form `"4 8\n(2, 3, E) LFRFF\n(0, 2, N) FFLFRFF\n"`
pub fn input_parser() -> impl Parser<char, (Grid, Vec<(Rover, Vec<Command>)>), Error = Simple<char>>
{
    let grid = grid_parser();

    let rover = rover_parser();

    let commands = commands_parser();

    grid.then_ignore(text::newline()).then(
        rover
            .padded()
            .then(commands.padded())
            .repeated()
            .collect::<Vec<_>>(),
    )
}

fn num_parser() -> impl Parser<char, i32, Error = Simple<char>> + Copy {
    text::int::<_, Simple<char>>(10).try_map(|n, span| {
        n.parse::<i32>()
            .map_err(|e| Simple::custom(span, format!("{}", e)))
    })
}

// parse [`Grid`] from input of the form `"4 8"`
fn grid_parser() -> impl Parser<char, Grid, Error = Simple<char>> {
    let num = num_parser();

    num.then_ignore(just(' '))
        .then(num)
        .map(|(m, n)| Grid { m, n })
}

// parse [`Rover`] from input of the form `"(2, 3, E)"`
fn rover_parser() -> impl Parser<char, Rover, Error = Simple<char>> {
    let num = num_parser();

    let orientation = choice((
        just('N').to(Orientation::N),
        just('E').to(Orientation::E),
        just('S').to(Orientation::S),
        just('W').to(Orientation::W),
    ));

    num.then_ignore(just(','))
        .then(num.padded())
        .then_ignore(just(','))
        .then(orientation.padded())
        .delimited_by(just('('), just(')'))
        .map(|((x, y), orientation)| Rover::new(Vector::new(x, y), orientation))
}

// parse commands of the form `"LFRFF"`
fn commands_parser() -> impl Parser<char, Vec<Command>, Error = Simple<char>> {
    choice((
        just('L').to(Command::Left),
        just('R').to(Command::Right),
        just('F').to(Command::Forward),
    ))
    .repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid() {
        let input = "4 8";
        let expected = Grid { m: 4, n: 8 };

        assert_eq!(expected, grid_parser().parse(input).unwrap());
    }

    #[test]
    fn parse_rover() {
        let input = "(2, 3, E)";
        let expected = Rover::new(Vector::new(2, 3), Orientation::E);

        assert_eq!(expected, rover_parser().parse(input).unwrap());
    }

    #[test]
    fn parse_commands() {
        use Command::*;

        let input = "LFRFF";
        let expected = vec![Left, Forward, Right, Forward, Forward];

        assert_eq!(expected, commands_parser().parse(input).unwrap());
    }

    #[test]
    fn parse_input() {
        use Command::*;

        let input = "\
            4 8
            (2, 3, E) LFRFF
            (0, 2, N) FFLFRFF";

        let expected = (
            Grid { m: 4, n: 8 },
            vec![
                (
                    Rover::new(Vector::new(2, 3), Orientation::E),
                    vec![Left, Forward, Right, Forward, Forward],
                ),
                (
                    Rover::new(Vector::new(0, 2), Orientation::N),
                    vec![Forward, Forward, Left, Forward, Right, Forward, Forward],
                ),
            ],
        );

        assert_eq!(expected, input_parser().parse(input).unwrap());
    }
}
