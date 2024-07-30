// Files which consists of all the functionalities required to do the java Downloads and stuff.
// This file will also have additional functionalities to extract and properly manage multiple
// versions of Java jdk from Adoptium

use colored::Colorize;
use reqwest::blocking::get;
use serde::Deserialize;
use std::{fs, io::{self, ErrorKind}, path::Path, process::Command};

pub fn download_jdk(java_version: u8) {
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

    let output_file = format!("./cache/jdk-{}.tar.gz", java_version);

    if file_exists(&output_file) {
        println!("File already present bro");
    } else {
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
        extract_jdk(&output_file);

        match move_files_from_random_dir() {
            Ok(_) => println!("{}","Files moved successfully. to java directory".green()),
            Err(e) => eprintln!("Error occurred: {}", e),
        }
    }
}

pub fn latest_java_version() -> String {
    let url = "https://api.adoptium.net/v3/info/available_releases";

    #[derive(Deserialize)]
    struct ApiResponse {
        most_recent_feature_release: Option<u32>,
    }

    let response = match get(url) {
        Ok(resp) => match resp.json::<ApiResponse>() {
            Ok(json) => json,
            Err(_) => return "unknown".to_string(),
        },
        Err(_) => return "unknown".to_string(),
    };

    if let Some(release) = response.most_recent_feature_release {
        release.to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn file_exists(file_path: &str) -> bool {
    let full_path = std::env::current_dir().unwrap().join(file_path);
    Path::new(&full_path).exists()
}

pub fn extract_jdk(file_path: &str) {
    if file_exists(file_path) {
        let status = Command::new("tar")
            .arg("-xvf")
            .arg(file_path)
            .arg("-C")
            .arg(format!("./java/"))
            .status()
            .expect("failed to execute curl command");

        if status.success() {
            println!("Java Successfully Extracted {}", file_path);
        } else {
            println!("Failed to extract the file.");
            std::process::exit(1);
        }
    }
}

pub fn move_files_from_random_dir() -> io::Result<()> {
    // Get the current working directory
    let cwd = std::env::current_dir()?;
    // Define the path to the "java" directory
    let java_dir = cwd.join("java");

    // Check if the "java" directory exists
    if !java_dir.is_dir() {
        return Err(io::Error::new(ErrorKind::NotFound, "java directory not found"));
    }

    // Read the contents of the "java" directory
    let entries: Vec<_> = fs::read_dir(&java_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    // There should be exactly one directory inside "java"
    let mut random_dir = None;
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if random_dir.is_some() {
                return Err(io::Error::new(ErrorKind::InvalidData, "More than one directory found"));
            }
            random_dir = Some(path);
        }
    }

    let random_dir = match random_dir {
        Some(dir) => dir,
        None => return Err(io::Error::new(ErrorKind::NotFound, "No directory found inside java")),
    };

    // Move all files and directories from the random directory to "java"
    for entry in fs::read_dir(&random_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = java_dir.join(src_path.file_name().unwrap());

        if file_type.is_dir() {
            fs::rename(&src_path, &dst_path)?;
        } else {
            fs::rename(&src_path, &dst_path)?;
        }
    }

    // Remove the now-empty random directory
    fs::remove_dir(&random_dir)?;

    Ok(())
}
