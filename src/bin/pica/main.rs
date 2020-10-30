#[macro_use]
extern crate clap;

mod commands;
mod util;

use clap::{App, AppSettings};
use std::process;
use util::CliError;

fn main() {
    let m = App::new("pica")
        .about("Tools to work with bibliographic records encoded in Pica+")
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(commands::print::cli())
        .get_matches();

    let result = match m.subcommand_name() {
        Some("print") => {
            commands::print::run(m.subcommand_matches("print").unwrap())
        }
        _ => unreachable!(),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(CliError::Io(err)) => {
            eprintln!("IO Error: {}", err);
            process::exit(1);
        }
        Err(CliError::Other(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}