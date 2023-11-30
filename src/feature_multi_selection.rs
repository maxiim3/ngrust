use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;

#[derive(Clone, Debug)]
pub struct SelectableOption {
    name: String,
    id: String,
    is_selected: bool,
}

pub trait SelectableOptionController {
    fn new(name: String) -> SelectableOption;
    fn set_active(&mut self);
    fn set_inactive(&mut self);
    fn get_id(&self) -> String;
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

    fn get_id(&self) -> String {
        self.id.clone()
    }
}


pub struct MultiSelection {
    options: Vec<SelectableOption>,
}

pub trait MultiSelectionController {
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
