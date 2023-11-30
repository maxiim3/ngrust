use crate::component::{Component, ComponentController};
use crate::feature_multi_selection::{MultiSelection, MultiSelectionController, SelectableOption, SelectableOptionController};
use crate::file_controller;
use crate::format;

pub const OPTIONS: [&str; 3] = ["Use props", "Use React State", "Is Client component"];

pub struct Program {
    component_options: Vec<SelectableOption>,
    raw_name: String,
}

pub trait ProgramController {
    fn new() -> Program;
    fn set_component_name(&mut self);
    fn set_component_options(&mut self);
    fn generate_component(&self) -> Component;
    fn generate_file(&self, component: Component);
    fn run(&mut self);
}

impl ProgramController for Program {
    fn new() -> Self {
        Program {
            component_options: Vec::new(),
            raw_name: String::new(),
        }
    }

    fn set_component_name(&mut self) {
        println!("{}", "Enter the name of the Component: ");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        self.raw_name = user_input.trim().to_string();
    }

    fn set_component_options(&mut self) {
        let mut multi_selection = MultiSelection::new(
            OPTIONS
                .iter()
                .map(|option| SelectableOption::new(String::from(*option)))
                .collect()
        );
        multi_selection.prompt_user();
        self.component_options = multi_selection.get_selection();
    }

    fn generate_component(&self) -> Component {
        Component::new(&self.raw_name, self.component_options.clone())
    }

    fn generate_file(&self, component: Component) {
        let file_name = format::file_name(&self.raw_name);
        let template = component.generate_template();

        let mut file = file_controller::create_file(file_name.as_str());
        file_controller::write_file(&mut file, template);
    }

    fn run(&mut self) {
        self.set_component_name();
        self.set_component_options();

        let component = self.generate_component();

        println!("Generating the file for {}...", self.raw_name);

        self.generate_file(component);
        println!(
            "Your component {} has successfully been created!",
            self.raw_name
        )
    }
}
