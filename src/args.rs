use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "rover")]
pub struct Args {
    /// Path to file containing grid and rover information.
    pub file: PathBuf,
}

pub fn get_args() -> Args {
    Args::parse()
}
