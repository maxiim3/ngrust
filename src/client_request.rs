use dialoguer::{MultiSelect, theme::ColorfulTheme};

pub fn ask_client(prompt: &str) -> String {
    println!("{}", prompt);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    return user_input;
}

pub fn is_affirmative(prompt: &str) -> bool {
    let user_input = ask_client(prompt);
    matches!(user_input.to_lowercase().trim(), "y" | "yes")
}

pub fn multi_selection<'a>(prompt: &str, options: Vec<&'a str>) -> Vec<&'a str> {
    println!("{}", prompt);

    let selection_index = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .interact()
        .unwrap();

    let mut selection: Vec<&'a str> = Vec::new();
    for index in selection_index.iter() {
        selection.push(options[*index])
    };

    return selection;
}
