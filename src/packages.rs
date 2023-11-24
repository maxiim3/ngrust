use std::fs::File;
use std::io::Write;
// ADD THIS
use crate::function_template::function_template;
use crate::utils;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input
}

fn get_yes_no_input(prompt: &str) -> bool {
    let user_input = get_input(prompt);
    matches!(user_input.to_lowercase().trim(), "y" | "yes")
}

pub fn generate_component() {
    let raw_component_name = get_input("Enter the name of the Component: ");
    let component_name = utils::to_pascal_case(&raw_component_name);
    let file_name = utils::to_kebab_case(&raw_component_name);


    let with_props = get_yes_no_input("Do you want to use props ? (y/N)");
    let with_state = get_yes_no_input("Do you want to use React State ? (y/N)");

    println!("Generating the file for {}...", raw_component_name);

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
        raw_component_name
    )
}

pub fn display_welcome_message() {
    println!("Welcome th ngrust");
    println!("Usage: ngrust --gc <COMPONENT_NAME> to generate a new component");
}
