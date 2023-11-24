pub fn function_template(component_name: &str, with_props: bool, with_state: bool) -> String {
    let prop_declaration = "
    type Props = any
    ";
    let prop_parameter = "props:Props";
    let state_import = "import React, {useState} from \"react\";";
    let state_declaration = "const [myState, setMyState] = useState<any>(null);";
    format!(
        "
    {3}
    {1}
    function {0}({2}){{

        {4}

            return (
                <>
                 Here is my component {0}
                </>
            )
        }}
     ",
        component_name, // 0
        if with_props { prop_declaration } else { "" }, //1
        if with_props { prop_parameter } else { "" }, //2
        if with_state { state_import } else { "" }, //3
        if with_state { state_declaration } else { "" } //4
    )
}
