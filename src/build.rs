use std::path::Path;
pub fn build() {
    // TODO Build this
}

pub fn check_project_structure() -> bool {
    let required_file = "config.toml";
    let required_dirs = vec!["server", "cache", "log", "java"];

    if !Path::new(required_file).exists() {
        return false;
    }

    for dir in required_dirs {
        if !Path::new(dir).is_dir() {
            return false;
        }
    }

    // All checks passed
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_project_structure_test() {
        std::env::set_current_dir("/tmp").expect("Failed to set current directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure1")
            .output()
            .expect("Failed to create test project directory");
        std::process::Command::new("touch")
            .arg("test_project_for_tree_structure1/config.toml")
            .output()
            .expect("Failed to create config.toml file");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure1/server")
            .output()
            .expect("Failed to create server directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure1/cache")
            .output()
            .expect("Failed to create cache directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure1/log")
            .output()
            .expect("Failed to create log directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure1/java")
            .output()
            .expect("Failed to create java directory");

        std::env::set_current_dir("/tmp/test_project_for_tree_structure1").expect("Failed to set current directory to test_project_for_tree_structure1");
        std::env::set_current_dir("/tmp").expect("Failed to set current directory");
        assert_eq!(check_project_structure(), true);
        std::process::Command::new("rm")
            .arg("-rf")
            .arg("/tmp/test_project_for_tree_structure1")
            .output()
            .expect("Failed to remove test project directory");
    }
    #[test]
    fn check_project_structure_test_empty() {
        std::env::set_current_dir("/tmp").expect("Failed to set current directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure2")
            .output()
            .expect("Failed to create test project directory");

        std::env::set_current_dir("/tmp/test_project_for_tree_structure2").expect("Failed to set current directory to test_project_for_tree_structure2");
        assert_eq!(check_project_structure(), false);
        std::process::Command::new("rm")
            .arg("-rf")
            .arg("/tmp/test_project_for_tree_structure2")
            .output()
            .expect("Failed to remove test project directory");
    }
    #[test]
    fn check_project_structure_test_except_config() {
        std::env::set_current_dir("/tmp").expect("Failed to set current directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure3")
            .output()
            .expect("Failed to create test project directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure3/server")
            .output()
            .expect("Failed to create server directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure3/cache")
            .output()
            .expect("Failed to create cache directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure3/log")
            .output()
            .expect("Failed to create log directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure3/java")
            .output()
            .expect("Failed to create java directory");

        std::env::set_current_dir("/tmp/test_project_for_tree_structure3").expect("Failed to set current directory to test_project_for_tree_structure3");
        assert_eq!(check_project_structure(), false);
        std::process::Command::new("rm")
            .arg("-rf")
            .arg("/tmp/test_project_for_tree_structure3")
            .output()
            .expect("Failed to remove test project directory");
    }
    #[test]
    fn check_project_structure_test_few_folders() {
        std::env::set_current_dir("/tmp").expect("Failed to set current directory");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure4")
            .output()
            .expect("Failed to create test project directory");
        std::process::Command::new("touch")
            .arg("test_project_for_tree_structure4/config.toml")
            .output()
            .expect("Failed to create config.toml file");
        std::process::Command::new("mkdir")
            .arg("test_project_for_tree_structure4/log")
            .output()
            .expect("Failed to create log directory");

        std::env::set_current_dir("/tmp/test_project_for_tree_structure4").expect("Failed to set current directory to test_project_for_tree_structure4");
        assert_eq!(check_project_structure(), false);
        std::process::Command::new("rm")
            .arg("-rf")
            .arg("/tmp/test_project_for_tree_structure4")
            .output()
            .expect("Failed to remove test project directory");
    }
}
