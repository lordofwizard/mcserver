use reqwest::blocking::get;
use serde::Deserialize;

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latest_java_version() {
        let version = latest_java_version();
        assert_eq!(version, "23");
    }
    
    #[test]
    fn test_latest_java_version_failed() {
        let version = latest_java_version();
        assert_ne!(version, "unknown");
    }
    #[test]
    fn test_version_is_int() {
        let version = latest_java_version();
        let version = version.parse::<u32>();
        assert!(version.is_ok());
    }
}
