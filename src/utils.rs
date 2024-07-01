// Collection of commonly used function I am using for mcserver

// Downloading a java version

use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn make_file_tree(project_name: &str) {
    // This function takes a project_name i.e. Minecraft Server name and generates basic tree
    // structure if not present

    let base_path = Path::new(project_name);

    if !base_path.exists() {
        println!("Warning: The tree structure wasn't present. Generating a new project.");
        fs::create_dir(base_path).expect("Failed to create project directory");
    }

    let directories = vec!["cache", "java", "log", "server"];

    for dir in directories {
        let dir_path = base_path.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).expect("Failed to create directory");
        }
    }
}

#[allow(dead_code)]
pub fn server_present() -> bool {
    let path = Path::new("config.toml");
    path.exists()
}
