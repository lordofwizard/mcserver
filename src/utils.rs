// Collection of commonly used function I am using for mcserver

// Downloading a java version

pub fn download_jdk(java_version: u8) {
    use reqwest::blocking::get;
    use serde::Deserialize;
    use std::process::Command;

    #[derive(Deserialize)]
    struct AvailableReleases {
        available_releases: Vec<u8>,
    }

    let response = match get("https://api.adoptium.net/v3/info/available_releases") {
        Ok(res) => res,
        Err(_) => {
            println!("Failed to make the API request, to Adoptium");
            std::process::exit(1);
        }
    };

    let releases: AvailableReleases = match response.json() {
        Ok(json) => json,
        Err(_) => {
            println!("Failed to parse the JSON response, from Adoptium");
            std::process::exit(1);
        }
    };

    if !releases.available_releases.contains(&java_version) {
        println!(
            "Given Java Version : {} is not available by the sources. ",
            java_version
        );
        std::process::exit(0);
    }

    let url = format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/linux/x64/jdk/hotspot/normal/eclipse",
        java_version
    );

    let output_file = format!("jdk-{}.tar.gz", java_version);
    let status = Command::new("curl")
        .arg("-L")
        .arg("--progress-bar")
        .arg(&url)
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("failed to execute curl command");

    if status.success() {
        println!("File downloaded successfully as {}", output_file);
    } else {
        println!("Failed to download the file.");
        std::process::exit(1);
    }
}

pub fn make_file_tree(project_name: &str) {
    // This function takes a project_name i.e. Minecraft Server name and generates basic tree
    // structure if not present
    use std::fs;
    use std::path::Path;

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
