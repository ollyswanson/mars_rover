# Rover

A simple command-line program to solve the "Mars Rover" challenge.

## Installation

To build & run locally first make sure you have installed the Rust toolchain by
following the documentation found
[here](https://www.rust-lang.org/tools/install).  
The minimum supported rust version (msrv) to compile the program is 1.58.1

## Running

This project uses the command-line runner [just](https://github.com/casey/just)
to provide some simple recipes for running the project/tests and opening the
documentation. To run without installing just, the recipes can be copied from
the (Justfile)[Justfile]. With just installed, running `just --list` or simply
`just` will print the available recipes to stdout.

Alternatively the program can be run with a debug build using `cargo run [PATH]`
where `PATH` is a path to a file containing the input. Leaving `PATH` blank
will read from stdin instead. `cargo run -- -h` will display a message with the
options available.

```
rover

USAGE:
    rover [PATH]

ARGS:
    <PATH>    Path to file containing grid and rover information. Leave blank to read from stdin

OPTIONS:
    -h, --help    Print help information
```
