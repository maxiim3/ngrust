extern crate clap;

use clap::{App, Arg};

use welcome_message::display_welcome_message;

use crate::configuration_file::handle_configuration_file;
use crate::main_program::{Program, ProgramController};

mod main_program;
mod format;
mod file_controller;
mod configuration_file;
mod feature_multi_selection;
mod component;
mod welcome_message;

fn main() {
    let matches = App::new("ngrust")
        .version("0.0.1")
        .author("Maxiim3")
        .about("Generate Component using a Ng Cli clone")
        .arg(
            Arg::with_name("gc")
                .long("gc")
                .help("Generates a new component")
        ).arg(
        Arg::with_name("init")
            .long("init")
            .help("Initialize configuration")
    )
        .get_matches();


    if matches.is_present("gc") {
        handle_configuration_file();
    display_welcome_message();
        let mut program = Program::new();
        program.run();
    } else if matches.is_present("init") {
        handle_configuration_file();
    }
}
