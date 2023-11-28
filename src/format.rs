use regex::Regex;

fn capitalize_first_letter(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut chars = input.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}

pub fn file_name(message: &str) -> String {
    message.trim().replace(" ", "-").to_lowercase()
}

pub fn component_name(message: &str) -> String {
    let rx = Regex::new(r"[ \-_]+").unwrap();

    let split = rx.replace(message.trim(), ",");
    let word_list: Vec<&str> = split.split(",").collect();

    let capitalized_list: Vec<String> = word_list
        .iter()
        .map(|item| capitalize_first_letter(item))
        .collect();

    capitalized_list.join("")
}
