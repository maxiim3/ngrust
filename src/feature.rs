use crate::{client_request, file_controller};
use crate::configuration_file;
use crate::format;
use crate::function_template;

pub fn generate_component() {
    // set name
    let raw_component_name = client_request::ask_client("Enter the name of the Component: ");


    let options = vec!["use props", "use React State", "Is Client component"];
    let selection = client_request::multi_selection("Select options : ", options);
    println!("{:?}", selection);

    // options
    let with_props = client_request::is_affirmative("Do you want to use props ? (y/N)");
    let with_state = client_request::is_affirmative("Do you want to use React State ? (y/N)");

    // set template
    let component_name = format::component_name(&raw_component_name);
    let template = function_template::function_template(&component_name, with_props, with_state);

    println!("Generating the file for {}...", raw_component_name);

    // set file

    let file_name = format::file_name(&raw_component_name);
    let mut file = file_controller::create_file(file_name.as_str());

    // write file
    file_controller::write_file(&mut file, template);

    //////////////////// User feedback ///////////////////////

    println!(
        "Your component {} has successfully been created!",
        raw_component_name
    )
}

fn print_rust_ascii() {
    let ascii = "
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒░░░░░░░░░░░░░░░░░░░░░░░▓
░░░░░░░░░░░░░░░░░░░░░░░░░░▒░░▓▓▓▒▓▓▓▒▒▓▓░░▒▒░░░░░░░░░░░░░░▓
░░░░░░░░░░░░░░░░░░░░░░▒░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░▓
░░░░░░░░░░░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░░▓
░░░░░░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░▓
░░░░░░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░▓
░░░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░▓
░░░░░▒▓▓░░░░░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░▓
▒▓▓▓▒▒▓▓▓▒░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒██▓▓▓▓▓▓▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░▓
░▓▓▓▓▓▓▓▓▒░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓░░░██▓▓▓▓█░░▓██▓▓▓▓▓▓▓▓▓▓▓▓░░░▓
░░▒▓▓▓▓▓▓▓▒░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓██████▓▓▓▓██████▓▓▓▓▓▓▓▓▓▓▓▓▓▒░▓
░░░░░▒▒░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓████▓▓▓▓▓████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓
░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░▓
░░░░░░░░░░░░░▒▓▓▓▓▒▓▓░▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▓▓▓▒▒▓▓▓▓▓▓▒░▓
░░░░░░░░░░░░░░▒▓▓▓▒░▓░░░░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▒░▓▒░▒▓▓░░░▓
░░░░░░░░░░░░░░░░▒▓▒░░░░░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▒░░░░░░▓▓░░░░▓
░░░░░░░░░░░░░░░░░░▒░░░░░░░░░░░░░░░░░▒▒░░░▓▓▓▓░░░░░░░▒░░░░░▓
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓▓▓▒░░░░░░░░░░░░░░";
    println!("{}", ascii)
}


pub fn display_welcome_message() {
    print_rust_ascii();
    let message = configuration_file::get_welcome_message();
    println!("{}", message)
}
