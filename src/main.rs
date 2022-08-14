use std::process;

mod args;
mod parser;
mod rover;
mod vector;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = args::get_args();

    let input = std::fs::read_to_string(&args.file)?;

    let (grid, lines) = match parser::parse_input(&input) {
        Ok(input) => input,
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            process::exit(1);
        }
    };

    for (mut rover, commands) in lines {
        rover.follow_commands(&commands, &grid);
        println!("{}", rover);
    }

    Ok(())
}
