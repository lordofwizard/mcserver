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
        panic!("Project '{}' already exists!", dir_name);
    }

    // Attempt to create the directory
    fs::create_dir(path)?;

    println!("Project '{}' successfully created.", dir_name);
    Ok(())
}

fn make_project_tree(project_name: &str) {
    let project_path = Path::new(project_name);

    // Check if the project directory exists
    if !project_path.exists() {
        panic!("Project directory '{}' does not exist.", project_name);
    }

    // Create the subdirectories: {cache, java, log, server}
    let subdirectories = vec!["cache", "java", "log", "server"];
    for subdir in &subdirectories {
        let subdir_path = project_path.join(subdir);
        if !subdir_path.exists() {
            fs::create_dir(&subdir_path).expect("Failed to create subdirectory");
        }
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
    #[should_panic(expected = "Project 'existing_project' already exists!")]
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

#[cfg(test)]
mod tests_for_project_tree {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::env;

    fn setup_tmp_directory() -> std::io::Result<()> {
        env::set_current_dir("/tmp")
    }

    #[test]
    fn test_make_project_tree_success() {
        setup_tmp_directory().unwrap();
        let project_name = "test_project_tree";

        // Create the project directory before calling the function
        if !Path::new(project_name).exists() {
            fs::create_dir(project_name).unwrap();
        }

        // Call the function
        make_project_tree(project_name);

        // Verify that the subdirectories were created
        let expected_dirs = vec!["cache", "java", "log", "server"];
        for dir in &expected_dirs {
            assert!(Path::new(&format!("{}/{}", project_name, dir)).exists());
        }

        // Clean up by removing the created directories and project folder
        for dir in &expected_dirs {
            fs::remove_dir(format!("{}/{}", project_name, dir)).unwrap();
        }
        fs::remove_dir(project_name).unwrap();
    }

    #[test]
    #[should_panic(expected = "Project directory 'nonexistent_project' does not exist.")]
    fn test_make_project_tree_panic_on_nonexistent_project() {
        setup_tmp_directory().unwrap();
        let project_name = "nonexistent_project";

        // This should panic because the project directory does not exist
        make_project_tree(project_name);
    }

    #[test]
    fn test_make_project_tree_existing_structure() {
        setup_tmp_directory().unwrap();
        let project_name = "existing_project_tree";

        // Create the project directory and subdirectories
        if !Path::new(project_name).exists() {
            fs::create_dir(project_name).unwrap();
        }
        fs::create_dir(format!("{}/cache", project_name)).unwrap();
        fs::create_dir(format!("{}/java", project_name)).unwrap();

        // Call the function, should complete without errors
        make_project_tree(project_name);

        // Verify all subdirectories are now present
        let expected_dirs = vec!["cache", "java", "log", "server"];
        for dir in &expected_dirs {
            assert!(Path::new(&format!("{}/{}", project_name, dir)).exists());
        }

        // Clean up by removing the created directories and project folder
        for dir in &expected_dirs {
            fs::remove_dir(format!("{}/{}", project_name, dir)).unwrap();
        }
        fs::remove_dir(project_name).unwrap();
    }
}
