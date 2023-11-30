use dialoguer::{MultiSelect, theme::ColorfulTheme};

use crate::{client_request, file_controller, function_template};
use crate::configuration_file;
use crate::format;

#[derive(Clone, Debug)]
struct SelectableOption {
    name: String,
    id: String,
    is_selected: bool,
}

trait SelectableOptionController {
    fn new(name: String) -> SelectableOption;
    fn set_active(&mut self);
    fn set_inactive(&mut self);
}

impl SelectableOptionController for SelectableOption {
    fn new(name: String) -> Self {
        SelectableOption {
            is_selected: false,
            id: String::from(&name.to_lowercase().trim().replace(" ", "-")),
            name,
        }
    }
    fn set_active(&mut self) {
        self.is_selected = true;
    }

    fn set_inactive(&mut self) {
        self.is_selected = false;
    }
}


pub struct MultiSelection {
    options: Vec<SelectableOption>,
}

trait MultiSelectionController {
    fn new(options: Vec<SelectableOption>) -> MultiSelection;
    fn list_to_string(&self) -> Vec<&str>;
    fn prompt_user(&mut self);
    fn get_selection(&self) -> Vec<SelectableOption>;
}

impl MultiSelectionController for MultiSelection {
    fn new(options: Vec<SelectableOption>) -> Self {
        MultiSelection {
            options
        }
    }
    fn list_to_string(&self) -> Vec<&str> {
        let mut string: Vec<&str> = Vec::new();
        for option in self.options.iter() {
            string.push(&option.name);
        }
        return string;
    }

    fn prompt_user(&mut self) {
        println!("{}", "Select options : ");


        MultiSelect::with_theme(&ColorfulTheme::default())
            .items(&self.list_to_string())
            .interact()
            .unwrap()
            .iter()
            .for_each(|index| {
                self.options[*index].set_active();
            })
    }
    fn get_selection(&self) -> Vec<SelectableOption> {
        // let mut selected_options: Vec<SelectableOption> = Vec::new();
        // for option in self.options.iter() {
        //     if option.is_selected {
        //         selected_options.push(option.clone());
        //     }
        // }
        // selected_options
        let selection = self.options
            .iter()
            .filter(|selectable_option| selectable_option.is_selected)
            .map(|selected_option| (*selected_option).clone())
            .collect();
        selection
    }
}


struct Component {
    name: String,
    options: Vec<SelectableOption>,
}

trait ComponentController {
    fn new(name: String, options: Vec<SelectableOption>) -> Component;
    fn generate_template(&self);
}

impl ComponentController for Component {
    fn new(name: String, options: Vec<SelectableOption>) -> Self {
        let name = format::component_name(&name);
        Component {
            name,
            options,
        }
    }
    fn generate_template(&self) -> String {
        format!(
            "
    { if with_state { "import React, {useState} from \"react\";" } else { "" }}
    { if with_props { " type Props = any " } else { "" }}
    function {component_name}({if with_props { "props:Props" } else { "" }}){{

        {if with_state { "const [myState, setMyState] = useState<any>(null);" } else { "" }}

            return (
                <>
                 Here is my component {component_name}
                </>
            )
        }}
     ",
            component_name, // 0
            if with_props { " type Props = any " } else { "" }, //1
            if with_props { "props:Props" } else { "" }, //2
            if with_state { "import React, {useState} from \"react\";" } else { "" }, //3
            if with_state { "const [myState, setMyState] = useState<any>(null);" } else { "" } //4
        )
    }
}

const OPTIONS: [&str; 3] = ["use props", "use React State", "Is Client component"];


pub fn generate_component() {
    // set name
    let raw_component_name = client_request::ask_client("Enter the name of the Component: ");
    let mut multi_selection = MultiSelection::new(
        OPTIONS
            .iter()
            .map(|option| SelectableOption::new(String::from(*option)))
            .collect()
    );
    multi_selection.prompt_user();
    let selection = multi_selection.get_selection();
    println!("{:?}", selection);


    // let options = vec!["use props", "use React State", "Is Client component"];
    // let selection = client_request::multi_selection("Select options : ", options);
    // println!("{:?}", selection);


    // options
    // let with_props = client_request::is_affirmative("Do you want to use props ? (y/N)");
    // let with_state = client_request::is_affirmative("Do you want to use React State ? (y/N)");

    // set template
    // let component_name = format::component_name(&raw_component_name);
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
