use crate::feature_multi_selection::{SelectableOption, SelectableOptionController};
use crate::format;

pub struct Component {
    name: String,
    options: Vec<SelectableOption>,
}

pub trait ComponentController {
    fn new(name: &str, options: Vec<SelectableOption>) -> Component;
    fn generate_template(&self) -> String;
}

impl ComponentController for Component {
    fn new(name: &str, options: Vec<SelectableOption>) -> Self {
        let name = format::component_name(name);
        Component { name, options }
    }
    fn generate_template(&self) -> String {
        let mut state_import = "";
        let mut use_state = "";

        let mut prop_type = "";
        let mut prop_parameter = "";

        let mut client_component = "";

        for option in self.options.iter() {
            match option.get_id().as_str() {
                "use-props" => {
                    prop_type = " type Props = any\n";
                    prop_parameter = "props:Props";
                }
                "use-react-state" => {
                    state_import = "import React, {useState} from \"react\";\n";
                    use_state = "const [myState, setMyState] = useState<any>(null);\n";
                }
                "is-client-component" => client_component = "\"use client\";\n",
                _ => {}
            }
        }

        return format!(
            "
    {0}{1}{2}
    function {3} ({4}) {{\n
           {5}
            return (
                <>
                 Here is my component {3}
                </>
            )
        }}
     ",
            client_component, state_import, prop_type, &self.name, prop_parameter, use_state
        )
            .to_string();
    }
}
