extern crate clap;

use clap::{App, Arg};

mod utils;
mod function_template;
mod packages;

fn main() {
    let matches = App::new("ngrust")
        .version("0.0.1")
        .author("Maxiim3")
        .about("Generate Component using a Ng Cli clone")
        .arg(
            Arg::with_name("gc")
                .long("gc")
                .help("Generates a new component"),
        )
        .get_matches();

    if matches.is_present("gc") {
        packages::generate_component()
    } else {
        packages::display_welcome_message()
    }
}
