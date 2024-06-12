// Files which consists of all the functionalities required to do the java Downloads and stuff.
// This file will also have additional functionalities to extract and properly manage multiple
// versions of Java jdk from Adoptium

use reqwest::blocking::get;
use serde::Deserialize;
use std::process::Command;

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

pub fn extract_jdk(java_version : u8){
    
}
