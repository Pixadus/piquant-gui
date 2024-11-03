// Description: functions to deal with file inputs and CLI arguments.

use clap::{Command, Arg, ArgMatches};

// Get all CLI arguments, if they exist. 
pub fn get_args() -> ArgMatches {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(Arg::new("INPUT")
                .help("Input image")
                .required(false)
                .index(1))
            .get_matches();
    matches
}