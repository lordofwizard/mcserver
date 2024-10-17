use reqwest::blocking::get;
use serde::Deserialize;
#[derive(Deserialize)]
struct ApiResponse {
    status: String,
    response: Option<ResponseDetails>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ResponseDetails {
    version: String,
    file: String,
    size: Size,
    md5: String,
    built: u64,
    stability: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Size {
    display: String,
    bytes: u64,
}

pub fn fetch_latest_minecraft_version() -> String {
    let url = "https://centrojars.com/api/fetchJar/vanilla/vanilla/";

    let response = match get(url) {
        Ok(resp) => match resp.json::<ApiResponse>() {
            Ok(json) => json,
            Err(_) => return "unknown".to_string(),
        },
        Err(_) => return "unknown".to_string(),
    };

    if response.status == "success" {
        if let Some(details) = response.response {
            return details.version;
        }
    }

    "unknown".to_string()
}
#[cfg(test)]
mod tests_for_fetch_version {
    use super::*;

    #[test]
    fn test_fetch_version_failed() {
        let version = fetch_latest_minecraft_version();
        assert_ne!(version, "unknown");
    }
    #[test]
    fn test_latest_java_version() {
        let version = fetch_latest_minecraft_version();
        assert_eq!(version, "1.21");
    }
}
