use std::{
    io::{self, Write},
    panic,
};
pub fn setup() {
    // TODO Build this
}

fn take_project_name() -> String {
    print!("What do you wanna call the project? ");
    io::stdout().flush().unwrap();

    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read input");

    project_name_validator(&project_name)
}

fn project_name_validator(project_name: &str) -> String{

    if project_name.contains(" ") {
        panic!("Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")
    }

    if project_name.trim().is_empty() {
        panic!("Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")
    }

    // Trim whitespace from the input
    let project_name = project_name.trim();

    // Check if the input meets the criteria
    if !project_name.is_empty() && project_name.chars().all(|c| c.is_alphabetic() || c == '_') { 
        return project_name.to_string();
    } else {
        panic!("Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // Valid Cases
    #[test]
    fn valid_project_name_with_alphabets() {
        let result = project_name_validator("project");
        assert_eq!(result, "project".to_string());
    }

    #[test]
    fn valid_project_name_with_underscore() {
        let result = project_name_validator("my_project");
        assert_eq!(result, "my_project".to_string());
    }

    #[test]
    fn valid_project_name_with_only_underscores() {
        let result = project_name_validator("____");
        assert_eq!(result, "____".to_string());
    }


    // Invalid Cases: Expected to Panic
    #[test]
    #[should_panic]
    fn invalid_project_name_with_trailing_spaces() {
        let result = project_name_validator("  myproject  ");
    }

    #[test]
    #[should_panic]
    fn project_name_with_space_should_panic() {
        project_name_validator("my project");
    }

    #[test]
    #[should_panic(expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")]
    fn project_name_with_numbers_should_panic() {
        project_name_validator("project123");
    }

    #[test]
    #[should_panic(expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")]
    fn project_name_with_special_chars_should_panic() {
        project_name_validator("project!@#");
    }

    #[test]
    #[should_panic(expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")]
    fn empty_project_name_should_panic() {
        project_name_validator("");
    }

    #[test]
    #[should_panic(expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace.")]
    fn project_name_with_only_spaces_should_panic() {
        project_name_validator("   ");
    }
}