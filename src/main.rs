extern crate clap;

use clap::{App, Arg};

mod function_template;
mod feature;
mod format;
mod client_request;
mod file_controller;
mod configuration_file;

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

    feature::display_welcome_message();

    if matches.is_present("gc") {
        configuration_file::handle_configuration_file();
        feature::generate_component()
    } else if matches.is_present("init") {
        configuration_file::handle_configuration_file()
    }
}
