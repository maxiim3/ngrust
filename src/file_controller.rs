use ::std::io::Write;
use std::fs::File;

use crate::configuration_file;

pub fn write_file(file: &mut File, template: String) {
    write!(
        file,
        "{}", template
    )
        .expect("Failed to write file");
}

pub fn create_file(file_name: &str) -> File {
    let path = configuration_file::get_path();
    let typescript_file = format!("{0}{1}.tsx", path, file_name);
    File::create(typescript_file).expect("Failed to create file")
}
