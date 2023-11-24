use std::fs::File;
use std::io::Write;
// ADD THIS
use crate::function_template::function_template;
use crate::utils;

pub fn generate_component() {
    println!("Enter the name of the Component: ");


    let mut user_input = String::new();
    ///////////////////// Component Name //////////////////////////

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to erad line");

    let component_name = utils::to_pascal_case(&user_input);
    let file_name = utils::to_kebab_case(&user_input);

    /////////////////////// Option 1 - props //////////////////////////

    user_input.clear();
    let mut with_props: bool = false;

    println!("Do you want to use props ? (y/N)");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to erad line");

    match user_input.to_lowercase().trim() {
        "y" | "yes" => { with_props = true; }
        _ => { with_props = false }
    }

    ///////////////////// Option 2  - state ////////////////////

    user_input.clear();
    let mut with_state: bool = false;

    println!("Do you want to use React State ? (y/N)");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to erad line");

    match user_input.to_lowercase().trim() {
        "y" | "yes" => { with_state = true; }
        _ => { with_state = false }
    }

    //////////////// Generate the File ///////////////////////

    println!("Generating the file for {}...", user_input);

    let typescript_file = format!("{}.tsx", file_name);
    let mut file = File::create(typescript_file).expect("Failed to create file");

    let template = function_template(&component_name, with_props, with_state);

    write!(
        file,
        "{}", template
    )
        .expect("Failed to write file");

    //////////////////// User feedback ///////////////////////

    println!(
        "Your component {} has successfully been created!",
        user_input
    )
}

pub fn display_welcome_message() {
    println!("Welcome th ngrust");
    println!("Usage: ngrust --gc <COMPONENT_NAME> to generate a new component");
}
