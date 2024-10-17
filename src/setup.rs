use std::{
    fs,
    io::{self, Write},
    panic,
    path::Path,
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

fn project_name_validator(project_name: &str) -> String {
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

fn make_project_directory(dir_name: &str) -> io::Result<()> {
    let path = Path::new(dir_name);

    // Check if the directory already exists
    if path.exists() {
        panic!("Directory '{}' already exists!", dir_name);
    }

    // Attempt to create the directory
    fs::create_dir(path)?;

    println!("Directory '{}' successfully created.", dir_name);
    Ok(())
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
        let _ = project_name_validator("  myproject  ");
    }

    #[test]
    #[should_panic]
    fn project_name_with_space_should_panic() {
        project_name_validator("my project");
    }

    #[test]
    #[should_panic(
        expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace."
    )]
    fn project_name_with_numbers_should_panic() {
        project_name_validator("project123");
    }
    #[test]
    #[should_panic(
        expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace."
    )]
    fn project_name_with_random_symbols_should_panic() {
        project_name_validator("project@#!%!@_ ");
    }

    #[test]
    #[should_panic(
        expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace."
    )]
    fn project_name_with_special_chars_should_panic() {
        project_name_validator("project!@#");
    }

    #[test]
    #[should_panic(
        expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace."
    )]
    fn empty_project_name_should_panic() {
        project_name_validator("");
    }

    #[test]
    #[should_panic(
        expected = "Invalid project name. Please enter only alphabetic characters or underscores, no numbers or whitespace."
    )]
    fn project_name_with_only_spaces_should_panic() {
        project_name_validator("   ");
    }
}

#[cfg(test)]
mod tests_for_directory_of_project {
    use super::*;
    use std::env;
    use std::fs;
    use std::io;
    use std::path::Path;

    fn setup_tmp_directory() -> io::Result<()> {
        // Change the current working directory to /tmp
        env::set_current_dir("/tmp")
    }

    #[test]
    fn test_create_new_directory() {
        setup_tmp_directory().unwrap();
        let dir_name = "test_project";

        // Ensure the directory does not exist before the test
        if Path::new(dir_name).exists() {
            fs::remove_dir(dir_name).unwrap();
        }

        // Try creating the directory
        assert!(make_project_directory(dir_name).is_ok());

        // Check if the directory was actually created
        assert!(Path::new(dir_name).exists());

        // Clean up by removing the directory after the test
        fs::remove_dir(dir_name).unwrap();
    }

    #[test]
    #[should_panic(expected = "Directory 'existing_project' already exists!")]
    fn test_panic_if_directory_exists() {
        setup_tmp_directory().unwrap();
        let dir_name = "existing_project";

        // Ensure the directory exists before the test
        if !Path::new(dir_name).exists() {
            fs::create_dir(dir_name).unwrap();
        }

        // This should panic because the directory already exists
        make_project_directory(dir_name).unwrap();

        // Clean up by removing the directory after the test
        fs::remove_dir(dir_name).unwrap();
    }

    #[test]
    fn test_directory_is_cleaned_up() {
        setup_tmp_directory().unwrap();
        let dir_name = "temporary_project";

        // Ensure directory is removed before the test if it exists
        if Path::new(dir_name).exists() {
            fs::remove_dir(dir_name).unwrap();
        }

        // Create the directory
        assert!(make_project_directory(dir_name).is_ok());

        // Verify that the directory was created
        assert!(Path::new(dir_name).exists());

        // Clean up by removing the directory after the test
        fs::remove_dir(dir_name).unwrap();

        // Verify that the directory was successfully removed
        assert!(!Path::new(dir_name).exists());
    }
}
