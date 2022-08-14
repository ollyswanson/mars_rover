use std::fs;
use std::io::{self, Read};
use std::process;

mod args;
mod parser;
mod rover;
mod vector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::get_args();

    let input = if let Some(path) = args.path {
        fs::read_to_string(path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    let (grid, lines) = match parser::parse_input(&input) {
        Ok(input) => input,
        Err(errors) => {
            // TODO: Improve error reporting using `ariadne` crate
            for error in errors {
                eprintln!("{error}");
            }
            process::exit(1);
        }
    };

    for (mut rover, commands) in lines {
        rover.follow_commands(&commands, &grid);
        println!("{rover}");
    }

    Ok(())
}
